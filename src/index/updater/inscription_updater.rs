use serde_json::Value;
use {super::*, std::collections::BTreeSet};

#[derive(Clone, Debug)]
pub(super) struct Flotsam {
  inscription_id: InscriptionId,
  offset: u64,
  origin: Origin,
  tx: Option<Transaction>,
}

// change name to Jetsam or more poetic german word
#[derive(Clone, Copy, Debug)]
enum Origin {
  New((u64, Option<InscriptionId>)),
  Old(SatPoint),
}

pub(super) struct InscriptionUpdater<'a, 'db, 'tx> {
  flotsam: Vec<Flotsam>,
  height: u64,
  id_to_satpoint: &'a mut Table<'db, 'tx, &'static InscriptionIdValue, &'static SatPointValue>,
  value_receiver: &'a mut Receiver<u64>,
  id_to_entry: &'a mut Table<'db, 'tx, &'static InscriptionIdValue, InscriptionEntryValue>,
  lost_sats: u64,
  next_number: u64,
  number_to_id: &'a mut Table<'db, 'tx, u64, &'static InscriptionIdValue>,
  outpoint_to_value: &'a mut Table<'db, 'tx, &'static OutPointValue, u64>,
  reward: u64,
  sat_to_inscription_id: &'a mut Table<'db, 'tx, u64, &'static InscriptionIdValue>,
  satpoint_to_id: &'a mut Table<'db, 'tx, &'static SatPointValue, &'static InscriptionIdValue>,
  timestamp: u32,
  value_cache: &'a mut HashMap<OutPoint, u64>,
  cached_children_by_id: &'a Mutex<HashMap<InscriptionId, Vec<InscriptionId>>>,
  // client: &'a Client,
}

impl<'a, 'db, 'tx> InscriptionUpdater<'a, 'db, 'tx> {
  pub(super) fn new(
    height: u64,
    id_to_satpoint: &'a mut Table<'db, 'tx, &'static InscriptionIdValue, &'static SatPointValue>,
    value_receiver: &'a mut Receiver<u64>,
    id_to_entry: &'a mut Table<'db, 'tx, &'static InscriptionIdValue, InscriptionEntryValue>,
    lost_sats: u64,
    number_to_id: &'a mut Table<'db, 'tx, u64, &'static InscriptionIdValue>,
    outpoint_to_value: &'a mut Table<'db, 'tx, &'static OutPointValue, u64>,
    sat_to_inscription_id: &'a mut Table<'db, 'tx, u64, &'static InscriptionIdValue>,
    satpoint_to_id: &'a mut Table<'db, 'tx, &'static SatPointValue, &'static InscriptionIdValue>,
    timestamp: u32,
    value_cache: &'a mut HashMap<OutPoint, u64>,
    cached_children_by_id: &'a Mutex<HashMap<InscriptionId, Vec<InscriptionId>>>,
    // client: &'a Client,
  ) -> Result<Self> {
    let next_number = number_to_id
      .iter()?
      .rev()
      .map(|(number, _id)| number.value() + 1)
      .next()
      .unwrap_or(0);

    Ok(Self {
      flotsam: Vec::new(),
      height,
      id_to_satpoint,
      value_receiver,
      id_to_entry,
      lost_sats,
      next_number,
      number_to_id,
      outpoint_to_value,
      reward: Height(height).subsidy(),
      sat_to_inscription_id,
      satpoint_to_id,
      timestamp,
      value_cache,
      cached_children_by_id,
      // client,
    })
  }

