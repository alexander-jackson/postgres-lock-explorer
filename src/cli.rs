use std::str::FromStr;

use clap::Parser;
use color_eyre::eyre::{eyre, Context};
use color_eyre::{Report, Result};
use ureq::Agent;

use crate::types::{LockAnalysisRequest, LockAnalysisResponse};

#[derive(Clone, Debug)]
struct Query(String);

impl FromStr for Query {
    type Err = Report;

    fn from_str(value: &str) -> Result<Self> {
        let inner = match value.strip_prefix('@') {
            Some(path) => std::fs::read_to_string(path)
                .wrap_err_with(|| format!("failed to read content from {path}"))?,
            None => value.to_owned(),
        };

        Ok(Query(inner))
    }
}

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(
        short = 'i',
        long = "input",
        help = "Query to run against the database, either as a string or a filepath prefixed with '@'"
    )]
    query: Query,
    #[arg(short = 'r', long = "relation", help = "Relation to filter locks for")]
    relation: Option<String>,
    #[arg(short = 's', long = "schema", help = "Schema to filter locks for")]
    schema: Option<String>,
    #[arg(long, help = "Port that the lock analysis server is running on")]
    server_port: Option<u16>,
}

pub fn run(args: Args) -> Result<()> {
    let agent = Agent::new();

    let server_port = args.server_port.unwrap_or(5430);
    let base = format!("http://localhost:{server_port}/locks");

    let response = make_request(&agent, &base, args.query.0, args.schema, args.relation)?;

    match response.len() {
        0 => println!("No locks were returned for this query"),
        _ => response.iter().for_each(display_analysis),
    };

    Ok(())
}

fn make_request(
    agent: &Agent,
    uri: &str,
    query: String,
    schema: Option<String>,
    relation: Option<String>,
) -> Result<Vec<LockAnalysisResponse>> {
    let payload = LockAnalysisRequest {
        query,
        schema,
        relation,
    };

    match agent.put(uri).send_json(payload) {
        Ok(res) => {
            let json = res
                .into_json()
                .wrap_err("failed to convert response body to JSON")?;

            Ok(json)
        }
        Err(ureq::Error::Status(code, response)) => {
            let text = response
                .into_string()
                .wrap_err("failed to convert response body to a string")?;

            Err(eyre!("Invalid request (response code {code}): {text}"))
        }
        Err(e) => Err(e.into()),
    }
}

fn display_analysis(analysis: &LockAnalysisResponse) {
    let LockAnalysisResponse {
        locktype,
        mode,
        schema,
        relation,
    } = analysis;

    println!("Lock of type '{locktype}' with mode '{mode}' will be taken on relation '{schema}.{relation}'")
}
