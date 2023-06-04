use serde_json::Value;
use super::*;

#[derive(Debug, Clone)]
pub(super) struct Flotsam {
  inscription_id: InscriptionId,
  offset: u64,
  origin: Origin,
  // populated if new inscription, None if transfer of existing inscription
  inscription_data: Option<Inscription>,
}

#[derive(Debug, Clone)]
enum Origin {
  New {
    fee: u64,
    cursed: bool,
    unbound: bool,
  },
  Old {
    old_satpoint: SatPoint,
  },
}

pub(super) struct InscriptionUpdater<'a, 'db, 'tx> {
  flotsam: Vec<Flotsam>,
  height: u64,
  id_to_satpoint: &'a mut Table<'db, 'tx, &'static InscriptionIdValue, &'static SatPointValue>,
  value_receiver: &'a mut Receiver<u64>,
  id_to_entry: &'a mut Table<'db, 'tx, &'static InscriptionIdValue, InscriptionEntryValue>,
  pub(super) lost_sats: u64,
  next_cursed_number: i64,
  next_number: i64,
  number_to_id: &'a mut Table<'db, 'tx, i64, &'static InscriptionIdValue>,
  outpoint_to_value: &'a mut Table<'db, 'tx, &'static OutPointValue, u64>,
  reward: u64,
  sat_to_inscription_id: &'a mut Table<'db, 'tx, u64, &'static InscriptionIdValue>,
  satpoint_to_id: &'a mut Table<'db, 'tx, &'static SatPointValue, &'static InscriptionIdValue>,
  timestamp: u32,
  pub(super) unbound_inscriptions: u64,
  value_cache: &'a mut HashMap<OutPoint, u64>,
}

