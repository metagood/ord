# 300 mint

## 1. Environment setup: `ord9`
Make sure to have `bitcoind` running and `rust` installed.  
1.1 Building `ord`
```
ssh <user>@<ord9_ip>

git clone git@github.com:metagood/ord.git
git checkout ord-pc-logfile-052

cargo build --release
```

1.2 Indexing
```
alias ord9=./target/release/ord
ord9 index
```

1.3 Move (or delete) any pre-existing ord wallet. Restarting `bitcoind` is required.
```
mv ~/.bitcoin/wallets/ord ~/.bitcoin/wallets/ord-old

killall bitcoind
bitcoind --daemon
```

1.4 Create a new wallet to receive the 300 inscriptions
```
ord9 wallet create
```
```json
{
  "mnemonic": "stadium three aspect awesome oyster ski bring wash call fan stage display",
  "address": "tb1p8xgst5e8wy6zwxdqm0cany69mqxghplva2xqzee0hn57twdp205qzg7trw",
  "public_key": "027f24948757f56793856ca16eb0b2e34644e8643dddd5fffda387515be0e3ad8a",
  "passphrase": ""
}
```

## 2. Environment setup: `ord5`
Make sure to have `bitcoind` running and `rust` installed.  
1.1 Building `ord`
```
ssh <user>@<ord5_ip>

git clone git@github.com:metagood/ord.git
git checkout inscribe-in-mempool

cargo build --release
```

1.2 Indexing
```
alias ord5=./target/release/ord
ord5 index
```

1.3 Move (or delete) any pre-existing ord wallet. Restarting `bitcoind` is required.
```
mv ~/.bitcoin/wallets/ord ~/.bitcoin/wallets/ord-old

killall bitcoind
bitcoind --daemon
```

## 3. Creating the reveal inscription
3.1 Create an ord wallet
```
ord5 wallet create
```
```json
{
  "mnemonic": "butter tuition draft reopen cereal match soft average thumb comfort dilemma labor",
  "address": "tb1psh7f8tvrrdd0re20ntnsayhd5g7pu2ckvx7clsh2f6tyvuyh8peq2stc40",
  "public_key": "037335a3c64eed26736c5fb83321441742f7e6cb8d8f10a2b2eb0397027dd200f7",
  "passphrase": ""
}
```
3.2 Fund it with just enough for one inscription (`0.00015 BTC`).  
3.3 Create the reveal txt file and inscribe it using `--dry-run` to not broadcast it.
```
echo "999" > reveal.txt
```
```sh
# use the address from ord9's wallet for destination (step 1.4)
ord5 wallet inscribe \
--fee-rate 1.0 \
--destination tb1p8xgst5e8wy6zwxdqm0cany69mqxghplva2xqzee0hn57twdp205qzg7trw \
--dry-run \
reveal.txt
```
```
{
  "commit": "1cc502b83155da5f7e46ae73d16f30d0b3b5ae3f4a113740c0d025759b0c4e82",
  "inscription": "746fa2816f997527eab5007c622f402ae4a1634f1ef41e2744cf82ebe6e073c2i0",
  "parent": null,
  "reveal": "746fa2816f997527eab5007c622f402ae4a1634f1ef41e2744cf82ebe6e073c2",
  "fees": 293
}
commit tx hex
010000000001010acc93eb8f2bee57d3ef2257eb520115a8536f80f0beab8bacab24b7d97161920000000000fdffffff029b270000000000002251201b8d03f5db831a5a19e0b23cdc7a0505109ef39b57e220b7e1c22fc62818f3a863120000000000002251208397d49b90850e3da31ed72edeeb39fcb9c941d637754eb33827ee4abde06b96014083679ccce44cbcbd7465a0bfb6de1a26ef955cb115ce1368f01f2c8a7c6ff8e51b94affa6d3c65bac929eeabe5576542ddcbe0b6eed533caf24fd1699770f37b00000000

reveal tx hex
01000000000101824e0c9b7525d0c04037114a3faeb5b3d0306fd173ae467e5fda5531b802c51c0000000000fdffffff011027000000000000225120399105d32771342719a0dbf1d99345d80c8b87ecea8c01672fbce9e5b9a153e80340d2fd4e0d3b7316fd12786c7698be810a182a01f1ad18d6b4942efbf80ec5b0294cecf991294232bbfcbaf9e6d08ec3695ddb1564139d44e824238e7916ac669d4a206b7ac0f8123521249c460e9e43c417b94eeb9912485699ca3a59463ab4396424ac0063036f7264010118746578742f706c61696e3b636861727365743d7574662d3800043939390a6821c06b7ac0f8123521249c460e9e43c417b94eeb9912485699ca3a59463ab439642400000000
```
3.4 Put this wallet on hold until it is needed again. Restarting `bitcoind` is required.
```
mv ~/.bitcoin/wallets/ord ~/.bitcoin/wallets/ord-reveal-inscription

killall bitcoind
bitcoind --daemon
```
## 4. Creating the 300 inscriptions
> **Note**  
> Only 20 will be created on this example

