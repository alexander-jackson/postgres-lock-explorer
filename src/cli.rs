use std::borrow::Cow;
use std::str::FromStr;

use clap::Parser;
use color_eyre::eyre::{eyre, Context};
use color_eyre::{Report, Result};
use ureq::serde::de::DeserializeOwned;
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
}

pub fn run(args: &Args) -> Result<()> {
    color_eyre::install()?;

    let agent = Agent::new();
    let base = Cow::Borrowed("http://localhost:5430/locks");

    let uri = match args.relation.as_ref() {
        Some(value) => Cow::Owned(format!("{base}/{value}")),
        None => base,
    };

    let response: Vec<LockAnalysisResponse> = make_request(&agent, &uri, &args.query.0)?;

    match response.len() {
        0 => println!("No locks were returned for this query"),
        _ => response.iter().for_each(display_analysis),
    };

    Ok(())
}

fn make_request<T: DeserializeOwned>(agent: &Agent, uri: &str, query: &str) -> Result<T> {
    let payload = LockAnalysisRequest {
        query: query.to_string(),
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
        relation,
    } = analysis;

    println!("Lock of type '{locktype}' with mode '{mode}' will be taken on relation '{relation}'")
}
