# 300 mint

## 1. Environment setup
Make sure to have `bitcoind` running, `rust` installed and no previous `ord` installation.  
1.1 Building `ord`
```
git clone git@github.com:metagood/ord.git
git checkout inscribe-in-mempool

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
```json
{
  "commit": "b7dfef2f97c626f75b1d02c9ca9f759dfc5419e245283c46cbd8befb4ec4d283",
  "inscription": "bd89bcea3f82864df2c8bc66c94949c54ad13c13e939d37b8a1b168632948eddi0",
  "reveal": "bd89bcea3f82864df2c8bc66c94949c54ad13c13e939d37b8a1b168632948edd",
  "fees": 292
}
```
3.4 Transfer `300 x 0.00015 = 0.045 BTC` to this wallet.
```sh
# Using a 20 x 0.00015 = 0.003 BTC UTXO in this example
# txhash:vout
06aac693f19f8d384764bfe090719d4b373ea11d536b2ef337fc3f6f1334e916:0
```
3.5 Split the `0.045 BTC` UTXO into 300 UTXOs of `0.00014990 BTC`.
```
ord wallet split --fee-rate=1.0 --amount=14990 --destination=tb1phca60mcg0pg5c2fhtck384pqn86rt5ljsen635rrtqezpxa7pcws7dz3y8 06aac693f19f8d384764bfe090719d4b373ea11d536b2ef337fc3f6f1334e916:0
```
3.6 Generate the 300 files to be inscribed
```
bash create-300.sh
```
3.7 Transfer the important UTXO to this wallet.
```sh
# txhash:vout
967d433a2068f742b25e35e106c9d476d70632cb291c3b8b498adf699d938de9:0
```
3.8 Identify the range where the important sats are in this output
```sh
# on this example, the output 967d...8de9:0 has 9500 sats and they are distributed as follows
# [0    - 7499] 7500 normal sats
# [7500 - 7519] 20 important sats*
# [7520 - 9499] 1980 normal sats
```
Make sure that:
* the UTXO has at least `550 sats` (dust limit) above the important sats
* the range containing important sats has at least 300 sats

3.9 Inscribe the 300 files giving one important sat to each
```
ord wallet inscribe-chain --fee-rate=1.5 --parent=bd89bcea3f82864df2c8bc66c94949c54ad13c13e939d37b8a1b168632948eddi0 --satpoint=967d433a2068f742b25e35e106c9d476d70632cb291c3b8b498adf699d938de9:0:7519 300-inscriptions/
```
```
Inscribing 300-inscriptions/1.json at 967d433a2068f742b25e35e106c9d476d70632cb291c3b8b498adf699d938de9:0:7519
{
  "commit": "6382e2d71145caf02b21f7786554823ddadc2e1cddc596cd1ad081b03dd4bfaa",
  "inscription": "aa928749abcbb93cff404a75ddede7fd46ca64aee6c1d2eb06c5d64b51908f2ei0",
  "parent": "bd89bcea3f82864df2c8bc66c94949c54ad13c13e939d37b8a1b168632948eddi0",
  "reveal": "aa928749abcbb93cff404a75ddede7fd46ca64aee6c1d2eb06c5d64b51908f2e",
  "fees": 769
}
Inscribing 300-inscriptions/2.json at 6382e2d71145caf02b21f7786554823ddadc2e1cddc596cd1ad081b03dd4bfaa:0:7518
{
  "commit": "2dc1e3dbf866774f601e1a5563197e0e379d304f30d633432145b8ce779739f9",
  "inscription": "905c1f7c5a4e982d84a78a859bb9156163dbc010d6e5de67e7111d6f9a98cefai0",
  "parent": "bd89bcea3f82864df2c8bc66c94949c54ad13c13e939d37b8a1b168632948eddi0",
  "reveal": "905c1f7c5a4e982d84a78a859bb9156163dbc010d6e5de67e7111d6f9a98cefa",
  "fees": 769
}

... [TRUNCATED]

