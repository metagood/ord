use std::io::Write;

use super::*;

#[derive(Debug, Parser)]
pub(crate) struct File {
  #[clap(help = "The inscription to extract the content.")]
  inscription_id: String,
  #[clap(help = "The name of the file to be created with the inscription content.")]
  filename: String,
}

impl File {
  pub(crate) fn run(self, options: Options) -> Result {
    let inscription_id = InscriptionId::from_str(self.inscription_id.as_str())?;

    let index = Index::open(&options)?;
    index.update()?;

    if let Some(inscription) = index.get_inscription_by_id(inscription_id)?{
      let content_bytes = inscription.body().unwrap();
      let mut file = fs::File::create(self.filename)?;
      file.write_all(content_bytes)?;

    } else {
      return Err(anyhow!("Inscription {} not found", inscription_id));
    }

    Ok(())
  }
}
