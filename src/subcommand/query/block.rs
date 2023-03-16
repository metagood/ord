use std::ptr::null;

use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Block {
  #[clap(help = "Block number")]
  pub(crate) height: u64,
}

impl Block {
  pub(crate) fn run(self, options: Options) -> Result {
    let index = Index::open(&options)?;
    index.update()?;

    if let Some(block) = index.get_block_by_height(self.height)?{
        print_json(block)?;
    } else {
        print_json({})?;
    }

    Ok(())
  }
}