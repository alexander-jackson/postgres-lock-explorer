use std::net::{Ipv4Addr, SocketAddrV4};
use std::ops::DerefMut;
use std::sync::Arc;

use axum::extract::{Json, State};
use axum::routing::put;
use axum::{Router, Server};
use backend_connector::{LockAnalysisRequest, LockAnalysisResponse};
use tokio::sync::Mutex;
use tokio_postgres::{Client, Config, NoTls};

mod args;
mod error;

use crate::args::Args;
use crate::error::ServerResult;

type SharedClient = Arc<Mutex<(Client, Client)>>;

async fn get_client(args: &Args) -> ServerResult<Client> {
    let mut config = Config::new();

    config
        .host(&args.host)
        .user(&args.user)
        .dbname(&args.database);

    let (client, conn) = config.connect(NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_env()?;

    let left = get_client(&args).await?;
    let right = get_client(&args).await?;

    let client = Arc::new(Mutex::new((left, right)));

    let router = Router::new()
        .route("/analyse", put(analyse_locks))
        .with_state(client);

    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 5430).into();

    let server = Server::bind(&addr).serve(router.into_make_service());
    server.await?;

    Ok(())
}

async fn analyse_locks(
    State(state): State<SharedClient>,
    Json(request): Json<LockAnalysisRequest>,
) -> ServerResult<Json<Option<LockAnalysisResponse>>> {
    let mut client = state.lock().await;
    let (ref mut left, ref right) = client.deref_mut();

    // Begin a transaction
    let transaction = left.transaction().await?;
    transaction.query(&request.query, &[]).await?;

    // Use the other connection to inspect the locks
    let lock = right
        .query_opt(
            r#"
            SELECT pl.locktype, pl.mode
            FROM pg_locks pl
            JOIN pg_stat_activity psa ON pl.pid = psa.pid
            JOIN pg_class pc ON pc.oid = pl.relation
            WHERE psa.query = $1
            AND pc.relname = $2
        "#,
            &[&request.query, &request.relation],
        )
        .await?;

    transaction.rollback().await?;

    let response = lock.map(|row| LockAnalysisResponse {
        locktype: row.get(0),
        mode: row.get(1),
    });

    Ok(Json(response))
}
