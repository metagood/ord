use std::str::FromStr;

use super::{print_json, Index, Options, Result};
use bitcoin::{ PackedLockTime, Witness, Script, Sequence, Transaction, TxIn, OutPoint, Txid, TxOut, Address };
use bitcoincore_rpc::RpcApi;

pub(crate) fn run(options: Options) -> Result {
  let index = Index::open(&options)?;
  index.update()?;

  // 1. use `ord wallet outputs` to find one unspent tx in your wallet
  let output_to_spend = OutPoint {
    txid: Txid::from_str("3977fa0cd63d46b23bc9af24c476d300b3415dc3ab20232b5a4a6814411c11f6")?, // replace here
    vout: 1, // replace here
  };

  let txin = TxIn {
    previous_output: output_to_spend,
    script_sig: Script::new(),
    sequence: Sequence::MAX,
    witness: Witness::new(),
  };

  // 2. use `ord wallet receive` to get an address to receive your transaction
  let to_address = Address::from_str("tb1pu6susa3tjgfjufkjrzyufu5vm5ms5a2w6erlcyuux3q8a2g57r4sdgvlv2")?; // replace here
  let output_locking_script = to_address.script_pubkey();

  // 3. Select an amount you want to send. Remember, the remaining will be used as a fee.
  //    E.g. You use a 0.09 utxo and put 0.085 in your tx output, you will pay 0.005 as a fee.
  let txout = TxOut {
    script_pubkey: output_locking_script,
    value: 850000, // replace here
  };

  let transaction = Transaction {
    version: 1,
    lock_time: PackedLockTime::ZERO,
    input: vec![txin],
    output: vec![txout]
  };

  print_json(&transaction)?;

  let client = options.bitcoin_rpc_client_for_wallet_command(false)?;
  let signed_tx = client.sign_raw_transaction_with_wallet(&transaction, None, None)?.hex;

  let txid = client.send_raw_transaction(&signed_tx)?;

  println!("Transaction: {}", txid);
  // tx sent with the commited params: https://mempool.space/signet/tx/4051765e569fb09474abebb7995dd354358b4b64902d1e38e421fe176629f039

  Ok(())
}
