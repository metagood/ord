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
  pub(crate) dry_run: bool,
  #[clap(long, help = "Send inscription to <DESTINATION>.")]
  pub(crate) destination: Option<Address>,
  #[clap(long, help = "Establish parent relationship with <PARENT>.")]
  pub(crate) parent: Option<InscriptionId>,
}

impl InscribeChain {
  pub(crate) fn run(self, options: Options) -> Result {
    let mut satpoint = self.satpoint;

    let dir = self.dir.read_dir()?;
    let files: Vec<_> = dir.collect();

    if files.len() as u64 > satpoint.offset {
      return Err(anyhow!(
        "Not enough sats: folder has {} files and output offset is {}",
        files.len(),
        satpoint.offset
      ));
    }

    for file in files {
      let inscribe = Inscribe {
        dry_run: false,
        fee_rate: self.fee_rate,
        commit_fee_rate: self.commit_fee_rate,
        destination: self.destination.clone(),
        file: file?.path(),
        no_backup: self.no_backup,
        no_limit: self.no_limit,
        satpoint: Some(satpoint),
        parent: self.parent.clone(),
      };

      let inscription = inscribe.run(options.clone())?;

      // update inscribe satpoint
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

    Ok(())
  }
}
