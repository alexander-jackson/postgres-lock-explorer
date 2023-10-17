use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;

use axum::routing::put;
use axum::{Router, Server};
use tokio::sync::Mutex;
use tokio_postgres::{Client, Config, NoTls};

mod args;
mod endpoints;
mod error;

use crate::args::Args;
use crate::error::ServerResult;

type SharedClient = Arc<Mutex<(Client, Client)>>;

async fn get_client(args: &Args) -> ServerResult<Client> {
    let mut config = Config::new();

    config
        .host(&args.host)
        .user(&args.user)
        .password(&args.password)
        .dbname(&args.database)
        .port(args.database_port);

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
        .route(
            "/locks/:relation",
            put(endpoints::analyse_locks_on_relation),
        )
        .route("/locks", put(endpoints::analyse_all_locks))
        .with_state(client);

    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 5430).into();

    let server = Server::bind(&addr).serve(router.into_make_service());
    server.await?;

    Ok(())
}
