use super::*;

pub(crate) fn run(options: Options) -> Result {
  println!("Updating ord index...");
  let index = Index::open(&options)?;
  index.update()?;
  
  println!("Taking snapshot of database...");
  println!("number - id");
  let numbers_and_ids = index.get_inscription_number_and_ids()?;
  for (number, id) in numbers_and_ids {
    println!("{number} - {id}");
  }

  println!("satoshi - id");
  let sats_and_ids = index.get_inscription_sats_and_ids()?;
  for (sat, id) in sats_and_ids {
    println!("{sat} - {id}");
  }

  Ok(())
}