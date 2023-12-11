use clap::Parser;
use color_eyre::eyre::Result;

mod args;
mod cli;
mod server;
mod types;

use crate::args::{Args, Command};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Query => crate::cli::run()?,
        Command::Serve(args) => crate::server::run(&args).await.unwrap(),
    };

    Ok(())
}
