use {
  super::*,
  bitcoin::{PackedLockTime, Witness},
};

#[derive(Debug, Parser)]
pub(crate) struct Split {
  #[clap(long)]
  pub(crate) fee_rate: FeeRate,
  #[clap(long)]
  pub(crate) amount: u64,
  #[clap(long)]
  pub(crate) destination: Address,
  #[clap()]
  pub(crate) outpoint: OutPoint,
}

impl Split {
  pub(crate) fn run(&self, options: Options) -> Result {
    let client = options.bitcoin_rpc_client_for_wallet_command(false)?;

    let output_to_spend = OutPoint {
      txid: self.outpoint.txid,
      vout: self.outpoint.vout,
    };

    let txin = TxIn {
      previous_output: output_to_spend,
      script_sig: Script::new(),
      sequence: Sequence::MAX,
      witness: Witness::new(),
    };

    let output_tx = client.get_raw_transaction(&self.outpoint.txid, None)?;
    let output_sats = output_tx.output[self.outpoint.vout as usize].value;
    let new_outputs_quantity = output_sats / self.amount;

    let to_address = self.destination.clone();
    let output_locking_script = to_address.script_pubkey();
    let mut outputs: Vec<TxOut> = vec![];

    for _ in 0..new_outputs_quantity {
      let txout = TxOut {
        script_pubkey: output_locking_script.clone(),
        value: self.amount,
      };

      outputs.push(txout);
    }

    let mut transaction = Transaction {
      version: 1,
      lock_time: PackedLockTime::ZERO,
      input: vec![txin],
      output: outputs,
    };

    let fee = self.fee_rate.fee(transaction.vsize());
    transaction.output[(new_outputs_quantity - 1) as usize].value = transaction.output
      [(new_outputs_quantity - 1) as usize]
      .value
      .checked_sub(fee.to_sat())
      .context(format!(
        "fees higher than output amount: {} > {}",
        fee, self.amount
      ))?;

    let signed_tx = client
      .sign_raw_transaction_with_wallet(&transaction, None, None)?
      .hex;
    let txid = client.send_raw_transaction(&signed_tx)?;

    println!("Transaction: {}", txid);

    Ok(())
  }
}
