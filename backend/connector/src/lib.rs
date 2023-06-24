use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LockAnalysisRequest {
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockAnalysisResponse {
    pub locktype: String,
    pub mode: String,
    pub relation: String,
}
