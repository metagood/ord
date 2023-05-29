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
  "mnemonic": "behind nest scare mistake ticket divorce stumble cage same tortoise clay search",
  "address": "tb1p2denmwlt3hkdnjdamm399dp6d36y3a53fsa0vaj8fxfn6560m6sq8m0glg",
  "public_key": "023c95cb6df3cadd29fbcdba58ffc77a493bc6d3db94ab453185fa671b5424b752",
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
  "mnemonic": "innocent arrest blue admit rocket cabbage crack come leave hidden stumble problem",
  "address": "tb1pv3x70rdfd76wdqdunafw53g7m8xs9y6mvfcg6fzvpx44ewh8vcgs8c98pq",
  "public_key": "02184f648ce7542fc1c1a68fec523abfce225a362fcf01d30c527236e9abb5f8f6",
  "passphrase": ""
}
```
3.2 Fund it with just enough for one inscription (`0.00015 BTC`).  
3.3 Create the reveal txt file and inscribe it using `--dry-run` to not broadcast it.
```
echo "999" > reveal.txt
```
```sh
# for the destination use the address from ord9's wallet (step 1.4)
ord5 wallet inscribe \
--fee-rate 1.0 \
--destination tb1p2denmwlt3hkdnjdamm399dp6d36y3a53fsa0vaj8fxfn6560m6sq8m0glg \
--dry-run \
reveal.txt
```
```
{
  "commit": "50a43b347b5402c03371e4141c538444dd46a819b7d7849cb76dc2204370b1ac",
  "inscription": "7e4f534fdd58838bfebad3c518a66c76afb97259911a4c344ef23b136b409b44i0",
  "parent": null,
  "reveal": "7e4f534fdd58838bfebad3c518a66c76afb97259911a4c344ef23b136b409b44",
  "fees": 293
}
commit tx hex
0100000000010199f09cd21ae51899bc2034a55ad8a50f8cf3dc20bb0354f05641eeb1814de0c20100000000fdffffff029b2700000000000022512057045365b773be604cbabe52e21939c4527e2692cba495197327bc1fc688e3786312000000000000225120f4a9f9a1fffc0d7f11ca4f4bd371f1fd37c6a53f4d21768e03c9d156f7e9612b01402c458b78e72c3cce3ebba2941f6dd3592d1f9b29d47168665a2e0cebcea1570f6f33678cef80e1f307406b3f7dde333b700e433ecf68ab65111436328aabe65500000000

