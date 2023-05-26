# 300 mint

## 1. Environment setup
Make sure to have `bitcoind` running, `rust` installed and no previous `ord` installation.  
1.1 Building `ord`
```
git clone git@github.com:metagood/ord.git
git checkout ord-pc-logfile-052

cargo build --release
```

1.2 Indexing
```
alias ord=./target/release/ord
ord index
```

## 2. Creating the reveal inscription
2.1 Create an ord wallet
```
ord wallet create
```
```json
{
  "mnemonic": "boost reopen else velvet stairs north glance perfect review faint debris vendor",
  "address": "tb1p7rpgexms8p35vyvk8zcs22mgp79sqvgf6q7na25sjjefpqj2whzsvkzesf",
  "public_key": "031fec97bc1edf5a674ecdee1cfef05d361ad6f07e82d6eafa8db2e60c84a898ce",
  "passphrase": ""
}
```
2.2 Fund it with just enough for one inscription (`0.00015 BTC`).  
2.3 Create the reveal txt file and inscribe it using `--dry-run` to not broadcast it. [[why?]()]
```
echo "999" > reveal.txt
ord wallet inscribe --fee-rate=1.0 --dry-run reveal.txt
```
```
{
  "commit": "f1938727c6c62854dcf2a03ad0c7472d11d6287f159ff3e762a14913e7899784",
  "inscription": "7789307187f985b659a88577c324a2e0d1d47b6185c83ed770c6a84ca0f96b20i0",
  "reveal": "7789307187f985b659a88577c324a2e0d1d47b6185c83ed770c6a84ca0f96b20",
  "fees": 293
}
commit tx hex
0100000000010190e04255f5059e62a12ba212be1e8ea3e7fcd7cf5474dc7f3ff3bd63bd41c3780000000000fdffffff029b27000000000000225120f137116e15b332bf2b65654c38875167c76b02b97449cbee6eea6785389692831b9b000000000000225120370f63d7d9540bd950f7f4c469e75bb19570173e28fb1e28b6263bff3a0ab8350140fc8067945e3476ce6477ff170d5ec37dbf50d5b13e8c62865fd58b684652da3e0a742839d9698ebf97131e462833939cfc0d5bc4f6697c159da4304b8fd9357600000000

reveal tx hex
01000000000101849789e71349a162e7f39f157f28d6112d47c7d03aa0f2dc5428c6c6278793f10000000000fdffffff011027000000000000225120e02f25580deedbf208ee84f78c1e1f5a3d780520de51668233d183468e2d308d0340ec920064008a617b7de6c2825750322397f9211b5a2f0f09c75980b7323488e34aebcf8012be43634efc131f64c21adbead67476b79cf8094f31870e7d76f6954a202c7363b0319ce5ccad2fe028603ffba0373a91373eff7bb0a3f3cc842554b2d0ac0063036f7264010118746578742f706c61696e3b636861727365743d7574662d3800043939390a6821c12c7363b0319ce5ccad2fe028603ffba0373a91373eff7bb0a3f3cc842554b2d000000000
```
2.4 Save the inscription id, commit and reveal transaction hex.  
2.4 Put this wallet on hold until it is needed again
```
mv ~/.bitcoin/wallets/ord ~/.bitcoin/wallets/ord-reveal-inscription
```
## 3. Creating the 300 inscriptions
> **Note**
> Only 20 will be created on this example

3.1 Create a new ord wallet (restarting `bitcoind` is required)
```
killall bitcoind
bitcoind --daemon

ord wallet create
```
```json
{
  "mnemonic": "brass knife empty issue submit portion need razor lend undo despair pelican",
  "address": "tb1phca60mcg0pg5c2fhtck384pqn86rt5ljsen635rrtqezpxa7pcws7dz3y8",
  "public_key": "0358bfbc7266e6e997dd7dee379b6be7e057a54d634dfcd51e9bdd112f0450f56a",
  "passphrase": ""
}
```
3.2 Fund it with enough for one inscription (vary according to file size).  
3.3 Get the file to be inscribed as the parent and inscribe it.
```
ord wallet inscribe --fee-rate=1.0 parent.txt
```
```
{
  "commit": "b7dfef2f97c626f75b1d02c9ca9f759dfc5419e245283c46cbd8befb4ec4d283",
  "inscription": "bd89bcea3f82864df2c8bc66c94949c54ad13c13e939d37b8a1b168632948eddi0",
  "reveal": "bd89bcea3f82864df2c8bc66c94949c54ad13c13e939d37b8a1b168632948edd",
  "fees": 292
}
```
3.4 Transfer the important UTXO to this wallet.  
3.5 Transfer `300 x 0.00015 = 0.045 BTC` to this wallet.  
3.6 Split the `0.045 BTC` UTXO into 300 UTXOs of `0.00014990 BTC`.
```
ord wallet split --fee-rate=1.0 --amount=14950
```