  pub(super) fn index_transaction_inscriptions(
    &mut self,
    tx: &Transaction,
    txid: Txid,
    input_sat_ranges: Option<&VecDeque<(u64, u64)>>,
  ) -> Result<u64> {
    let mut floating_inscriptions = Vec::new();
    let mut inscribed_offsets = BTreeSet::new();
    let mut input_value = 0;
    for tx_in in &tx.input {
      // skip subsidy since no inscriptions possible
      if tx_in.previous_output.is_null() {
        input_value += Height(self.height).subsidy();
        continue;
      }

      // find existing inscriptions on input aka transfers
      for (old_satpoint, inscription_id) in
        Index::inscriptions_on_output(self.satpoint_to_id, tx_in.previous_output)?
      {
        floating_inscriptions.push(Flotsam {
          offset: input_value + old_satpoint.offset,
          inscription_id,
          origin: Origin::Old(old_satpoint),
          tx: None,
        });

        inscribed_offsets.insert(input_value + old_satpoint.offset);
      }

      // find new inscriptions
      if let Some(inscription) = Inscription::from_tx_input(tx_in) {
        // ignore new inscriptions on already inscribed offset (sats)
        if !inscribed_offsets.contains(&input_value) {
          let inscription_id = InscriptionId {
            txid,
            index: 0, // will have to be updated for multi inscriptions
          };

          // parent has to be in an input before child
          // think about specifying a more general approach in a protocol doc/BIP
          if let Some(parent_candidate) = inscription.get_parent_id() {
            log::debug!(
              "INDEX: inscription {} has inscribed parent {}",
              inscription_id,
              parent_candidate
            );
          }

          let parent = inscription.get_parent_id().filter(|&parent_id| {
            floating_inscriptions
              .iter()
              .any(|flotsam| flotsam.inscription_id == parent_id)
          });

          if let Some(parent) = parent {
            log::debug!(
              "INDEX: inscription {} has confirmed parent {}",
              inscription_id,
              parent
            );
          }

          floating_inscriptions.push(Flotsam {
            inscription_id,
            offset: input_value,
            origin: Origin::New((0, parent)),
            tx: Some(tx.clone()),
          });
        }
      }

      // different ways to get the utxo set (input amount)
      input_value += if let Some(value) = self.value_cache.remove(&tx_in.previous_output) {
        value
      } else if let Some(value) = self
        .outpoint_to_value
        .remove(&tx_in.previous_output.store())?
      {
        value.value()
      } else {
        self.value_receiver.blocking_recv().ok_or_else(|| {
          anyhow!(
            "failed to get transaction for {}",
            tx_in.previous_output.txid
          )
        })?
      }
    }

    // calulate genesis fee for new inscriptions
    let total_output_value = tx.output.iter().map(|txout| txout.value).sum::<u64>();
    let mut floating_inscriptions = floating_inscriptions
      .into_iter()
      .map(|flotsam| {
        if let Flotsam {
          inscription_id,
          offset,
          origin: Origin::New((_, parent)),
          tx,
        } = flotsam
        {
          Flotsam {
            inscription_id,
            offset,
            origin: Origin::New((input_value - total_output_value, parent)),
            tx,
          }
        } else {
          flotsam
        }
      })
      .collect::<Vec<Flotsam>>();

    let is_coinbase = tx
      .input
      .first()
      .map(|tx_in| tx_in.previous_output.is_null())
      .unwrap_or_default();

    if is_coinbase {
      floating_inscriptions.append(&mut self.flotsam);
    }

    floating_inscriptions.sort_by_key(|flotsam| flotsam.offset);
    let mut inscriptions = floating_inscriptions.into_iter().peekable();

    let mut output_value = 0;
    for (vout, tx_out) in tx.output.iter().enumerate() {
      let end = output_value + tx_out.value;

      while let Some(flotsam) = inscriptions.peek() {
        if flotsam.offset >= end {
          break;
        }

        let new_satpoint = SatPoint {
          outpoint: OutPoint {
            txid,
            vout: vout.try_into().unwrap(),
          },
          offset: flotsam.offset - output_value,
        };

        self.update_inscription_location(
          input_sat_ranges,
          inscriptions.next().unwrap(), // This will need to change when we implement multiple inscriptions per TX (#1298).
          new_satpoint,
        )?;
      }

      output_value = end;

      self.value_cache.insert(
        OutPoint {
          vout: vout.try_into().unwrap(),
          txid,
        },
        tx_out.value,
      );
    }

    if is_coinbase {
      for flotsam in inscriptions {
        let new_satpoint = SatPoint {
          outpoint: OutPoint::null(),
          offset: self.lost_sats + flotsam.offset - output_value,
        };
        self.update_inscription_location(input_sat_ranges, flotsam, new_satpoint)?;
      }

      Ok(self.reward - output_value)
    } else {
      self.flotsam.extend(inscriptions.map(|flotsam| Flotsam {
        offset: self.reward + flotsam.offset,
        ..flotsam
      }));
      self.reward += input_value - output_value;
      Ok(0)
    }
  }

