use super::*;

#[derive(Debug, Parser)]
pub(crate) struct File {
  #[clap(long)]
  pub(crate) inscription: InscriptionId,
  #[clap()]
  pub(crate) filename: String,
}

impl File {
  pub(crate) fn run(&self, options: Options) -> SubcommandResult {
    let client = options.bitcoin_rpc_client()?;
    // TODO: not sure which one we need here
    // let client = options.bitcoin_rpc_client_for_wallet_command(false)?;

    let tx = client.get_raw_transaction(&self.inscription.txid, None)?;
    let inscription = &Inscription::from_transaction(&tx)[self.inscription.index as usize];

    let content_bytes = inscription.inscription.body().unwrap();
    let mut file = fs::File::create(self.filename.clone())?;
    file.write_all(content_bytes)?;

    Ok(Box::new(()))
  }
}
