use std::borrow::Cow;

use color_eyre::eyre::{eyre, Context};
use color_eyre::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use ureq::serde::de::DeserializeOwned;
use ureq::Agent;

use crate::types::{LockAnalysisRequest, LockAnalysisResponse};

pub fn run() -> Result<()> {
    color_eyre::install()?;

    let agent = Agent::new();

    let query = get_text("Enter a query")?;
    let query = resolve_query_text(&query)?;

    let with_relation = Confirm::new()
        .with_prompt("Do you want to specify a relation?")
        .interact()?;

    let base = Cow::Borrowed("http://localhost:5430/locks");
    let uri = match with_relation {
        true => Cow::Owned(format!("{base}/{}", get_text("Enter a relation")?)),
        false => base,
    };

    let response: Vec<LockAnalysisResponse> = make_request(&agent, &uri, &query)?;

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

fn get_text(prompt: &str) -> Result<String> {
    let theme = ColorfulTheme::default();

    let value = Input::with_theme(&theme)
        .with_prompt(prompt)
        .interact_text()
        .wrap_err("failed to read input from prompt")?;

    Ok(value)
}

fn resolve_query_text(input: &str) -> Result<Cow<'_, str>> {
    if let Some(filename) = input.strip_prefix('@') {
        let content = std::fs::read_to_string(filename)
            .wrap_err_with(|| format!("failed to read query from {filename}"))?;

        return Ok(Cow::Owned(content));
    }

    return Ok(Cow::Borrowed(input));
}