impl<'a, 'db, 'tx> InscriptionUpdater<'a, 'db, 'tx> {
  pub(super) fn new(
    height: u64,
    id_to_satpoint: &'a mut Table<'db, 'tx, &'static InscriptionIdValue, &'static SatPointValue>,
    value_receiver: &'a mut Receiver<u64>,
    id_to_entry: &'a mut Table<'db, 'tx, &'static InscriptionIdValue, InscriptionEntryValue>,
    lost_sats: u64,
    number_to_id: &'a mut Table<'db, 'tx, i64, &'static InscriptionIdValue>,
    outpoint_to_value: &'a mut Table<'db, 'tx, &'static OutPointValue, u64>,
    sat_to_inscription_id: &'a mut Table<'db, 'tx, u64, &'static InscriptionIdValue>,
    satpoint_to_id: &'a mut Table<'db, 'tx, &'static SatPointValue, &'static InscriptionIdValue>,
    timestamp: u32,
    unbound_inscriptions: u64,
    value_cache: &'a mut HashMap<OutPoint, u64>,
  ) -> Result<Self> {
    let next_cursed_number = number_to_id
      .iter()?
      .map(|(number, _id)| number.value() - 1)
      .next()
      .unwrap_or(-1);

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
      next_cursed_number,
      next_number,
      number_to_id,
      outpoint_to_value,
      reward: Height(height).subsidy(),
      sat_to_inscription_id,
      satpoint_to_id,
      timestamp,
      unbound_inscriptions,
      value_cache,
    })
  }

  pub(super) fn index_transaction_inscriptions(
    &mut self,
    tx: &Transaction,
    txid: Txid,
    input_sat_ranges: Option<&VecDeque<(u64, u64)>>,
  ) -> Result {
    let mut new_inscriptions = Inscription::from_transaction(tx).into_iter().peekable();
    let mut floating_inscriptions = Vec::new();
    let mut inscribed_offsets = BTreeMap::new();
    let mut input_value = 0;
    let mut id_counter = 0;

    for (input_index, tx_in) in tx.input.iter().enumerate() {
      // skip subsidy since no inscriptions possible
      if tx_in.previous_output.is_null() {
        input_value += Height(self.height).subsidy();
        continue;
      }

      // find existing inscriptions on input aka transfers of inscriptions
      for (old_satpoint, inscription_id) in
        Index::inscriptions_on_output(self.satpoint_to_id, tx_in.previous_output)?
      {
        let offset = input_value + old_satpoint.offset;
        floating_inscriptions.push(Flotsam {
          offset,
          inscription_id,
          origin: Origin::Old { old_satpoint },
          inscription_data: None,
        });

        inscribed_offsets.insert(offset, inscription_id);
      }

      let offset = input_value;

      // multi-level cache for UTXO set to get to the input amount
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
      };

      // go through all inscriptions in this input
      while let Some(inscription) = new_inscriptions.peek() {
        if inscription.tx_in_index != u32::try_from(input_index).unwrap() {
          break;
        }

        let initial_inscription_is_cursed = inscribed_offsets
          .get(&offset)
          .and_then(
            |inscription_id| match self.id_to_entry.get(&inscription_id.store()) {
              Ok(option) => option.map(|entry| InscriptionEntry::load(entry.value()).number < 0),
              Err(_) => None,
            },
          )
          .unwrap_or(false);

        let cursed = !initial_inscription_is_cursed
          && (inscription.tx_in_index != 0
            || inscription.tx_in_offset != 0
            || inscribed_offsets.contains_key(&offset));

        // In this first part of the cursed inscriptions implementation we ignore reinscriptions.
        // This will change once we implement reinscriptions.
        let unbound = inscribed_offsets.contains_key(&offset)
          || inscription.tx_in_offset != 0
          || input_value == 0;

        let inscription_id = InscriptionId {
          txid,
          index: id_counter,
        };

        floating_inscriptions.push(Flotsam {
          inscription_id,
          offset,
          origin: Origin::New {
            fee: 0,
            cursed,
            unbound,
          },
          inscription_data: Some(inscription.inscription.clone()),
        });

        new_inscriptions.next();
        id_counter += 1;
      }
    }

    // still have to normalize over inscription size
    let total_output_value = tx.output.iter().map(|txout| txout.value).sum::<u64>();
    let mut floating_inscriptions = floating_inscriptions
      .into_iter()
      .map(|flotsam| {
        if let Flotsam {
          inscription_id,
          offset,
          origin:
            Origin::New {
              fee: _,
              cursed,
              unbound,
            },
            inscription_data,
        } = flotsam
        {
          Flotsam {
            inscription_id,
            offset,
            origin: Origin::New {
              fee: (input_value - total_output_value) / u64::from(id_counter),
              cursed,
              unbound,
            },
            inscription_data,
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
          inscriptions.next().unwrap(),
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
      self.lost_sats += self.reward - output_value;
      Ok(())
    } else {
      self.flotsam.extend(inscriptions.map(|flotsam| Flotsam {
        offset: self.reward + flotsam.offset - output_value,
        ..flotsam
      }));
      self.reward += input_value - output_value;
      Ok(())
    }
  }

  fn update_inscription_location(
    &mut self,
    input_sat_ranges: Option<&VecDeque<(u64, u64)>>,
    flotsam: Flotsam,
    new_satpoint: SatPoint,
  ) -> Result {
    let inscription_id = flotsam.inscription_id.store();
    let mut new_inscription_number: i64 = 0;
    let unbound = match flotsam.origin {
      Origin::Old { old_satpoint } => {
        self.satpoint_to_id.remove(&old_satpoint.store())?;

        false
      }
      Origin::New {
        fee,
        cursed,
        unbound,
      } => {
        let number = if cursed {
          let next_cursed_number = self.next_cursed_number;
          self.next_cursed_number -= 1;

          next_cursed_number
        } else {
          let next_number = self.next_number;
          self.next_number += 1;

          next_number
        };

        self.number_to_id.insert(number, &inscription_id)?;
        new_inscription_number = number;

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

        self.id_to_entry.insert(
          &inscription_id,
          &InscriptionEntry {
            fee,
            height: self.height,
            number,
            sat,
            timestamp: self.timestamp,
          }
          .store(),
        )?;

        unbound
      }
    };

    let satpoint = if unbound {
      let new_unbound_satpoint = SatPoint {
        outpoint: unbound_outpoint(),
        offset: self.unbound_inscriptions,
      };
      self.unbound_inscriptions += 1;
      new_unbound_satpoint.store()
    } else {
      new_satpoint.store()
    };

    self.satpoint_to_id.insert(&satpoint, &inscription_id)?;
    self.id_to_satpoint.insert(&inscription_id, &satpoint)?;

    let inscription_id = InscriptionId::load(inscription_id);
    let satpoint = SatPoint::load(satpoint);
    // let inscription_entry = self.id_to_entry.get(&inscription_id.store())?.unwrap();
    // let inscription_number = InscriptionEntry::load(inscription_entry.value()).number;
    let inscription_number = if new_inscription_number != 0 {
      new_inscription_number
    }  else {
      let inscription_entry = self.id_to_entry.get(&inscription_id.store())?.unwrap();
      InscriptionEntry::load(inscription_entry.value()).number
    };

    if let Some(inscription) = flotsam.inscription_data {
      let is_brc_20 = Self::is_brc_20(&self, &inscription);
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
    } else {
      log::info!(
        target: "new_inscription_satpoint",
        "{},{},{},{}",
        self.height,
        satpoint,
        inscription_id,
        inscription_number,
      );
    }

    Ok(())
  }

  fn valid_json(data: Option<&[u8]>) -> bool {
    match data {
      Some(bytes) => serde_json::from_slice::<Value>(bytes).is_ok(),
      None => false,
    }
  }

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
