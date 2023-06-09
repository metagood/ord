# 300 mint

## 1. Environment setup: `ord14`
Make sure to have `bitcoind` running and `rust` installed.  
1.1 Building `ord`
```
ssh <user>@<ord14_ip>

git clone git@github.com:metagood/ord.git
git checkout ord-logfile-060

cargo build --release
```

1.2 Copy binary to ~/bin/ and start indexing (or download full index and log file from S3 bucket if exists)
```
cp -a ~/ord/target/release/ord ~/bin/
cd ~/bin/
ord index
```

1.3 Move (or delete) any pre-existing ord wallet. Restarting `bitcoind` is required.
```
mv ~/.bitcoin/wallets/ord ~/.bitcoin/wallets/ord-old

killall bitcoind
bitcoind --daemon
```

1.4 Create a new wallet to receive the 300 inscriptions **IMPORTANT**: this wallet's public key will need to be set as the taproot pub key env var in `ord-marketplace/packages/om-server/.env`
```
ord wallet create | tee wallet-holding-300-inscriptions.txt
```
```json
{
  "mnemonic": "behind nest scare mistake ticket divorce stumble cage same tortoise clay search",
  "address": "tb1p2denmwlt3hkdnjdamm399dp6d36y3a53fsa0vaj8fxfn6560m6sq8m0glg",
  "public_key": "023c95cb6df3cadd29fbcdba58ffc77a493bc6d3db94ab453185fa671b5424b752",
  "passphrase": ""
}
```

1.5 Set public key in `ord-marketplace/packages/om-server/.env`
```
PROJECT_OWNER_TAPROOT_PUB_KEY=023c95cb6df3cadd29fbcdba58ffc77a493bc6d3db94ab453185fa671b5424b752
```

## 2. Environment setup: `ord12`
Make sure to have `bitcoind` running and `rust` installed.  
2.1 Building `ord`
```
ssh <user>@<ord12_ip>

git clone git@github.com:metagood/ord.git
git checkout inscribe-in-mempool

cargo build --release
```

2.2 Copy binary to ~/bin/ and start indexing (or download full index and log file from S3 bucket if exists)
```
cp -a ~/ord/target/release/ord ~/bin/
cd ~/bin/
ord index
```

2.3 Move (or delete) any pre-existing ord wallet. Restarting `bitcoind` is required.
```
mv ~/.bitcoin/wallets/ord ~/.bitcoin/wallets/ord-old

killall bitcoind
bitcoind --daemon
```

## 3. Creating the reveal inscription (`ord12`)
3.1 Create an ord wallet
```
ord12 wallet create | tee reveal-offset-wallet.txt
```
```json
{
  "mnemonic": "innocent arrest blue admit rocket cabbage crack come leave hidden stumble problem",
  "address": "tb1pv3x70rdfd76wdqdunafw53g7m8xs9y6mvfcg6fzvpx44ewh8vcgs8c98pq",
  "public_key": "02184f648ce7542fc1c1a68fec523abfce225a362fcf01d30c527236e9abb5f8f6",
  "passphrase": ""
}
```
3.2 Fund it with just enough for one inscription and high fee rate (`0.005 BTC`).  
3.3 Create the reveal txt file and inscribe it using `--dry-run` to not broadcast it.
```
echo "999" > reveal.txt
```
```sh
# for the destination use the address from ord14's wallet-holding-300-inscriptions (step 1.4)
ord12 wallet inscribe \
--fee-rate 100.0 \
--destination tb1p2denmwlt3hkdnjdamm399dp6d36y3a53fsa0vaj8fxfn6560m6sq8m0glg \
--dry-run \
reveal.txt | tee offset-tx-hex
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
3.4 Test both commit and reveal tx hex to ensure they are valid and can be decoded. The below commands should print out the JSON tx code.
```
bitcoin-cli testmempoolaccept '["commit_hex", "reveal_hex"]'

bitcoin-cli decoderawtransaction <commit_hex>

bitcoin-cli decoderawtransaction <reveal_hex>
```
3.5 Put this wallet on hold until it is needed again. Restarting `bitcoind` is required.
```
mv ~/.bitcoin/wallets/ord ~/.bitcoin/wallets/ord-reveal-inscription

killall bitcoind
bitcoind --daemon
```
## 4. Creating the 300 inscriptions (ord12)
> **Note**  
> Only 20 will be created on this example

4.1 Create a new ord wallet (restarting `bitcoind` is required)
```
ord12 wallet create | tee wallet-holding-parent-dimensions.txt
```
```json
{
  "mnemonic": "dress glare differ before excuse usage town quantum ladder movie cushion clip",
  "address": "tb1pysn0zy62526txvyvn4384psk2g6wv6f36dm7mw7vc7t996d0xkvqmcuj0r",
  "public_key": "031d6fa549dfdb0b48f6db233cfbbf79b711255f93b3e78c86b6373723f39d0c55",
  "passphrase": ""
}
```
4.2 Danny login to machine with wallet holding parent dimensions inscription
4.3 Danny send parent inscription to wallet created in 4.1
```sh
# for the destination use the address from step 4.1
ord wallet send --fee-rate 100.0 tb1pysn0zy62526txvyvn4384psk2g6wv6f36dm7mw7vc7t996d0xkvqmcuj0r 2dbdf9ebbec6be793fd16ae9b797c7cf968ab2427166aaf390b90b71778266abi0
```
4.4 Transfer `300 x 0.0002 = 0.065 BTC` (includes .005 extra to be safe) to the wallet created in 4.1, then find the outpoint to this utxo.
```sh
ord12 wallet outputs

