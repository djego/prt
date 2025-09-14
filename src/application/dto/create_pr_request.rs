use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePullRequestRequest {
    pub title: String,
    pub description: String,
    pub source_branch: String,
    pub target_branch: String,
}