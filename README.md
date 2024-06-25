# BTC oracle

A cli tool that fetches btc price from coinbase, signs the price using lamport signatures, stores the price in a taproot script,and unlocks the btc if the signed price is less.
## Running instructions

1. Clone the repository
2. Run ```cargo build``` and ```cargo run ```
