use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Witness {
  #[clap(help = "The inscription script is the second data on witness array")]
  pub(crate) inscription_script: String,
  #[clap(long, help = "Display only the inscription's body")]
  pub(crate) only_body: bool,
  #[clap(long, help = "Display only the inscription's content type")]
  pub(crate) only_content_type: bool,
  #[clap(long, help = "[To be implemented]")]
  pub(crate) only_parent: bool,
}

impl Witness {
  pub(crate) fn run(self, _options: Options) -> Result {
    let data = self.inscription_script.as_str();
    if data.len() % 2 == 1 { return Err(anyhow!("Odd length hex string")) };

    let result;

    let witness = bitcoin::Witness::from_vec(vec![vec![], hex::decode(data).unwrap(), vec![]]);
    if let Some(inscription) = Inscription::from_witness(&witness){
        let body = inscription.body().unwrap_or(&[]);
        let content_type = inscription.content_type().unwrap_or("");

      if self.only_body {
        result = format!("{:?}", body);
      } else if self.only_content_type {
        result = format!("{}", content_type);
      } else if self.only_parent {
        return Err(anyhow!("--only-parent not implemented yet"));
      } else {
        result = format!("{{\"body\": {:?}, \"content_type\": \"{}\"}}", body, content_type);
      }
    } else {
        let data_shortened = data[..(data.len().min(12))].to_string() + if data.len() > 12 { "..." } else {""};
        return Err(anyhow!("Failed to parse witness data {}", data_shortened));
    }
    
    println!("{}", result);
    Ok(())
  }
}