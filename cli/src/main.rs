use color_eyre::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use ureq::serde::de::DeserializeOwned;
use ureq::Agent;

use backend_connector::LockAnalysisResponse;

fn main() -> Result<()> {
    color_eyre::install()?;

    let agent = Agent::new();

    let query = get_text("Enter a query")?;

    let with_relation = Confirm::new()
        .with_prompt("Do you want to specify a relation?")
        .interact()?;

    let base = format!("http://localhost:5430/locks");

    if with_relation {
        let relation = get_text("Enter a relation")?;
        let uri = format!("{base}/{relation}");

        let response: Option<LockAnalysisResponse> = make_request(agent, &uri, &query)?;

        match response {
            Some(analysis) => display_analysis(analysis),
            None => println!("No locks will be taken on {relation}"),
        }
    } else {
        let response: Vec<LockAnalysisResponse> = make_request(agent, &base, &query)?;

        for analysis in response {
            display_analysis(analysis);
        }
    }

    Ok(())
}

fn make_request<T: DeserializeOwned>(agent: Agent, uri: &str, query: &str) -> Result<T> {
    let value = agent.get(uri).query("query", query).call()?.into_json()?;

    Ok(value)
}

fn display_analysis(analysis: LockAnalysisResponse) {
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
