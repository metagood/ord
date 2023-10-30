use super::*;

#[derive(Debug, Parser)]
pub(crate) struct File {
  #[clap(long)]
  pub(crate) inscription: InscriptionId,
  #[clap()]
  pub(crate) filename: PathBuf,
}

impl File {
  pub(crate) fn run(&self, options: Options) -> SubcommandResult {
    let client = options.bitcoin_rpc_client()?;
    let tx = client.get_raw_transaction(&self.inscription.txid, None)?;
    let inscriptions = ParsedEnvelope::from_transaction(&tx);

    let mut filename = self.filename.clone();
    let mut file_number = 2;

    for inscription in inscriptions {
      let content_bytes = inscription.payload.body().unwrap();
      let mut file = fs::File::create(self.filename.clone())?;
      file.write_all(content_bytes)?;

      filename.set_file_name(format!(
        "{}-{}{}",
        file_number,
        filename.file_stem().unwrap().to_str().unwrap(),
        filename.extension().unwrap().to_str().unwrap()
      ));
      file_number += 1;
    }

    Ok(Box::new(()))
  }
}