reveal tx hex
01000000000101acb1704320c26db79c84d7b719a846dd4484531c14e47133c002547b343ba4500000000000fdffffff01102700000000000022512053733dbbeb8decd9c9bddee252b43a6c7448f6914c3af6764749933d534fdea003404c8053708e073423854cbd07efad25b93b2cf4c2c9b7d3ef98a011d9164f3f2353c90cd9622998e5322820f64c24e8e2a17aac011d3b15da77a1e026087fbd294a20dfc54602ac25b8863da0a938ad7143adc7809556486faf850955c5f840e77417ac0063036f7264010118746578742f706c61696e3b636861727365743d7574662d3800043939390a6821c1dfc54602ac25b8863da0a938ad7143adc7809556486faf850955c5f840e7741700000000
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
  "mnemonic": "dress glare differ before excuse usage town quantum ladder movie cushion clip",
  "address": "tb1pysn0zy62526txvyvn4384psk2g6wv6f36dm7mw7vc7t996d0xkvqmcuj0r",
  "public_key": "031d6fa549dfdb0b48f6db233cfbbf79b711255f93b3e78c86b6373723f39d0c55",
  "passphrase": ""
}
```
4.2 Fund it with enough for the parent inscription (vary according to file size).  
4.3 Get the file to be inscribed as the parent and inscribe it.
```sh
# for the destination use the address from step 4.1
ord5 wallet inscribe \
--fee-rate 1.0 \
--destination tb1pysn0zy62526txvyvn4384psk2g6wv6f36dm7mw7vc7t996d0xkvqmcuj0r \
parent.txt
```
```json
{
  "commit": "5aff9fd0503346dc98e8f861e3fde6802faf567e8ca06c89a99bfb482a132278",
  "inscription": "db3b817f1676b7b00f6ed90dea14821b60ffc4f995af156b8b2c0f8ed5bdb829i0",
  "parent": null,
  "reveal": "db3b817f1676b7b00f6ed90dea14821b60ffc4f995af156b8b2c0f8ed5bdb829",
  "fees": 293
}
```
4.4 Transfer `300 x 0.00015 = 0.045 BTC` to this wallet.
```sh
# Using a 20 x 0.00015 = 0.003 BTC UTXO in this example
# txhash:vout
46b289bd46c53a2ef6bc96bea18c6ef277d8037cb5a967f00ecb74ad21ee2ca9:1
```
4.5 Split the `0.045 BTC` UTXO into 300 UTXOs of `0.00014990 BTC`. Don't need to wait for the previous transaction to be mined, from step 4.4.
```sh
# for the destination use the address from step 4.1
ord5 wallet split \
--fee-rate 1.0 \
--amount 14990 \
--destination tb1pysn0zy62526txvyvn4384psk2g6wv6f36dm7mw7vc7t996d0xkvqmcuj0r \
46b289bd46c53a2ef6bc96bea18c6ef277d8037cb5a967f00ecb74ad21ee2ca9:1
```
4.6 Generate the 300 files to be inscribed
```sh
# for this example this file was updated to produce only 20 files
bash create-300.sh
```
4.7 Transfer the important UTXO to this wallet.
```sh
# txhash:vout
c1c9dbf3b86ea75a5a4fa80cf7aef14c2840bc92c254f11188e656198f35f892:0
```
4.8 Identify the range where the important sats are in this output
```sh
# on this example, the output c1c9...f892:0 has 95000 sats and they are distributed as follows
# [0    - 7499] 7500 normal sats
# [7500 - 7519] 20 important sats*
# [7520 - 94999] 87480 normal sats
```
Make sure that:
* the UTXO has at least `550 sats` (dust limit) above the important sats
* the range containing important sats has at least `300 sats`

4.9 Inscribe the 300 files giving one important sat to each  
Important to note
* Wait for all the transactions to be mined before running this step.
* `--fee-rate` is set to `1.5` instead of `1.0` to avoid the possibility of getting "min relay fee not met" error in the middle of the transactions chain
* `--parent` is the inscription id inscribed on step `4.3`
* `--destination` must be the address given when the ord9 wallet was created on step `4.1`
* `--satpoint` has to point to the last important sat position, `c1c9...f892:0:7519`, because this command will consume the important sats in a bottom-up direction
* Upon completion, the `inscribe-chain` command will give the next command to be run. It's highly important to wait for the current transaction chain to be mined before running this new command, otherwise the next chain will be broken in the middle with the error "transaction chain size limit exceeded"
```sh
# for the destination use the address from ord9's wallet (step 1.4)
# it's important to use exactly the one from step 1.4, because we have the public key for that address.
ord5 wallet inscribe-chain \
--fee-rate 1.5 \
--parent db3b817f1676b7b00f6ed90dea14821b60ffc4f995af156b8b2c0f8ed5bdb829i0 \
--destination tb1p2denmwlt3hkdnjdamm399dp6d36y3a53fsa0vaj8fxfn6560m6sq8m0glg \
--satpoint c1c9dbf3b86ea75a5a4fa80cf7aef14c2840bc92c254f11188e656198f35f892:0:7519 \
300-inscriptions/
```
```
Inscribing 300-inscriptions/1.json at c1c9dbf3b86ea75a5a4fa80cf7aef14c2840bc92c254f11188e656198f35f892:0:7519
{
  "commit": "72cc2b074f19c37ca0f81e2ad9177abb8f918c42a2e8d6e16314ee8f69af8638",
  "inscription": "d4f79e33497fcb263274916d9fef0b105107a588f7b823e29babda435cf4bf27i0",
  "parent": "db3b817f1676b7b00f6ed90dea14821b60ffc4f995af156b8b2c0f8ed5bdb829i0",
  "reveal": "d4f79e33497fcb263274916d9fef0b105107a588f7b823e29babda435cf4bf27",
  "fees": 682
}
Inscribing 300-inscriptions/2.json at 72cc2b074f19c37ca0f81e2ad9177abb8f918c42a2e8d6e16314ee8f69af8638:0:7518
{
  "commit": "21d63c5491e5ea1a9598acb980d8fcf538ca4b7642ad8503ca8f9acea31f55a8",
  "inscription": "5c0f07b2aa7bf17e8528d289034210a5b6fbee9d39b043d7b9632a5a9cc3064ai0",
  "parent": "db3b817f1676b7b00f6ed90dea14821b60ffc4f995af156b8b2c0f8ed5bdb829i0",
  "reveal": "5c0f07b2aa7bf17e8528d289034210a5b6fbee9d39b043d7b9632a5a9cc3064a",
  "fees": 769
}

