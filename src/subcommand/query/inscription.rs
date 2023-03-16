use super::*;

pub(crate) fn run(_options: Options) -> Result {
  println!("[subcommand::query::inscription]: Not implemented yet.");
  /*
  let index = Index::open(&options)?;
  index.update()?;


  let inscription_id = index.get_inscription_id_by_inscription_number(666)?.unwrap();
  let entry = index.get_inscription_entry(inscription_id)?.unwrap();
  let inscription = index.get_inscription_by_id(inscription_id)?.unwrap();
  let satpoint = index.get_inscription_satpoint_by_id(inscription_id)?.unwrap();

  let output = index
    .get_transaction(satpoint.outpoint.txid)?.unwrap()
    .output
    .into_iter()
    .nth(satpoint.outpoint.vout.try_into().unwrap()).unwrap();

  let previous = if let Some(previous) = entry.number.checked_sub(1) {
    Some(
    index
      .get_inscription_id_by_inscription_number(previous)?.unwrap()
    )
  } else {
    None
  };

  let next = index.get_inscription_id_by_inscription_number(entry.number + 1)?.unwrap();

  print_json(inscription_id);
  print_json(satpoint);
  print_json(output);
  print_json(previous);
  print_json(next);
  */
  
  Ok(())
}