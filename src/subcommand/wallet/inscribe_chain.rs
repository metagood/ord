use std::fs::{DirBuilder, DirEntry};

use super::{inscribe::Inscribe, *};

#[derive(Debug, Parser)]
pub(crate) struct InscribeChain {
  #[clap(long, help = "Inscribe <SATPOINT>")]
  pub(crate) satpoint: SatPoint,
  #[clap(long, help = "Use fee rate of <FEE_RATE> sats/vB")]
  pub(crate) fee_rate: FeeRate,
  #[clap(
    long,
    help = "Use <COMMIT_FEE_RATE> sats/vbyte for commit transaction.\nDefaults to <FEE_RATE> if unset."
  )]
  pub(crate) commit_fee_rate: Option<FeeRate>,
  #[clap(help = "Inscribe sat with contents of <FILE>")]
  pub(crate) dir: PathBuf,
  #[clap(long, help = "Do not back up recovery key.")]
  pub(crate) no_backup: bool,
  #[clap(
    long,
    help = "Do not check that transactions are equal to or below the MAX_STANDARD_TX_WEIGHT of 400,000 weight units. Transactions over this limit are currently nonstandard and will not be relayed by bitcoind in its default configuration. Do not use this flag unless you understand the implications."
  )]
  pub(crate) no_limit: bool,
  #[clap(long, help = "Don't sign or broadcast transactions.")]
  pub(crate) destination: Option<Address>,
  #[clap(long, help = "Establish parent relationship with <PARENT>.")]
  pub(crate) parent: Option<InscriptionId>,
}

// maximum of 12
const INSCRIPTION_PER_BLOCK: usize = 10;

impl InscribeChain {
  pub(crate) fn run(self, options: Options) -> Result {
    let mut satpoint = self.satpoint;

    let dir = self.dir.read_dir()?;
    let mut files: Vec<DirEntry> = dir
      .map(|f| f.unwrap())
      .filter(|file| file.path().is_file())
      .collect();
    files.sort_by(|a, b| get_number_from_dir_entry(a).cmp(&get_number_from_dir_entry(b)));

    if files.len() as u64 > satpoint.offset + 1 {
      return Err(anyhow!(
        "Not enough sats: folder has {} files and output offset is {}",
        files.len(),
        satpoint.offset
      ));
    }

    DirBuilder::new()
      .create(&self.dir.join("inscribed"))
      .unwrap_or_default();

    for i in 0..(files.len().min(INSCRIPTION_PER_BLOCK)) {
      let file = files.get(i).unwrap();
      let file_path = file.path();

      let inscribe = Inscribe {
        dry_run: false,
        fee_rate: self.fee_rate,
        commit_fee_rate: self.commit_fee_rate,
        destination: self.destination.clone(),
        file: file_path.clone(),
        no_backup: self.no_backup,
        no_limit: self.no_limit,
        satpoint: Some(satpoint),
        parent: self.parent.clone(),
      };

      println!("Inscribing {} at {}", file_path.clone().display(), satpoint);
      let inscription = inscribe.run(options.clone())?;

      fs::rename(
        file_path.clone(),
        &self
          .dir
          .join("inscribed")
          .join(file_path.file_name().unwrap()),
      )?;

      // update satpoint to inscribe for next iteration
      if satpoint.offset >= 1 {
        satpoint.offset -= 1;

        satpoint = SatPoint {
          outpoint: OutPoint {
            txid: inscription.commit,
            vout: 0,
          },
          offset: satpoint.offset,
        };
      };
    }

    println!("\nSuccess!");
    println!(
      "{} new inscriptions pending in the mempool.",
      files.len().min(INSCRIPTION_PER_BLOCK)
    );
    println!("\nTo continue inscribing, wait for the block to be mined and run:");
    println!("{}", self.get_resume_cli_command(satpoint));

    Ok(())
  }

  fn get_resume_cli_command(&self, updated_satpoint: SatPoint) -> String {
    let mut cli = format!(
      "ord wallet inscribe-chain --fee-rate {}",
      self.fee_rate.fee(10 as usize).to_sat() as f64 / 10.0
    );
    if let Some(parent) = self.parent {
      cli.push_str(&format!(" --parent {}", parent));
    }
    if let Some(destination) = &self.destination {
      cli.push_str(&format!(" --destination {}", destination));
    }
    cli.push_str(&format!(" --satpoint {}", updated_satpoint));
    cli.push_str(&format!(" {}", self.dir.display()));

    return cli;
  }
}

fn get_number_from_dir_entry(dir_entry: &DirEntry) -> u64 {
  dir_entry
    .path()
    .file_stem()
    .unwrap()
    .to_str()
    .unwrap()
    .parse()
    .context("Name the files in the format: <number>.<extension>, e.g. 1.json")
    .unwrap()
}
