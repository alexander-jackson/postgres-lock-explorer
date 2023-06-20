use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;

use axum::extract::State;
use axum::routing::put;
use axum::{Router, Server};
use tokio::sync::Mutex;
use tokio_postgres::{Client, NoTls};

type SharedClient = Arc<Mutex<Client>>;

#[tokio::main]
async fn main() {
    let (client, connection) = tokio_postgres::connect("host=localhost user=alex", NoTls)
        .await
        .expect("Failed to get connection");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let client = Arc::new(Mutex::new(client));

    let router = Router::new()
        .route("/analyse", put(analyse_locks))
        .with_state(client);

    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 5430).into();

    let server = Server::bind(&addr).serve(router.into_make_service());
    server.await.expect("Failed to run server");
}

async fn analyse_locks(State(state): State<SharedClient>) -> &'static str {
    let mut client = state.lock().await;

    // Begin a transaction
    let transaction = client
        .transaction()
        .await
        .expect("Failed to start a transaction");

    transaction
        .commit()
        .await
        .expect("Failed to commit the transaction");

    "Hello, World!"
}
