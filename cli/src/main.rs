use color_eyre::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use ureq::serde::de::DeserializeOwned;
use ureq::Agent;

use backend_connector::{LockAnalysisRequest, LockAnalysisResponse};

fn main() -> Result<()> {
    color_eyre::install()?;

    let agent = Agent::new();

    let query = get_text("Enter a query")?;
    let query = resolve_query_text(&query)?;

    let with_relation = Confirm::new()
        .with_prompt("Do you want to specify a relation?")
        .interact()?;

    let base = format!("http://localhost:5430/locks");
    let uri = match with_relation {
        true => format!("{base}/{}", get_text("Enter a relation")?),
        false => base,
    };

    let response: Vec<LockAnalysisResponse> = make_request(agent, &uri, &query)?;

    match response.len() {
        0 => println!("No locks were returned for this query"),
        _ => response.iter().for_each(display_analysis),
    };

    Ok(())
}

fn make_request<T: DeserializeOwned>(agent: Agent, uri: &str, query: &str) -> Result<T> {
    let payload = LockAnalysisRequest {
        query: query.to_string(),
    };

    let value = agent.put(uri).send_json(&payload)?.into_json()?;

    Ok(value)
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
        .interact_text()?;

    Ok(value)
}

fn resolve_query_text(input: &str) -> Result<String> {
    if let Some(filename) = input.strip_prefix("@") {
        let content = std::fs::read_to_string(&filename)?;

        return Ok(content);
    }

    return Ok(input.to_string());
}
