use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LockAnalysisRequest {
    pub query: String,
    pub relation: String,
}

#[derive(Serialize)]
pub struct LockAnalysisResponse {
    pub locktype: String,
    pub mode: String,
}
