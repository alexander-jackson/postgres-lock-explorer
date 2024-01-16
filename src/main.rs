use clap::Parser;
use color_eyre::eyre::Result;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

mod args;
mod cli;
mod explain;
mod server;
mod types;

use crate::args::{Args, Command};

fn setup() -> Result<()> {
    color_eyre::install()?;

    let fmt_layer = tracing_subscriber::fmt::layer().with_ansi(true);
    let env_layer = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()?;

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(env_layer)
        .init();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    setup()?;

    let args = Args::parse();

    match args.command {
        Command::Query(args) => crate::cli::run(&args)?,
        Command::Serve(args) => crate::server::run(&args).await.unwrap(),
        Command::Explain(args) => crate::explain::run(&args)?,
    };

    Ok(())
}
