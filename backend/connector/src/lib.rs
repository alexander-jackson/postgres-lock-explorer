use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LockAnalysisRequest {
    pub query: String,
    pub relation: String,
}

impl LockAnalysisRequest {
    pub fn new<Q: Into<String>, R: Into<String>>(query: Q, relation: R) -> Self {
        Self {
            query: query.into(),
            relation: relation.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockAnalysisResponse {
    pub locktype: String,
    pub mode: String,
}