# txhash:vout
46b289bd46c53a2ef6bc96bea18c6ef277d8037cb5a967f00ecb74ad21ee2ca9:1
```
4.5 Split the `0.065 BTC` UTXO into 300 UTXOs of `0.00020000 BTC`. Don't need to wait for the previous transaction to be mined, from step `4.4`.
```sh
# for the destination use the address from step 4.1
ord12 wallet split \
--fee-rate 100.0 \
--amount 20000 \
--destination tb1pysn0zy62526txvyvn4384psk2g6wv6f36dm7mw7vc7t996d0xkvqmcuj0r \
46b289bd46c53a2ef6bc96bea18c6ef277d8037cb5a967f00ecb74ad21ee2ca9:1
```
4.6 Generate the 300 files to be inscribed
```sh
# for this example this file was updated to produce only 20 files
bash create-300.sh
```
4.7 Danny transfer the important UTXO to this wallet.
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
* `--destination` must be the address given when the ord12 wallet was created on step `4.1`
* `--satpoint` has to point to the last important sat position, `c1c9...f892:0:7519`, because this command will consume the important sats in a bottom-up direction
* Upon completion, the `inscribe-chain` command will give the next command to be run. It's highly important to wait for the current transaction chain to be mined before running this new command, otherwise the next chain will be broken in the middle with the error "transaction chain size limit exceeded"
```sh
# for the destination use the address from ord12's wallet (step 1.4)
# it's important to use exactly the one from step 1.4, because we have the public key for that address.
ord12 wallet inscribe-chain \
--fee-rate 1.5 \
--parent db3b817f1676b7b00f6ed90dea14821b60ffc4f995af156b8b2c0f8ed5bdb829i0 \
--destination tb1p2denmwlt3hkdnjdamm399dp6d36y3a53fsa0vaj8fxfn6560m6sq8m0glg \
--satpoint c1c9dbf3b86ea75a5a4fa80cf7aef14c2840bc92c254f11188e656198f35f892:0:7519 \
300-inscriptions/ | tee inscribe-chain/inscribe-chain-{num}.txt
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
ord12 wallet inscribe-chain --fee-rate 1.5 --parent db3b817f1676b7b00f6ed90dea14821b60ffc4f995af156b8b2c0f8ed5bdb829i0 --destination tb1p2denmwlt3hkdnjdamm399dp6d36y3a53fsa0vaj8fxfn6560m6sq8m0glg --satpoint 0ba61faa20a556f6e22a23b3c43a002395049133ce2586bbc9a06ad9fc7221ff:0:7509 300-inscriptions/
```
4.12 Loop through steps `4.10` and `4.11` until the folder `300-inscriptions` has no more files left to be inscribed.

## 5. Reveal
5.1 Broadcast the commit transaction from step `3.3`
```
bitcoin-cli sendrawtransaction 0100000000010199f09cd21ae51899bc2034a55ad8a50f8cf3dc20bb0354f05641eeb1814de0c20100000000fdffffff029b2700000000000022512057045365b773be604cbabe52e21939c4527e2692cba495197327bc1fc688e3786312000000000000225120f4a9f9a1fffc0d7f11ca4f4bd371f1fd37c6a53f4d21768e03c9d156f7e9612b01402c458b78e72c3cce3ebba2941f6dd3592d1f9b29d47168665a2e0cebcea1570f6f33678cef80e1f307406b3f7dde333b700e433ecf68ab65111436328aabe65500000000
```
5.2 Broadcast the reveal transaction from step `3.3`
```
bitcoin-cli sendrawtransaction 01000000000101acb1704320c26db79c84d7b719a846dd4484531c14e47133c002547b343ba4500000000000fdffffff01102700000000000022512053733dbbeb8decd9c9bddee252b43a6c7448f6914c3af6764749933d534fdea003404c8053708e073423854cbd07efad25b93b2cf4c2c9b7d3ef98a011d9164f3f2353c90cd9622998e5322820f64c24e8e2a17aac011d3b15da77a1e026087fbd294a20dfc54602ac25b8863da0a938ad7143adc7809556486faf850955c5f840e77417ac0063036f7264010118746578742f706c61696e3b636861727365743d7574662d3800043939390a6821c1dfc54602ac25b8863da0a938ad7143adc7809556486faf850955c5f840e7741700000000
```
