// use ethers::prelude::*;
// // use ethers::contract::abigen;
// use ethers::types::{Filter, H160, H256, U256};
// use ethers::utils::keccak256;
// use anyhow::Result;
// use std::sync::Arc;
// use std::str::FromStr;
// use rusqlite::{params, Connection};
// use tokio_stream::StreamExt;

// abigen!(
//     ERC20,
//     r#"[
//         event Transfer(address indexed from, address indexed to, uint256 value)
//     ]"#
// );

// #[tokio::main]
// async fn main() -> Result<()> {
//     // Load environment variables from .env
//     dotenv::dotenv().ok();

//     let ws_url = std::env::var("POLYGON_RPC_WS")
//         .expect("Missing POLYGON_RPC_WS in .env");

//     let ws = Ws::connect(ws_url).await?;
//     // let provider = Provider::new(ws);
//     // let provider = Arc::new(provider);
//     let provider = Arc::new(Provider::new(ws));

//     let pol_address: H160 = "0x0000000000000000000000000000000000001010"
//         .parse()
//         .unwrap();
//     let pol_contract = ERC20::new(pol_address, provider.clone());

//     println!("✅ Connected! Listening for new blocks...");
//     let binance_addresses: Vec<H160> = vec![
//         H160::from_str("0xF977814e90dA44bFA03b6295A0616a897441aceC").unwrap(),
//         H160::from_str("0xe7804c37c13166fF0b37F5aE0BB07A3aEbb6e245").unwrap(),
//         H160::from_str("0x505e71695E9bc45943c58adEC1650577BcA68fD9").unwrap(),
//         H160::from_str("0x290275e3db66394C52272398959845170E4DCb88").unwrap(),
//         H160::from_str("0xD5C08681719445A5Fdce2Bda98b341A49050d821").unwrap(),
//         H160::from_str("0x082489A616aB4D46d1947eE3F912e080815b08DA").unwrap()
//     ];

//     let conn = Connection::open("pol_indexer.db")?;
//     conn.execute_batch(
//         "
//         CREATE TABLE IF NOT EXISTS transfers (
//             id INTEGER PRIMARY KEY AUTOINCREMENT,
//             tx_hash TEXT,
//             from_address TEXT,
//             to_address TEXT,
//             amount TEXT,
//             direction TEXT,
//             block_number INTEGER,
//             timestamp INTEGER
//         );

//         CREATE TABLE IF NOT EXISTS netflow (
//             id INTEGER PRIMARY KEY AUTOINCREMENT,
//             cumulative_value TEXT,
//             updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
//         );
//         ",
//     )?;
//     println!("✅ Database initialized");

//     // Ensure netflow row exists
//     conn.execute(
//         "INSERT INTO netflow (cumulative_value) 
//          SELECT '0' WHERE NOT EXISTS (SELECT 1 FROM netflow)",
//         [],
//     )?;

//     // Create an event builder first
// //     let transfer_event = pol_contract
// //         .event::<TransferFilter>()
// //         .from_block(BlockNumber::Latest);
// //         // .address(pol_address); 
// //     // Then subscribe
// //     let mut transfer_stream = transfer_event.subscribe_with_meta().await?;


// // //     let mut stream = provider.subscribe_blocks().await?;
// // //     while let Some(block) = stream.next().await {
// // //         if let Some(number) = block.number {
// // //             println!("🔹 New Block: #{} (hash: {:?})", number, block.hash);

// // //             // Fetch full block with transactions
// // //             if let Some(full_block) = provider.get_block_with_txs(number).await? {
// // //                 println!("   Block contains {} transactions", full_block.transactions.len());

// // //                 for tx in full_block.transactions {
// // //                     let from = tx.from;
// // //                     let to = tx.to.unwrap_or_default();

// // //                     // Print every transaction
// // //                     println!(
// // //                         "   Tx hash={:?}, from={:?}, to={:?}, value={}",
// // //                         tx.hash, from, tx.to, tx.value
// // //                     );

// // //                     // Check Binance addresses
// // //                     if binance_addresses.contains(&from) {
// // //                         println!("   🚨 Outflow from Binance: {} wei", tx.value);
// // //                     }
// // //                     if binance_addresses.contains(&to) {
// // //                         println!("   ✅ Inflow to Binance: {} wei", tx.value);
// // //                     }
// // //                 }
// // //             }
// // //         }
// // //     }

