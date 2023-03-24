use super::*;

struct SnapshotData {
  inscription_number: u64,
  inscription_id: InscriptionId,
  transaction_id: Txid,
  satoshi_location: SatPoint,
}

pub(crate) fn run(options: Options) -> Result {
  let index = Index::open(&options)?;
  index.update()?;

  let inscriptions = index.get_inscriptions(None)?;
  let inscription_numbers = index.get_inscription_ids_by_number()?;

  let mut snapshot_rows: Vec<SnapshotData> = vec![];

  for (satpoint, id) in inscriptions {
    let row = SnapshotData {
      inscription_number: inscription_numbers[&id],
      inscription_id: id,
      transaction_id: id.txid,
      satoshi_location: satpoint,
    };

    snapshot_rows.push(row);
  }

  snapshot_rows.sort_by(|a, b| a.inscription_number.cmp(&b.inscription_number));

  
  println!("inscription_number, inscription_id, transaction_id, satoshi_location");
  for row in snapshot_rows {
    println!("{}, {}, {}, {}", row.inscription_number, row.inscription_id, row.transaction_id, row.satoshi_location);
  }

  Ok(())
}