... [TRUNCATED] 

Inscribing 300-inscriptions/9.json at cdaca6dd542e14a9ef29cc1d7567e07928fd5782342b92d1e47955a353d24b37:0:7511
{
  "commit": "616f407f07c3ee2361f9cee0243219eae8dfb945b38e3dae1393c189624cb5f1",
  "inscription": "66311ed501c4ace9628e9b183b10633fb62cdd518a41dcfdd299ce4feb6c3542i0",
  "parent": "db3b817f1676b7b00f6ed90dea14821b60ffc4f995af156b8b2c0f8ed5bdb829i0",
  "reveal": "66311ed501c4ace9628e9b183b10633fb62cdd518a41dcfdd299ce4feb6c3542",
  "fees": 769
}
Inscribing 300-inscriptions/10.json at 616f407f07c3ee2361f9cee0243219eae8dfb945b38e3dae1393c189624cb5f1:0:7510
{
  "commit": "0ba61faa20a556f6e22a23b3c43a002395049133ce2586bbc9a06ad9fc7221ff",
  "inscription": "c49d89f52b7209b3a5fb3ab4d826c6b29a99972718ba677008460651e2ebcd38i0",
  "parent": "db3b817f1676b7b00f6ed90dea14821b60ffc4f995af156b8b2c0f8ed5bdb829i0",
  "reveal": "c49d89f52b7209b3a5fb3ab4d826c6b29a99972718ba677008460651e2ebcd38",
  "fees": 770
}

Success!
10 new inscriptions pending in the mempool.

To continue inscribing, wait for the block to be mined and run:
ord wallet inscribe-chain --fee-rate 1.5 --parent db3b817f1676b7b00f6ed90dea14821b60ffc4f995af156b8b2c0f8ed5bdb829i0 --destination tb1p2denmwlt3hkdnjdamm399dp6d36y3a53fsa0vaj8fxfn6560m6sq8m0glg --satpoint 0ba61faa20a556f6e22a23b3c43a002395049133ce2586bbc9a06ad9fc7221ff:0:7509 300-inscriptions/
```

4.10 Wait for the previous transaction chain to be mined  
4.11 Run the next `inscribe-chain` command
```
ord5 wallet inscribe-chain --fee-rate 1.5 --parent db3b817f1676b7b00f6ed90dea14821b60ffc4f995af156b8b2c0f8ed5bdb829i0 --destination tb1p2denmwlt3hkdnjdamm399dp6d36y3a53fsa0vaj8fxfn6560m6sq8m0glg --satpoint 0ba61faa20a556f6e22a23b3c43a002395049133ce2586bbc9a06ad9fc7221ff:0:7509 300-inscriptions/
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
