use super::*;

pub(crate) mod inscription;
pub(crate) mod sat;
pub(crate) mod block;

#[derive(Debug, Parser)]
pub(crate) enum Query {
  #[clap(about = "Display information about an inscription")]
  Inscription,
  #[clap(about = "Display information about a satoshi")]
  Sat,
  #[clap(about = "Display information about a block")]
  Block(block::Block),
}

impl Query {
  pub(crate) fn run(self, options: Options) -> Result {
    match self {
      Self::Inscription => inscription::run(options),
      Self::Sat => sat::run(options),
      Self::Block(block) => block.run(options),
    }
  }
}