use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LockAnalysisRequest {
    pub query: String,
    pub relation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockAnalysisResponse {
    pub locktype: String,
    pub mode: String,
}