// // //     Ok(())
// // // }
// //     while let Some(event) = transfer_stream.next().await {
// //     match event {
// //         Ok((transfer, meta)) => {
// //             // ✅ transfer has: from, to, value
// //             let from = transfer.from;
// //             let to = transfer.to;
// //             let value = transfer.value; // U256

// //             // ✅ meta has: transaction_hash, block_number, etc.
// //             let tx_hash = meta.transaction_hash;
// //             let block_number = meta.block_number.as_u64();
// //             let value_i128: i128 = value.as_u128() as i128;
// //             let timestamp = chrono::Utc::now().timestamp();

// //             let mut direction = None;
// //             let mut delta: i128 = 0;

// //             if binance_addresses.contains(&to) {
// //                 direction = Some("inflow");
// //                 delta = value_i128;
// //             } else if binance_addresses.contains(&from) {
// //                 direction = Some("outflow");
// //                 delta = -value_i128;
// //             }

// //             if let Some(dir) = direction {
// //                 println!("📦 {:?}: {} POL (tx {:?})", dir, value, tx_hash);

// //                 // Insert into transfers table
// //                 conn.execute(
// //                     "INSERT INTO transfers 
// //                      (tx_hash, from_address, to_address, amount, direction, block_number, timestamp)
// //                      VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
// //                     params![
// //                         format!("{:?}", tx_hash),
// //                         format!("{:?}", from),
// //                         format!("{:?}", to),
// //                         value.to_string(),
// //                         dir,
// //                         block_number,
// //                         timestamp,
// //                     ],
// //                 )?;

// //                 // Read current netflow (last row)
// //                 let current: i128 = conn
// //                     .query_row(
// //                         "SELECT cumulative_value FROM netflow ORDER BY id DESC LIMIT 1",
// //                         [],
// //                         |row| {
// //                             let s: String = row.get(0)?;
// //                             Ok(s.parse::<i128>().unwrap_or(0))
// //                         },
// //                     )
// //                     .unwrap_or(0);

// //                 // Update with new delta
// //                 let new_value = current + delta;
// //                 conn.execute(
// //                     "INSERT INTO netflow (cumulative_value) VALUES (?1)",
// //                     params![new_value.to_string()],
// //                 )?;

// //                 println!("📊 Updated cumulative netflow: {}", new_value);
// //             }
// //         }
// //         Err(e) => println!("⚠️ Error parsing event: {:?}", e),
// //     }
// // }

// // Ok(())
// // }
//     let transfer_sig = "Transfer(address,address,uint256)";
//     let topic0 = H256::from(keccak256(transfer_sig.as_bytes()));

//     let filter = Filter::new()
//         .address(pol_address)
//         .event(&topic0);

//     let mut stream = provider.subscribe_logs(&filter).await?;

//     // --- Process events ---
// //     while let Some(log) = stream.next().await {
// //         match log {
// //             Ok(l) => {
// //                 let tx_hash = l.transaction_hash.unwrap_or_default();
// //                 let block_number = l.block_number.unwrap_or_default().as_u64();
// //                 let timestamp = chrono::Utc::now().timestamp();

// //                 // Decode ERC20 Transfer data
// //                 if let Ok(decoded) = abi_decode_transfer(&l) {
// //                     let from = decoded.0;
// //                     let to = decoded.1;
// //                     let value = decoded.2;

// //                     let mut direction = None;
// //                     let mut delta: i128 = 0;

// //                     if binance_addresses.contains(&to) {
// //                         direction = Some("inflow");
// //                         delta = value.as_u128() as i128;
// //                     } else if binance_addresses.contains(&from) {
// //                         direction = Some("outflow");
// //                         delta = -(value.as_u128() as i128);
// //                     }

// //                     if let Some(dir) = direction {
// //                         println!(
// //                             "📦 {:?}: {} POL (tx {:?})",
// //                             dir, value, tx_hash
// //                         );

// //                         // Insert into DB
// //                         conn.execute(
// //                             "INSERT INTO transfers 
// //                              (tx_hash, from_address, to_address, amount, direction, block_number, timestamp)
// //                              VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
// //                             params![
// //                                 format!("{:?}", tx_hash),
// //                                 format!("{:?}", from),
// //                                 format!("{:?}", to),
// //                                 value.to_string(),
// //                                 dir,
// //                                 block_number as i64,
// //                                 timestamp,
// //                             ],
// //                         )?;

// //                         // Update netflow
// //                         let current: i128 = conn
// //                             .query_row(
// //                                 "SELECT cumulative_value FROM netflow ORDER BY id DESC LIMIT 1",
// //                                 [],
// //                                 |row| {
// //                                     let s: String = row.get(0)?;
// //                                     Ok(s.parse::<i128>().unwrap_or(0))
// //                                 },
// //                             )
// //                             .unwrap_or(0);

// //                         let new_value = current + delta;
// //                         conn.execute(
// //                             "INSERT INTO netflow (cumulative_value) VALUES (?1)",
// //                             params![new_value.to_string()],
// //                         )?;

// //                         println!("📊 Updated cumulative netflow: {}", new_value);
// //                     }
// //                 } else {
// //                     println!("⚠️ Failed to decode log {:?}", tx_hash);
// //                 }
// //             }
// //             Err(e) => println!("⚠️ Error parsing log: {:?}", e),
// //         }
// //     }

// //     Ok(())
// // }

// // /// Decode Transfer(address,address,uint256) log
// // fn abi_decode_transfer(log: &Log) -> Result<(H160, H160, U256), abi::Error> {
// //     // Topics[1] = from, Topics[2] = to, Data = value
// //     let from = H160::from_slice(&log.topics[1][12..]);
// //     let to = H160::from_slice(&log.topics[2][12..]);
// //     let value = U256::from_big_endian(&log.data.0);

// //     Ok((from, to, value))
// // }
//     while let Some(l) = stream.next().await {
//     let tx_hash = l.transaction_hash.unwrap_or_default();
//     let block_number = l.block_number.unwrap_or_default().as_u64();
//     let timestamp = chrono::Utc::now().timestamp();

//     if let Ok((from, to, value)) = abi_decode_transfer(&l) {
//         let mut direction = None;
//         let mut delta: i128 = 0;

//         if binance_addresses.contains(&to) {
//             direction = Some("inflow");
//             delta = value.as_u128() as i128;
//         } else if binance_addresses.contains(&from) {
//             direction = Some("outflow");
//             delta = -(value.as_u128() as i128);
//         }

//         if let Some(dir) = direction {
//             println!("📦 {:?}: {} POL (tx {:?})", dir, value, tx_hash);

//             // Insert transfer
//             conn.execute(
//                 "INSERT INTO transfers 
//                  (tx_hash, from_address, to_address, amount, direction, block_number, timestamp)
//                  VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
//                 params![
//                     format!("{:?}", tx_hash),
//                     format!("{:?}", from),
//                     format!("{:?}", to),
//                     value.to_string(),
//                     dir,
//                     block_number as i64,
//                     timestamp,
//                 ],
//             )?;

//             // Update netflow
//             let current: i128 = conn
//                 .query_row(
//                     "SELECT cumulative_value FROM netflow ORDER BY id DESC LIMIT 1",
//                     [],
//                     |row| {
//                         let s: String = row.get(0)?;
//                         Ok(s.parse::<i128>().unwrap_or(0))
//                     },
//                 )
//                 .unwrap_or(0);

//             let new_value = current + delta;
//             conn.execute(
//                 "INSERT INTO netflow (cumulative_value) VALUES (?1)",
//                 params![new_value.to_string()],
//             )?;

//             println!("📊 Updated cumulative netflow: {}", new_value);
//         }
//     } else {
//         println!("⚠️ Failed to decode log {:?}", tx_hash);
//     }
// }

// /// Decode Transfer(address,address,uint256) log
// fn abi_decode_transfer(log: &Log) -> Result<(H160, H160, U256), abi::Error> {
//     // Topics[1] = from, Topics[2] = to, Data = value
//     let from = H160::from_slice(&log.topics[1][12..]);
//     let to = H160::from_slice(&log.topics[2][12..]);
//     let value = U256::from_big_endian(&log.data.0);

//     Ok((from, to, value))
// }

// use ethers::prelude::*;
// use ethers::types::{Filter, H160, H256, U256};
// use ethers::utils::keccak256;
// use rusqlite::{params, Connection};
// use std::sync::Arc;
// use anyhow::Result;
// use tokio_stream::StreamExt;

// #[tokio::main]
// async fn main() -> Result<()> {
//     dotenv::dotenv().ok();

//     let args: Vec<String> = std::env::args().collect();

//     // If user runs `cargo run -- query`, just print netflow
//     if args.len() > 1 && args[1] == "query" {
//         let conn = Connection::open("pol_indexer.db")?;
//         let latest: i128 = conn
//             .query_row(
//                 "SELECT cumulative_value FROM netflow ORDER BY id DESC LIMIT 1",
//                 [],
//                 |row| {
//                     let s: String = row.get(0)?;
//                     Ok(s.parse::<i128>().unwrap_or(0))
//                 },
//             )
//             .unwrap_or(0);
//         println!("📊 Current cumulative netflow: {}", latest);
//         return Ok(());
//     }

//     // Load WS endpoint
//     let ws_url = std::env::var("POLYGON_RPC_WS")
//         .expect("Missing POLYGON_RPC_WS in .env");

//     // Binance hot wallets you want to track
//     // let binance_addresses: Vec<H160> = vec![
//     //     "0x3f5CE5FBFe3E9af3971dD833D26BA9b5C936f0bE".parse().unwrap(),
//     //     "0x564286362092D8e7936f0549571a803B203aAceD".parse().unwrap(),
//     // ];
//     let binance_addresses: Vec<H160> = vec![
//         "0xF977814e90dA44bFA03b6295A0616a897441aceC".parse().unwrap(),
//         "0xe7804c37c13166fF0b37F5aE0BB07A3aEbb6e245".parse().unwrap(),

//         "0x505e71695E9bc45943c58adEC1650577BcA68fD9".parse().unwrap(),

//         "0x290275e3db66394C52272398959845170E4DCb88".parse().unwrap(),
//         "0xD5C08681719445A5Fdce2Bda98b341A49050d821".parse().unwrap(),
//         "0x082489A616aB4D46d1947eE3F912e080815b08DA".parse().unwrap()

//         // H160::from_str("0xF977814e90dA44bFA03b6295A0616a897441aceC").unwrap(),
//         // H160::from_str("0xe7804c37c13166fF0b37F5aE0BB07A3aEbb6e245").unwrap(),
//         // H160::from_str("0x505e71695E9bc45943c58adEC1650577BcA68fD9").unwrap(),
//         // H160::from_str("0x290275e3db66394C52272398959845170E4DCb88").unwrap(),
//         // H160::from_str("0xD5C08681719445A5Fdce2Bda98b341A49050d821").unwrap(),
//         // H160::from_str("0x082489A616aB4D46d1947eE3F912e080815b08DA").unwrap()
//     ];

//     // POL token contract (Polygon PoS POL ERC20 or native wrapper)
//     let pol_address: H160 = "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174"
//         .parse()
//         .unwrap(); // ⚠ replace with actual POL ERC20 contract if needed

//     // Connect provider
//     let ws = Ws::connect(ws_url).await?;
//     let provider = Arc::new(Provider::new(ws));

//     println!("✅ Connected! Listening for POL transfers...");

//     // --- DB setup ---
//     let conn = Connection::open("pol_indexer.db")?;
//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS transfers (
//             id INTEGER PRIMARY KEY AUTOINCREMENT,
//             tx_hash TEXT,
//             from_address TEXT,
//             to_address TEXT,
//             amount TEXT,
//             direction TEXT,
//             block_number INTEGER,
//             timestamp INTEGER
//         )",
//         [],
//     )?;
//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS netflow (
//             id INTEGER PRIMARY KEY AUTOINCREMENT,
//             cumulative_value TEXT,
//             timestamp INTEGER
//         )",
//         [],
//     )?;
//     println!("✅ Database initialized");

//     // --- Event filter for Transfer(address,address,uint256) ---
//     // let transfer_sig = "Transfer(address,address,uint256)";
//     // let topic0 = H256::from(keccak256(transfer_sig.as_bytes()));

//     // let filter = Filter::new()
//     //     .address(pol_address)
//     //     .event(&topic0);

//     // let mut stream = provider.subscribe_logs(&filter).await?;
//     // --- Event filter for Transfer(address,address,uint256) ---
//     let transfer_sig = "Transfer(address,address,uint256)";
//     let topic0 = H256::from(keccak256(transfer_sig.as_bytes()));

