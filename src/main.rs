
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

