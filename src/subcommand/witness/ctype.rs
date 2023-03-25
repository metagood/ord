use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Ctype {
  #[clap(help = "The inscription to extract the content type.")]
  inscription_id: String,
}

impl Ctype {
  pub(crate) fn run(self, options: Options) -> Result {
    let inscription_id = InscriptionId::from_str(self.inscription_id.as_str())?;

    let index = Index::open(&options)?;
    index.update()?;

    if let Some(inscription) = index.get_inscription_by_id(inscription_id)? {
      println!("{}", inscription.content_type().unwrap());
    } else {
      return Err(anyhow!("Inscription {} not found", inscription_id));
    }

    Ok(())
  }
}