//     let filter = Filter::new()
//         .address(pol_address)
//         .topic0(topic0);

//     let mut stream = provider.subscribe_logs(&filter).await?;

//     // --- Process events ---
//     while let Some(l) = stream.next().await {
//         let tx_hash = l.transaction_hash.unwrap_or_default();
//         let block_number = l.block_number.unwrap_or_default().as_u64();
//         let timestamp = chrono::Utc::now().timestamp();

//         if let Ok((from, to, value)) = abi_decode_transfer(&l) {
//             let mut direction = None;
//             let mut delta: i128 = 0;

//             if binance_addresses.contains(&to) {
//                 direction = Some("inflow");
//                 delta = value.as_u128() as i128;
//             } else if binance_addresses.contains(&from) {
//                 direction = Some("outflow");
//                 delta = -(value.as_u128() as i128);
//             }

//             if let Some(dir) = direction {
//                 println!("📦 {:?}: {} POL (tx {:?})", dir, value, tx_hash);

//                 // Insert transfer
//                 conn.execute(
//                     "INSERT INTO transfers 
//                      (tx_hash, from_address, to_address, amount, direction, block_number, timestamp)
//                      VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
//                     params![
//                         format!("{:?}", tx_hash),
//                         format!("{:?}", from),
//                         format!("{:?}", to),
//                         value.to_string(),
//                         dir,
//                         block_number as i64,
//                         timestamp,
//                     ],
//                 )?;

//                 // Update netflow
//                 let current: i128 = conn
//                     .query_row(
//                         "SELECT cumulative_value FROM netflow ORDER BY id DESC LIMIT 1",
//                         [],
//                         |row| {
//                             let s: String = row.get(0)?;
//                             Ok(s.parse::<i128>().unwrap_or(0))
//                         },
//                     )
//                     .unwrap_or(0);

//                 let new_value = current + delta;
//                 conn.execute(
//                     "INSERT INTO netflow (cumulative_value, timestamp) VALUES (?1, ?2)",
//                     params![new_value.to_string(), timestamp],
//                 )?;

//                 println!("📊 Updated cumulative netflow: {}", new_value);
//             }
//         } else {
//             println!("⚠️ Failed to decode log {:?}", tx_hash);
//         }
//     }

//     Ok(())
// }

// /// Decode Transfer(address,address,uint256) log
// fn abi_decode_transfer(log: &Log) -> Result<(H160, H160, U256), abi::Error> {
//     // Topics[1] = from, Topics[2] = to, Data = value
//     let from = H160::from_slice(&log.topics[1][12..]);
//     let to = H160::from_slice(&log.topics[2][12..]);
//     let value = U256::from_big_endian(&log.data.0);

//     Ok((from, to, value))
// }

// use ethers::prelude::*;
// use ethers::types::{Filter, H160, H256, U256};
// use ethers::utils::keccak256;
// use rusqlite::{params, Connection};
// use std::sync::Arc;
// use anyhow::Result;
// use tokio_stream::StreamExt;
// use axum::{Router, routing::get};
// use std::net::SocketAddr;
// use ethers::providers::Ws;
// use tokio::net::TcpListener;


// #[tokio::main]
// async fn main() -> Result<()> {
//     dotenv::dotenv().ok();

//     let args: Vec<String> = std::env::args().collect();

//     // --- CLI query mode ---
//     if args.len() > 1 && args[1] == "query" {
//         let conn = Connection::open("pol_indexer.db")?;
//         let latest: i128 = conn
//             .query_row(
//                 "SELECT cumulative_value FROM netflow ORDER BY id DESC LIMIT 1",
//                 [],
//                 |row| {
//                     let s: String = row.get(0)?;
//                     Ok(s.parse::<i128>().unwrap_or(0))
//                 },
//             )
//             .unwrap_or(0);
//         println!("📊 Current cumulative netflow: {}", latest);
//         return Ok(());
//     }

//     // --- Load WS endpoint ---
//     let ws_url = std::env::var("POLYGON_RPC_WS")
//         .expect("Missing POLYGON_RPC_WS in .env");

