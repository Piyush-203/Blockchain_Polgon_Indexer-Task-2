# Blockchain_Polgon_Indexer-Task-2

A real-time blockchain data indexing system that monitors the Polygon network for POL token transfers involving **Binance exchange addresses**.
The system extracts transfer logs, persists them into a SQLite database, maintains cumulative netflow metrics **(inflow – outflow)**, and provides a REST API and CLI for querying.

## 🚀 Features
- Real-Time Indexing – Processes live blocks from Polygon WebSocket RPC.

- Targeted Monitoring – Tracks POL token transfers with Binance hot wallets.

- SQLite Database Storage – Persists raw transfers and cumulative netflow.

- Cumulative Netflow Metric – Continuously updated inflow/outflow value.

- CLI Query Tool – Fetch latest netflow directly from terminal.

- REST API Endpoints – Access cumulative netflow via HTTP.

- Configurable via .env – Database path, contract address, Binance addresses, RPC endpoint.


## 🚀 Quick Start
1. Clone the repository
- `git clone <repository-url>`
- `cd Blockchain_Polgon_Indexer-Task-2`

2. Configure Environment
- Create a .env file in the root directory:
