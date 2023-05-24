use bitcoin::{
  secp256k1::PublicKey,
  util::bip32::{self, ExtendedPubKey},
};

use super::*;

#[derive(Serialize)]
struct Output {
  mnemonic: Mnemonic,
  address: Address,
  public_key: PublicKey,
  passphrase: Option<String>,
}

#[derive(Debug, Parser)]
pub(crate) struct Create {
  #[clap(
    long,
    default_value = "",
    help = "Use <PASSPHRASE> to derive wallet seed."
  )]
  pub(crate) passphrase: String,
}

impl Create {
  pub(crate) fn run(self, options: Options) -> Result {
    let mut entropy = [0; 16];
    rand::thread_rng().fill_bytes(&mut entropy);

    let mnemonic = Mnemonic::from_entropy(&entropy)?;
    let seed = mnemonic.to_seed(self.passphrase.clone());
    let secp = Secp256k1::new();
    let root = bip32::ExtendedPrivKey::new_master(options.chain().network(), &seed)?;

    let xprv = root.derive_priv(&secp, &DerivationPath::from_str("m/86'/1'/0'")?)?;
    let xpub = ExtendedPubKey::from_priv(&secp, &xprv);
    let public_key = xpub
      .derive_pub(&secp, &DerivationPath::from_str("m/0/0")?)?
      .public_key;

    initialize_wallet(&options, mnemonic.to_seed(self.passphrase.clone()))?;

    let address = options
      .bitcoin_rpc_client_for_wallet_command(false)?
      .get_new_address(None, Some(bitcoincore_rpc::json::AddressType::Bech32m))?;

    print_json(Output {
      mnemonic,
      address,
      public_key,
      passphrase: Some(self.passphrase),
    })?;

    Ok(())
  }
}
