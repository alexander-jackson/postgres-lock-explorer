use backend_connector::{LockAnalysisRequest, LockAnalysisResponse};
use color_eyre::Result;
use dialoguer::{theme::ColorfulTheme, Input};
use ureq::Agent;

fn main() -> Result<()> {
    color_eyre::install()?;

    let agent = Agent::new();

    let query = get_text("Enter a query")?;
    let relation = get_text("Enter a relation")?;

    let request = LockAnalysisRequest::new(&query, &relation);

    let response: Option<LockAnalysisResponse> = agent
        .put("http://localhost:5430/analyse")
        .send_json(&request)?
        .into_json()?;

    match response {
        Some(LockAnalysisResponse { locktype, mode }) => {
            println!("Lock of type '{locktype}' with mode '{mode}' will be taken on relation '{relation}'")
        }
        None => println!("No lock will be taken on relation '{relation}' by this query"),
    }

    Ok(())
}

fn get_text(prompt: &str) -> Result<String> {
    let theme = ColorfulTheme::default();

    let value = Input::with_theme(&theme)
        .with_prompt(prompt)
        .interact_text()?;

    Ok(value)
}
