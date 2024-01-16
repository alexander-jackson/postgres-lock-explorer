use serde::{Deserialize, Serialize};

use crate::lock::Lock;

#[derive(Debug, Serialize, Deserialize)]
pub struct LockAnalysisRequest {
    pub query: String,
    pub relation: Option<String>,
    pub schema: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockAnalysisResponse {
    pub locktype: String,
    pub mode: Lock,
    pub schema: String,
    pub relation: String,
}
