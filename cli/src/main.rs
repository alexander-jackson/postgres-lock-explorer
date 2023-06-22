use backend_connector::{LockAnalysisRequest, LockAnalysisResponse};
use color_eyre::Result;
use ureq::Agent;

fn main() -> Result<()> {
    color_eyre::install()?;

    let agent = Agent::new();

    let request = LockAnalysisRequest {
        query: String::from("SELECT * FROM example"),
        relation: String::from("example"),
    };

    let response: LockAnalysisResponse = agent
        .put("http://localhost:5430/analyse")
        .send_json(&request)?
        .into_json()?;

    println!("{response:?}");

    Ok(())
}
