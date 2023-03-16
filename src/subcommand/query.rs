use super::*;

#[derive(Debug, Parser)]
pub(crate) enum Query {
  InscriptionId,
  InscriptionNumber,
  Sat,
  Block,
}

impl Query {
  pub(crate) fn run(self, options: Options) -> Result {
    match self {
        Self::InscriptionId => println!("Query by inscription id"),
        Self::InscriptionNumber => println!("Query by inscription number"),
        Self::Sat => println!("Query by sat number"),
        Self::Block => println!("Query by block number"),
    }

    Ok(())
  }
}