Inscribing 300-inscriptions/9.json at 843a1018b70363d360fcf51573c191d2ca1060a5fcbb4075f24d817f6f59a11e:0:7511
{
  "commit": "5d4bda0e9f137f1c4f9f077ed7b637552b506039e7bbc46fe02b424af5360f35",
  "inscription": "da929a6715e0231c3643807ac65126b2aab033fa48e3069a90cb226fbf68e43ci0",
  "parent": "bd89bcea3f82864df2c8bc66c94949c54ad13c13e939d37b8a1b168632948eddi0",
  "reveal": "da929a6715e0231c3643807ac65126b2aab033fa48e3069a90cb226fbf68e43c",
  "fees": 769
}
Inscribing 300-inscriptions/10.json at 5d4bda0e9f137f1c4f9f077ed7b637552b506039e7bbc46fe02b424af5360f35:0:7510
{
  "commit": "a2a627094403497b477dbeec56ab0cefe2a662f57762e485dd815da0da8ea69f",
  "inscription": "f73c80bdd87e4df1eb6c41fe815515e656930ca67cdfa6a5e4ec07acb3de7c3di0",
  "parent": "bd89bcea3f82864df2c8bc66c94949c54ad13c13e939d37b8a1b168632948eddi0",
  "reveal": "f73c80bdd87e4df1eb6c41fe815515e656930ca67cdfa6a5e4ec07acb3de7c3d",
  "fees": 770
}

Success!
10 new inscriptions pending in the mempool.

To continue inscribing, wait for the block to be mined and run:
ord wallet inscribe-chain --fee-rate 1.5 --parent bd89bcea3f82864df2c8bc66c94949c54ad13c13e939d37b8a1b168632948eddi0 --satpoint a2a627094403497b477dbeec56ab0cefe2a662f57762e485dd815da0da8ea69f:0:7509 300-inscriptions/
```
Important to note
* `--fee-rate` is set to `1.5` instead of `1.0` to avoid the possibility of getting "min relay fee not met" error in the middle of the transactions chain
* `--satpoint` has to point to the last important sat position, `967d...8de9:0:7519`, because this command will consume all 300 sats above it, including it
* Upon completion, the `inscribe-chain` will give the next command to be run. It's highly important to wait for the current transaction chain to be mined before running this new command, otherwise the next chain will be broken in the middle with the error "transaction chain size limit exceeded"

3.10 Wait for the previous transaction chain to be mined  
3.11 Run the next `inscribe-chain` command
```
ord wallet inscribe-chain --fee-rate 1.5 --parent bd89bcea3f82864df2c8bc66c94949c54ad13c13e939d37b8a1b168632948eddi0 --satpoint a2a627094403497b477dbeec56ab0cefe2a662f57762e485dd815da0da8ea69f:0:7509 300-inscriptions/
```
3.12 Loop through steps `3.10` and `3.11` until the folder `300-inscriptions` has no more files left to be inscribed.

## 4. Reveal
4.1 Broadcast the commit transaction from step `2.3`
```
bitcoin-cli sendrawtransaction 0100000000010190e04255f5059e62a12ba212be1e8ea3e7fcd7cf5474dc7f3ff3bd63bd41c3780000000000fdffffff029b27000000000000225120f137116e15b332bf2b65654c38875167c76b02b97449cbee6eea6785389692831b9b000000000000225120370f63d7d9540bd950f7f4c469e75bb19570173e28fb1e28b6263bff3a0ab8350140fc8067945e3476ce6477ff170d5ec37dbf50d5b13e8c62865fd58b684652da3e0a742839d9698ebf97131e462833939cfc0d5bc4f6697c159da4304b8fd9357600000000
```
4.2 Broadcast the reveal transaction from step `2.3`
```
bitcoin-cli sendrawtransaction 01000000000101849789e71349a162e7f39f157f28d6112d47c7d03aa0f2dc5428c6c6278793f10000000000fdffffff011027000000000000225120e02f25580deedbf208ee84f78c1e1f5a3d780520de51668233d183468e2d308d0340ec920064008a617b7de6c2825750322397f9211b5a2f0f09c75980b7323488e34aebcf8012be43634efc131f64c21adbead67476b79cf8094f31870e7d76f6954a202c7363b0319ce5ccad2fe028603ffba0373a91373eff7bb0a3f3cc842554b2d0ac0063036f7264010118746578742f706c61696e3b636861727365743d7574662d3800043939390a6821c12c7363b0319ce5ccad2fe028603ffba0373a91373eff7bb0a3f3cc842554b2d000000000
```