//     // Binance hot wallets
//     let binance_addresses: Vec<H160> = vec![
//         "0xF977814e90dA44bFA03b6295A0616a897441aceC".parse().unwrap(),
//         "0xe7804c37c13166fF0b37F5aE0BB07A3aEbb6e245".parse().unwrap(),

//         "0x505e71695E9bc45943c58adEC1650577BcA68fD9".parse().unwrap(),

//         "0x290275e3db66394C52272398959845170E4DCb88".parse().unwrap(),
//         "0xD5C08681719445A5Fdce2Bda98b341A49050d821".parse().unwrap(),
//         "0x082489A616aB4D46d1947eE3F912e080815b08DA".parse().unwrap()

//     ];

//     // POL token contract (replace if needed)
//     let pol_address: H160 = "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174"
//         .parse()
//         .unwrap();

//     // Connect provider
//     let ws = Ws::connect(ws_url).await?;
//     let provider = Arc::new(Provider::new(ws));

//     println!("✅ Connected! Listening for POL transfers...");

//     // --- DB setup ---
//     let conn = Arc::new(Connection::open("pol_indexer.db")?);
//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS transfers (
//             id INTEGER PRIMARY KEY AUTOINCREMENT,
//             tx_hash TEXT,
//             from_address TEXT,
//             to_address TEXT,
//             amount TEXT,
//             direction TEXT,
//             block_number INTEGER,
//             timestamp INTEGER
//         )",
//         [],
//     )?;
//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS netflow (
//             id INTEGER PRIMARY KEY AUTOINCREMENT,
//             cumulative_value TEXT,
//             timestamp INTEGER
//         )",
//         [],
//     )?;
//     println!("✅ Database initialized");

//     // --- REST API ---
//     let api_conn = conn.clone();
//     tokio::spawn(async move {
//         start_rest_api(api_conn).await;
//     });

//     // --- Subscribe to logs ---
//     let transfer_sig = "Transfer(address,address,uint256)";
//     let topic0 = H256::from(keccak256(transfer_sig.as_bytes()));

//     let filter = Filter::new()
//         .address(pol_address)
//         .topic0(topic0);

//     let mut stream = provider.subscribe_logs(&filter).await?;

//     while let Some(l) = stream.next().await {
//         let tx_hash = l.transaction_hash.unwrap_or_default();
//         let block_number = l.block_number.unwrap_or_default().as_u64();
//         let timestamp = chrono::Utc::now().timestamp();

//         if let Ok((from, to, value)) = abi_decode_transfer(&l) {
//             let mut direction = None;
//             let mut delta: i128 = 0;

//             if binance_addresses.contains(&to) {
//                 direction = Some("inflow");
//                 delta = value.as_u128() as i128;
//             } else if binance_addresses.contains(&from) {
//                 direction = Some("outflow");
//                 delta = -(value.as_u128() as i128);
//             }

//             if let Some(dir) = direction {
//                 println!("📦 {:?}: {} POL (tx {:?})", dir, value, tx_hash);

//                 // Insert transfer
//                 conn.execute(
//                     "INSERT INTO transfers 
//                      (tx_hash, from_address, to_address, amount, direction, block_number, timestamp)
//                      VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
//                     params![
//                         format!("{:?}", tx_hash),
//                         format!("{:?}", from),
//                         format!("{:?}", to),
//                         value.to_string(),
//                         dir,
//                         block_number as i64,
//                         timestamp,
//                     ],
//                 )?;

//                 // Update netflow
//                 let current: i128 = conn
//                     .query_row(
//                         "SELECT cumulative_value FROM netflow ORDER BY id DESC LIMIT 1",
//                         [],
//                         |row| {
//                             let s: String = row.get(0)?;
//                             Ok(s.parse::<i128>().unwrap_or(0))
//                         },
//                     )
//                     .unwrap_or(0);

//                 let new_value = current + delta;
//                 conn.execute(
//                     "INSERT INTO netflow (cumulative_value, timestamp) VALUES (?1, ?2)",
//                     params![new_value.to_string(), timestamp],
//                 )?;

//                 println!("📊 Updated cumulative netflow: {}", new_value);
//             }
//         } else {
//             println!("⚠️ Failed to decode log {:?}", tx_hash);
//         }
//     }

//     Ok(())
// }

