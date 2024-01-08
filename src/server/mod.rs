use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;

use axum::routing::put;
use axum::Router;
use clap::Parser;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio_postgres::{Client, Config, NoTls};

mod endpoints;
mod error;

use crate::server::error::ServerResult;

type SharedClient = Arc<Mutex<(Client, Client)>>;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long, help = "Hostname of the database server")]
    host: String,
    #[arg(
        short = 'U',
        long,
        help = "Username for connecting to the database server"
    )]
    user: String,
    #[arg(long, help = "Password for connecting to the database server")]
    password: Option<String>,
    #[arg(short, long, help = "Name of the database to connect to")]
    database: String,
    #[arg(short = 'p', long = "port", help = "Port of the database server")]
    database_port: Option<u16>,
    #[arg(long, help = "Port to run the server itself on")]
    server_port: Option<u16>,
}

async fn get_client(args: &Args) -> ServerResult<Client> {
    let mut config = Config::new();

    config
        .host(&args.host)
        .user(&args.user)
        .password(&args.password.clone().unwrap_or_default())
        .dbname(&args.database)
        .port(args.database_port.unwrap_or(5432));

    let (client, conn) = config.connect(NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn run(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let left = get_client(args).await?;
    let right = get_client(args).await?;

    let client = Arc::new(Mutex::new((left, right)));

    let router = Router::new()
        .route(
            "/locks/:relation",
            put(endpoints::analyse_locks_on_relation),
        )
        .route("/locks", put(endpoints::analyse_all_locks))
        .with_state(client);

    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, args.server_port.unwrap_or(5430));
    let listener = TcpListener::bind(&addr).await?;

    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
