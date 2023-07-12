use super::*;

#[derive(Debug, Parser)]
pub(crate) struct File {
  #[clap(long)]
  pub(crate) inscription: InscriptionId,
  #[clap()]
  pub(crate) filename: String,
}

impl File {
  pub(crate) fn run(&self, options: Options) -> Result {
    let client = options.bitcoin_rpc_client()?;

    let tx = client.get_raw_transaction(&self.inscription.txid, None)?;
    let inscription = &Inscription::from_transaction(&tx)[self.inscription.index as usize];

    let content_bytes = inscription.inscription.body().unwrap();
    let mut file = fs::File::create(self.filename.clone())?;
    file.write_all(content_bytes)?;

    Ok(())
  }
}