// /// Decode Transfer(address,address,uint256) log
// fn abi_decode_transfer(log: &Log) -> Result<(H160, H160, U256), abi::Error> {
//     let from = H160::from_slice(&log.topics[1][12..]);
//     let to = H160::from_slice(&log.topics[2][12..]);
//     let value = U256::from_big_endian(&log.data.0);

//     Ok((from, to, value))
// }

// /// REST API server
// async fn start_rest_api(conn: Arc<Connection>) {
//     let app = Router::new().route(
//         "/netflow",
//         get({
//             let conn = conn.clone();
//             move || async move {
//                 let current: i128 = conn
//                     .query_row(
//                         "SELECT cumulative_value FROM netflow ORDER BY id DESC LIMIT 1",
//                         [],
//                         |row| {
//                             let s: String = row.get(0)?;
//                             Ok(s.parse::<i128>().unwrap_or(0))
//                         },
//                     )
//                     .unwrap_or(0);

//                 format!("{{\"cumulative_netflow\": {}}}", current)
//             }
//         }),
//     );

//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     println!("🚀 REST API available at http://{}", addr);

//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }


// use axum::{routing::get, Json, Router};
// use ethers::providers::{Provider, Ws};
use ethers::providers::{Provider, Ws, Middleware};
use ethers::types::{Filter, H160, H256, U256, Log};
use ethers::utils::keccak256;
use rusqlite::{params, Connection};
use std::net::SocketAddr;
use std::sync::Arc;
use anyhow::Result;
use tokio_stream::StreamExt;
use serde::Serialize;
use tracing::{info, warn, error};
use axum::{routing::get, response::Html, Json, Router};



#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let args: Vec<String> = std::env::args().collect();

    // CLI query mode
    if args.len() > 1 && args[1] == "query" {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "pol_indexer.db".to_string());
        let conn = Connection::open(db_url)?;
        let latest: i128 = conn
            .query_row(
                "SELECT cumulative_value FROM netflow ORDER BY id DESC LIMIT 1",
                [],
                |row| {
                    let s: String = row.get(0)?;
                    Ok(s.parse::<i128>().unwrap_or(0))
                },
            )
            .unwrap_or(0);
        println!("📊 Current cumulative netflow: {}", latest);
        return Ok(());
    }

    // --- Start indexer task ---
    tokio::spawn(async move {
        if let Err(e) = run_indexer().await {
            error!("❌ Indexer error: {:?}", e);
        }
    });

    // --- Start REST API server ---
    let app = Router::new()
    .route("/", get(index_page))
    .route("/netflow", get(get_netflow));

    let port: u16 = std::env::var("API_PORT")
        .unwrap_or("3000".to_string())
        .parse()
        .unwrap_or(3000);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("🌐 REST API running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}
use std::env;

fn load_addresses() -> (H160, Vec<H160>) {
    // POL contract
    let pol_address: H160 = env::var("POL_CONTRACT")
        .expect("Missing POL_CONTRACT in .env")
        .parse()
        .expect("Invalid POL_CONTRACT address");

    // Binance addresses
    let binance_env = env::var("BINANCE_ADDRESSES")
        .expect("Missing BINANCE_ADDRESSES in .env");

    let binance_addresses: Vec<H160> = binance_env
        .split(',')
        .map(|addr| addr.trim().parse().expect("Invalid Binance address"))
        .collect();

    (pol_address, binance_addresses)
}

async fn index_page() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>POL Indexer</title>
        </head>
        <body style="font-family: Arial; text-align: center; margin-top: 100px;">
            <h1>✅ POL Indexer API</h1>
            <p>Click below to view the current netflow data:</p>
            <button onclick="window.location.href='/netflow'" 
                    style="padding: 10px 20px; font-size: 16px; cursor: pointer;">
                View Netflow
            </button>
        </body>
        </html>
    "#)
}
/// REST API handler
async fn get_netflow() -> Json<NetflowResponse> {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "pol_indexer.db".to_string());
    let conn = Connection::open(db_url).unwrap();
    let latest: i128 = conn
        .query_row(
            "SELECT cumulative_value FROM netflow ORDER BY id DESC LIMIT 1",
            [],
            |row| {
                let s: String = row.get(0)?;
                Ok(s.parse::<i128>().unwrap_or(0))
            },
        )
        .unwrap_or(0);

    Json(NetflowResponse { netflow: latest })
}

