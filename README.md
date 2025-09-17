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
```
- git clone <repository-url>
- cd Blockchain_Polgon_Indexer-Task-2
  ```

2. Configure Environment
- Create a .env file in the root directory:
  
3. Build and Run
```
- cargo build --release
- cargo run
  ```

4. CLI Mode
   
- Query the latest cumulative netflow directly from the DB:
```
- cargo run -- query
  ```
- Example output
```
📊 Current cumulative netflow: -14800000
```

5. 🌐 HTTP API
   
- The REST API runs on the configured port (default: 3000).
- Endpoints
  - GET / → Landing page with a button to view netflow.
  - GET /netflow → Returns JSON:
 
6. 🛠️ Architecture
- System Components

  - Indexer → Connects to Polygon via WebSocket, listens for Transfer logs.
  
  - Database → SQLite stores raw transfers + netflow snapshots.
  
  - REST API → Provides access to the latest netflow.
  
  - CLI → Simple query mode for direct terminal access
   ``` 
   ┌──────────────┐     ┌───────────────┐     ┌─────────────┐
   │ Polygon Node │──▶─│ Indexer Logic  │──▶─│ SQLite DB   │
   └──────────────┘     └───────────────┘     └─────────────┘
                                    │
                   ┌────────────────┴─────────────┐
                   │                              │
            ┌─────────────┐                 ┌──────────────┐
            │   REST API  │                 │ CLI Query    │
            └─────────────┘                 └──────────────┘

  ```
7. 📦 Requirements

- **Rust**(1.75+ recommended)

- Polygon WebSocket RPC endpoint (e.g., Alchemy, Infura)
