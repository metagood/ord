# ord wallet inscribe-chain
> Broadcast a transaction chain to the mempool inscribing 10 ordinals, each containing one special sat extracted from a designated output.

![](chain.png)

## Example
1. Create the files to be inscribed:
    ```bash
    mkdir files/
    echo "1" > files/1.txt
    echo "2" > files/2.txt
    echo "3" > files/3.txt
    echo "4" > files/4.txt
    echo "5" > files/5.txt
    echo "6" > files/6.txt
    echo "7" > files/7.txt
    echo "8" > files/8.txt
    echo "9" > files/9.txt
    echo "10" > files/10.txt
    echo "11" > files/11.txt
    echo "12" > files/12.txt
    echo "13" > files/13.txt
    echo "14" > files/14.txt
    echo "15" > files/15.txt
    ```
2. Get the special utxo
    ```bash
    ord wallet outputs
    ```
    ```json
    [
        {
          "output": "7f320d87dd2d011ba9a3dbc66c46aed4b0b3a9a0a4d1c93fe3ed97ab280463f5:8",
          "amount": 15000
        }
    ]
    ```
    Lets pick sats from `10_001` to `10_015` as the special ones, so my `satpoint` is:
    ```
    7f320d87dd2d011ba9a3dbc66c46aed4b0b3a9a0a4d1c93fe3ed97ab280463f5:8:10014
    ```
3. Run the `inscribe-chain` command
    > **note**
    > You must have at least 10 additional available utxos, each containing a minimum of 12,000 sats, when running this command. If you don't have enough, use the `ord wallet split` command to break one of your larger utxos into several smaller ones..

    ```bash
    ord wallet inscribe-chain --fee-rate=1.0 --satpoint=7f320d87dd2d011ba9a3dbc66c46aed4b0b3a9a0a4d1c93fe3ed97ab280463f5:8:10014 files/
    ```
    The sats `24015`, `24014`, `24013`, `24012`, `24011`, `24010`, `24009`, `24008`, `24007` and `24006` were extracted from the special utxo and given to the inscribed files `1.json`, `2.json`, ... `10.json` in this order.

4. Now we have to **wait for the block to be mined** and then run the next command (that was given when the previous completed):
    > **warning**
    > If you run the following command while the previous transaction chain is still pending in the mempool, you will be able to inscribe only 2 more inscriptions and will have a commit transaction without the reveal. In other words, it will waste valuable special sats and will require a manual task to fix the transaction chain.
    ```bash
    ord wallet inscribe-chain --fee-rate 1 --satpoint d3ecd1d753e09d2ca3a69fe1a36a7b829e3c096d44f101d11c5a3f6aef1e757b:0:10004 files/
    ```