4.1 Create a new ord wallet (restarting `bitcoind` is required)
```
ord5 wallet create
```
```json
{
  "mnemonic": "burst fiction thank assault own double skill lyrics blind country tribe moment",
  "address": "tb1pmqwwx0kdhdekfyzwc33mfctcf6j9z4huvc2m6kru9stpws4veguqkq9j0s",
  "public_key": "035d7a50961320da1cd59b75ee2ab8ce52f88d1f1ef65fec8be7dc958e1f502381",
  "passphrase": ""
}
```
4.2 Fund it with enough for the parent inscription (vary according to file size).  
4.3 Get the file to be inscribed as the parent and inscribe it.
```sh
# use the address from ord9's wallet for destination (step 1.4)
ord5 wallet inscribe \
--fee-rate 1.0 \
--destination tb1p8xgst5e8wy6zwxdqm0cany69mqxghplva2xqzee0hn57twdp205qzg7trw \
parent.txt
```
```json
{
  "commit": "b7dfef2f97c626f75b1d02c9ca9f759dfc5419e245283c46cbd8befb4ec4d283",
  "inscription": "bd89bcea3f82864df2c8bc66c94949c54ad13c13e939d37b8a1b168632948eddi0",
  "reveal": "bd89bcea3f82864df2c8bc66c94949c54ad13c13e939d37b8a1b168632948edd",
  "fees": 292
}
```
4.4 Transfer `300 x 0.00015 = 0.045 BTC` to this wallet.
```sh
# Using a 20 x 0.00015 = 0.003 BTC UTXO in this example
# txhash:vout
06aac693f19f8d384764bfe090719d4b373ea11d536b2ef337fc3f6f1334e916:0
```
4.5 Split the `0.045 BTC` UTXO into 300 UTXOs of `0.00014990 BTC`.
```
ord wallet split \
--fee-rate 1.0 \
--amount 14990 \
06aac693f19f8d384764bfe090719d4b373ea11d536b2ef337fc3f6f1334e916:0
```
4.6 Generate the 300 files to be inscribed
```
bash create-300.sh
```
4.7 Transfer the important UTXO to this wallet.
```sh
# txhash:vout
967d433a2068f742b25e35e106c9d476d70632cb291c3b8b498adf699d938de9:0
```
4.8 Identify the range where the important sats are in this output
```sh
# on this example, the output 967d...8de9:0 has 9500 sats and they are distributed as follows
# [0    - 7499] 7500 normal sats
# [7500 - 7519] 20 important sats*
# [7520 - 9499] 1980 normal sats
```
Make sure that:
* the UTXO has at least `550 sats` (dust limit) above the important sats
* the range containing important sats has at least 300 sats

4.9 Inscribe the 300 files giving one important sat to each
```
ord wallet inscribe-chain \
--fee-rate 1.5 \
--parent bd89bcea3f82864df2c8bc66c94949c54ad13c13e939d37b8a1b168632948eddi0 \
--destination tb1phca60mcg0pg5c2fhtck384pqn86rt5ljsen635rrtqezpxa7pcws7dz3y8 \
--satpoint 967d433a2068f742b25e35e106c9d476d70632cb291c3b8b498adf699d938de9:0:7519 \
300-inscriptions/
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
* `--destination` must be the address given when the wallet was created on step `4.1`
* `--satpoint` has to point to the last important sat position, `967d...8de9:0:7519`, because this command will consume all 300 sats above it, including it
* Upon completion, the `inscribe-chain` will give the next command to be run. It's highly important to wait for the current transaction chain to be mined before running this new command, otherwise the next chain will be broken in the middle with the error "transaction chain size limit exceeded"

4.10 Wait for the previous transaction chain to be mined  
4.11 Run the next `inscribe-chain` command
```
ord wallet inscribe-chain --fee-rate 1.5 --parent bd89bcea3f82864df2c8bc66c94949c54ad13c13e939d37b8a1b168632948eddi0 --destination tb1phca60mcg0pg5c2fhtck384pqn86rt5ljsen635rrtqezpxa7pcws7dz3y8 --satpoint a2a627094403497b477dbeec56ab0cefe2a662f57762e485dd815da0da8ea69f:0:7509 300-inscriptions/
```
4.12 Loop through steps `4.10` and `4.11` until the folder `300-inscriptions` has no more files left to be inscribed.

## 4. Reveal
4.1 Broadcast the commit transaction from step `2.3`
```
bitcoin-cli sendrawtransaction 0100000000010190e04255f5059e62a12ba212be1e8ea3e7fcd7cf5474dc7f3ff3bd63bd41c3780000000000fdffffff029b27000000000000225120f137116e15b332bf2b65654c38875167c76b02b97449cbee6eea6785389692831b9b000000000000225120370f63d7d9540bd950f7f4c469e75bb19570173e28fb1e28b6263bff3a0ab8350140fc8067945e3476ce6477ff170d5ec37dbf50d5b13e8c62865fd58b684652da3e0a742839d9698ebf97131e462833939cfc0d5bc4f6697c159da4304b8fd9357600000000
```
4.2 Broadcast the reveal transaction from step `2.3`
```
bitcoin-cli sendrawtransaction 01000000000101849789e71349a162e7f39f157f28d6112d47c7d03aa0f2dc5428c6c6278793f10000000000fdffffff011027000000000000225120e02f25580deedbf208ee84f78c1e1f5a3d780520de51668233d183468e2d308d0340ec920064008a617b7de6c2825750322397f9211b5a2f0f09c75980b7323488e34aebcf8012be43634efc131f64c21adbead67476b79cf8094f31870e7d76f6954a202c7363b0319ce5ccad2fe028603ffba0373a91373eff7bb0a3f3cc842554b2d0ac0063036f7264010118746578742f706c61696e3b636861727365743d7574662d3800043939390a6821c12c7363b0319ce5ccad2fe028603ffba0373a91373eff7bb0a3f3cc842554b2d000000000
```
