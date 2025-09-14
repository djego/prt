use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RepositoryInfo {
    pub owner: String,
    pub name: String,
    pub url: String,
    pub default_branch: String,
}