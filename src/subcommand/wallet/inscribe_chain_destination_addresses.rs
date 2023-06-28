use std::fs::{DirBuilder, DirEntry};

use super::{inscribe::Inscribe, *};

#[derive(Debug, Parser)]
pub(crate) struct InscribeChainDestinationAddresses {
  #[clap(long, help = "Inscribe <SATPOINT>")]
  pub(crate) satpoint: SatPoint,
  #[clap(long, help = "Use fee rate of <FEE_RATE> sats/vB")]
  pub(crate) fee_rate: FeeRate,
  #[clap(
    long,
    help = "Use <COMMIT_FEE_RATE> sats/vbyte for commit transaction.\nDefaults to <FEE_RATE> if unset."
  )]
  pub(crate) commit_fee_rate: Option<FeeRate>,
  #[clap(help = "Inscribe sats with contents of <dir>/inscriptions/ and send to <dir>/addresses/")]
  pub(crate) dir: PathBuf,
  #[clap(long, help = "Do not back up recovery key.")]
  pub(crate) no_backup: bool,
  #[clap(
    long,
    help = "Do not check that transactions are equal to or below the MAX_STANDARD_TX_WEIGHT of 400,000 weight units. Transactions over this limit are currently nonstandard and will not be relayed by bitcoind in its default configuration. Do not use this flag unless you understand the implications."
  )]
  pub(crate) no_limit: bool,
  #[clap(long, help = "Establish parent relationship with <PARENT>.")]
  pub(crate) parent: Option<InscriptionId>,
}

// maximum of 12
const INSCRIPTION_PER_BLOCK: usize = 12;

impl InscribeChainDestinationAddresses {
  pub(crate) fn run(self, options: Options) -> Result {
    let mut satpoint = self.satpoint;

    let inscriptions_path = self.dir.join("inscriptions");

    if !Path::new(&inscriptions_path).exists() {
      return Err(anyhow!("Error: inscriptions/ directory does not exist"));
    }

    let addresses_path = self.dir.join("addresses");

    if !Path::new(&addresses_path).exists() {
      return Err(anyhow!("Error: addresses/ directory does not exist"));
    }

    let mut inscriptions_files: Vec<DirEntry> = fs::read_dir(inscriptions_path)
      .unwrap()
      .map(|f| f.unwrap())
      .filter(|file| file.path().is_file())
      .collect();
    inscriptions_files.sort_by(|a, b| get_number_from_dir_entry(a).cmp(&get_number_from_dir_entry(b)));

    let mut addresses_files: Vec<DirEntry> = fs::read_dir(addresses_path)
      .unwrap()
      .map(|f| f.unwrap())
      .filter(|file| file.path().is_file())
      .collect();
    addresses_files.sort_by(|a, b| get_number_from_dir_entry(a).cmp(&get_number_from_dir_entry(b)));

    if inscriptions_files.len() != addresses_files.len() {
      return Err(anyhow!(
        "The number of files in 'inscriptions' and 'addresses' subdirectories is not the same: inscriptions/ has {} files and addresses/ has {} files",
        inscriptions_files.len(),
        addresses_files.len()
      ));
    }

    if inscriptions_files.len() as u64 > satpoint.offset + 1 {
      return Err(anyhow!(
        "Not enough sats: folder has {} files and output offset is {}",
        inscriptions_files.len(),
        satpoint.offset
      ));
    }

    DirBuilder::new()
      .create(&self.dir.join("inscribed"))
      .unwrap_or_default();

    for i in 0..(inscriptions_files.len().min(INSCRIPTION_PER_BLOCK)) {
      let file = inscriptions_files.get(i).unwrap();
      let file_path = file.path();
      let destination_address_file = addresses_files.get(i).unwrap();
      let destination_address_file_path = destination_address_file.path();
      let destination_address_str = fs::read_to_string(destination_address_file_path.clone())?;
      let destination_address = Address::from_str(destination_address_str.as_str().trim()).ok();

      let inscribe = Inscribe {
        dry_run: false,
        fee_rate: self.fee_rate,
        commit_fee_rate: self.commit_fee_rate,
        destination: destination_address,
        file: file_path.clone(),
        no_backup: true,
        no_limit: self.no_limit,
        satpoint: Some(satpoint),
        parent: self.parent.clone(),
        commit: None,
        keypair: None,
      };

      println!("Inscribing {} at {}, and sending to {}", file_path.clone().display(), satpoint, destination_address_str);
      let inscription = inscribe.run(options.clone())?;

      fs::rename(
        file_path.clone(),
        &self
          .dir
          .join("inscribed")
          .join(file_path.file_name().unwrap()),
      )?;

      fs::rename(
        destination_address_file_path.clone(),
        &self
          .dir
          .join("inscribed")
          .join(destination_address_file_path.file_name().unwrap()),
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
      inscriptions_files.len().min(INSCRIPTION_PER_BLOCK)
    );
    println!("\nTo continue inscribing, wait for the block to be mined and run:");
    println!("{}", self.get_resume_cli_command(satpoint));

    Ok(())
  }

  fn get_resume_cli_command(&self, updated_satpoint: SatPoint) -> String {
    let mut cli = format!(
      "ord wallet inscribe-chain-destination-addresses --fee-rate {}",
      self.fee_rate.fee(10 as usize).to_sat() as f64 / 10.0
    );
    if let Some(parent) = self.parent {
      cli.push_str(&format!(" --parent {}", parent));
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
