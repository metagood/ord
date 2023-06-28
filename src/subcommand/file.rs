use super::*;

#[derive(Debug, Parser)]
pub(crate) struct File {
  #[clap(long)]
  pub(crate) tx: Txid,
  #[clap()]
  pub(crate) filename: String,
}

impl File {
  pub(crate) fn run(&self, options: Options) -> Result {
    let client = options.bitcoin_rpc_client_for_wallet_command(false)?;

    let tx = client.get_raw_transaction(&self.tx, None)?;
    let inscription = Inscription::from_transaction(&tx).unwrap();

    let content_bytes = inscription.body().unwrap();
    let mut file = fs::File::create(self.filename.clone())?;
    file.write_all(content_bytes)?;

    Ok(())
  }
}
