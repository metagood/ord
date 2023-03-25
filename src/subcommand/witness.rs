use super::*;

pub(crate) mod ctype;
pub(crate) mod file;

#[derive(Debug, Parser)]
pub(crate) enum Witness {
  #[clap(about = "Display the content type from an inscription")]
  Ctype(ctype::Ctype),
  #[clap(about = "Write the inscription content to a file")]
  File(file::File)
}

impl Witness {
  pub(crate) fn run(self, options: Options) -> Result {
    match self {
      Self::Ctype(ctype) => ctype.run(options),
      Self::File(file) => file.run(options),
    }
  }
}