  fn update_inscription_location(
    &mut self,
    input_sat_ranges: Option<&VecDeque<(u64, u64)>>,
    flotsam: Flotsam,
    new_satpoint: SatPoint,
  ) -> Result {
    let inscription_id = flotsam.inscription_id.store();

    match flotsam.origin {
      Origin::Old(old_satpoint) => {
        self.satpoint_to_id.remove(&old_satpoint.store())?;
      }
      Origin::New((fee, parent)) => {
        self
          .number_to_id
          .insert(&self.next_number, &inscription_id)?;

        let mut sat = None;
        if let Some(input_sat_ranges) = input_sat_ranges {
          let mut offset = 0;
          for (start, end) in input_sat_ranges {
            let size = end - start;
            if offset + size > flotsam.offset {
              let n = start + flotsam.offset - offset;
              self.sat_to_inscription_id.insert(&n, &inscription_id)?;
              sat = Some(Sat(n));
              break;
            }
            offset += size;
          }
        }

        log::debug!(
          "INDEX: assigned {} for inscription {} at height {}",
          &self.next_number,
          flotsam.inscription_id,
          self.height
        );

        self.id_to_entry.insert(
          &inscription_id,
          &InscriptionEntry {
            fee,
            height: self.height,
            number: self.next_number,
            parent,
            sat,
            timestamp: self.timestamp,
          }
          .store(),
        )?;

        if let Some(parent) = parent {
          self.update_cached_children(parent, flotsam.inscription_id);
        }

        self.next_number += 1;
      }
    }

    let new_satpoint = new_satpoint.store();

    self.satpoint_to_id.insert(&new_satpoint, &inscription_id)?;
    self.id_to_satpoint.insert(&inscription_id, &new_satpoint)?;

    let inscription_id = InscriptionId::load(inscription_id);
    let satpoint = SatPoint::load(new_satpoint);
    let inscription_entry = self.id_to_entry.get(&inscription_id.store())?.unwrap();
    let inscription_number = InscriptionEntry::load(inscription_entry.value()).number;

    if let Some(tx) = flotsam.tx {
      let inscription = Inscription::from_transaction(&tx).unwrap();
      let is_brc_20 = Self::is_brc_20(&self, &inscription);

      // if !Self::is_brc_20(&self, &inscription) {
      let content_type = inscription.content_type().unwrap_or("");
      let content_len = inscription.body().map_or(0, |body| body.len());

      log::info!(
        target: "new_inscription_satpoint",
        "{},{},{},{},{},{},{}",
        self.height,
        satpoint,
        inscription_id,
        inscription_number,
        content_type,
        content_len,
        is_brc_20,
      );
      // }
    } else {
      // let inscription = self
      //   .get_transaction(inscription_id.txid)?
      //   .and_then(|tx| Inscription::from_transaction(&tx)).unwrap();

      // if !Self::is_brc_20(&self, &inscription) {
      log::info!(
        target: "new_inscription_satpoint",
        "{},{},{},{}",
        self.height,
        satpoint,
        inscription_id,
        inscription_number,
      );
      // }
    }

    Ok(())
  }

  fn update_cached_children(&self, parent: InscriptionId, inscription_id: InscriptionId) {
    let mut cache = self.cached_children_by_id.lock().unwrap();

    // only update the cache if it is already populated, so we retrieve the full list of children when required
    if let Some(children) = cache.get_mut(&parent) {
      children.push(inscription_id);
    }
  }

  fn valid_json(data: Option<&[u8]>) -> bool {
    match data {
      Some(bytes) => serde_json::from_slice::<Value>(bytes).is_ok(),
      None => false,
    }
  }

  // fn get_transaction(&self, txid: Txid) -> Result<Option<Transaction>> {
  //   self.client.get_raw_transaction(&txid, None).into_option()
  // }

  fn is_brc_20(&self, inscription: &Inscription) -> bool {
    let valid_json = Self::valid_json(inscription.body());
    if valid_json {
      let json_result: Result<Value, serde_json::Error> =
        serde_json::from_slice(&inscription.body().unwrap());
      let json: Value = json_result.unwrap();
      let empty_json = serde_json::Map::new();
      let json_obj = json.as_object().unwrap_or(&empty_json);
      if json_obj.contains_key("p") {
        let p = json_obj.get("p").unwrap();
        if p.is_string() {
          let p_str = p.as_str().unwrap();
          if p_str.to_lowercase() == "brc-20" {
            return true;
          }
        }
      }
    }
    false
  }
}
