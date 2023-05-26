# 300 mint

## 1. Environment setup
Make sure to have `bitcoind` running, `rust` installed and no previous `ord` installation.  
1.1 Building `ord`
```
git@github.com:metagood/ord.git
git checkout ord-pc-logfile-052
cargo build --release
```

1.2 Indexing
```
alias ord=./target/release/ord
ord index
```

## 2. Creating the reveal inscription
Create a new ord wallet and fund it with a single utxo containing just enough for one inscription (> `12_000 sats`).
```
ord wallet create
```
```

```
