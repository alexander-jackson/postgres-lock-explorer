use std::net::{Ipv4Addr, SocketAddrV4};
use std::ops::DerefMut;
use std::sync::Arc;

use axum::extract::{Json, State};
use axum::routing::put;
use axum::{Router, Server};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tokio_postgres::{Client, NoTls};

mod error;

use crate::error::ServerResult;

type SharedClient = Arc<Mutex<(Client, Client)>>;

async fn get_client() -> ServerResult<Client> {
    let (client, conn) =
        tokio_postgres::connect("host=localhost user=alex dbname=testing", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let left = get_client().await?;
    let right = get_client().await?;

    let client = Arc::new(Mutex::new((left, right)));

    let router = Router::new()
        .route("/analyse", put(analyse_locks))
        .with_state(client);

    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 5430).into();

    let server = Server::bind(&addr).serve(router.into_make_service());
    server.await?;

    Ok(())
}

#[derive(Deserialize)]
struct LockAnalysisRequest {
    query: String,
    relation: String,
}

#[derive(Serialize)]
struct LockAnalysisResponse {
    locktype: String,
    mode: String,
}

async fn analyse_locks(
    State(state): State<SharedClient>,
    Json(request): Json<LockAnalysisRequest>,
) -> ServerResult<Json<LockAnalysisResponse>> {
    let mut client = state.lock().await;
    let (ref mut left, ref right) = client.deref_mut();

    // Begin a transaction
    let transaction = left.transaction().await?;
    transaction.query(&request.query, &[]).await?;

    // Use the other connection to inspect the locks
    let lock = right
        .query_one(
            r#"
            SELECT pl.locktype, mode
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

    let response = LockAnalysisResponse {
        locktype: lock.get(0),
        mode: lock.get(1),
    };

    Ok(Json(response))
}