#[derive(Serialize)]
struct NetflowResponse {
    netflow: i128,
}

/// Indexer loop
async fn run_indexer() -> Result<()> {
    let ws_url = std::env::var("POLYGON_RPC_WS")
        .expect("Missing POLYGON_RPC_WS in .env");

    // let binance_addresses: Vec<H160> = vec![
    //     "0xF977814e90dA44bFA03b6295A0616a897441aceC".parse().unwrap(),
    //     "0xe7804c37c13166fF0b37F5aE0BB07A3aEbb6e245".parse().unwrap(),

    //     "0x505e71695E9bc45943c58adEC1650577BcA68fD9".parse().unwrap(),

    //     "0x290275e3db66394C52272398959845170E4DCb88".parse().unwrap(),
    //     "0xD5C08681719445A5Fdce2Bda98b341A49050d821".parse().unwrap(),
    //     "0x082489A616aB4D46d1947eE3F912e080815b08DA".parse().unwrap()
    // ];

    // let pol_address: H160 = "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174"
    //     .parse()
    //     .unwrap();
    let (pol_address, binance_addresses) = load_addresses();

    // Connect provider
    let ws = Ws::connect(ws_url).await?;
    let provider = Arc::new(Provider::new(ws));

    info!("✅ Connected! Listening for POL transfers...");

    // DB setup
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "pol_indexer.db".to_string());

    let conn = Connection::open(db_url)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS transfers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tx_hash TEXT,
            from_address TEXT,
            to_address TEXT,
            amount TEXT,
            direction TEXT,
            block_number INTEGER,
            timestamp INTEGER
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS netflow (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            cumulative_value TEXT,
            timestamp INTEGER
        )",
        [],
    )?;
    info!("✅ Database initialized");

    // Set up filter
    let transfer_sig = "Transfer(address,address,uint256)";
    let topic0 = H256::from(keccak256(transfer_sig.as_bytes()));

    let filter = Filter::new().address(pol_address).topic0(topic0);

    let mut stream = provider.subscribe_logs(&filter).await?;

    // Process logs
    while let Some(l) = stream.next().await {
        let tx_hash = l.transaction_hash.unwrap_or_default();
        let block_number = l.block_number.unwrap_or_default().as_u64();
        let timestamp = chrono::Utc::now().timestamp();

        if let Ok((from, to, value)) = abi_decode_transfer(&l) {
            let mut direction = None;
            let mut delta: i128 = 0;

            if binance_addresses.contains(&to) {
                direction = Some("inflow");
                delta = value.as_u128() as i128;
            } else if binance_addresses.contains(&from) {
                direction = Some("outflow");
                delta = -(value.as_u128() as i128);
            }

            if let Some(dir) = direction {
                info!("📦 {:?}: {} POL (tx {:?})", dir, value, tx_hash);

                conn.execute(
                    "INSERT INTO transfers 
                     (tx_hash, from_address, to_address, amount, direction, block_number, timestamp)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    params![
                        format!("{:?}", tx_hash),
                        format!("{:?}", from),
                        format!("{:?}", to),
                        value.to_string(),
                        dir,
                        block_number as i64,
                        timestamp,
                    ],
                )?;

                let current: i128 = conn
                    .query_row(
                        "SELECT cumulative_value FROM netflow ORDER BY id DESC LIMIT 1",
                        [],
                        |row| {
                            let s: String = row.get(0)?;
                            Ok(s.parse::<i128>().unwrap_or(0))
                        },
                    )
                    .unwrap_or(0);

                let new_value = current + delta;
                conn.execute(
                    "INSERT INTO netflow (cumulative_value, timestamp) VALUES (?1, ?2)",
                    params![new_value.to_string(), timestamp],
                )?;

                info!("📊 Updated cumulative netflow: {}", new_value);
            }
        } else {
            warn!("⚠️ Failed to decode log {:?}", tx_hash);
        }
    }

    Ok(())
}

/// Decode ERC20 Transfer(address,address,uint256) log
fn abi_decode_transfer(log: &Log) -> Result<(H160, H160, U256), ethers::abi::Error> {
    let from = H160::from_slice(&log.topics[1][12..]);
    let to = H160::from_slice(&log.topics[2][12..]);
    let value = U256::from_big_endian(&log.data.0);
    Ok((from, to, value))
}

