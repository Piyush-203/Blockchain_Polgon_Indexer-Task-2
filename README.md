# Blockchain_Polgon_Indexer-Task-2

A Rust-based indexer for tracking **Polygon (USDC.e/POL)** token flows into and out of **Binance hot wallets**.  
It stores transfer data in a SQLite database and exposes a REST API for querying **cumulative netflow**.

## 🚀 Features
- Connects to Polygon via **WebSocket RPC** (Alchemy, Infura, etc.)
- Listens to `Transfer(address,address,uint256)` logs of the POL contract
- Tracks inflows/outflows into Binance wallets
- Stores all transfers + cumulative netflow in SQLite
- REST API with a simple web UI:
  - `/` → Homepage with button
  - `/netflow` → Latest cumulative netflow (JSON)
