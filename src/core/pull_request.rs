pub struct PullRequest {
    pub title: String,
    pub description: String,
    pub source_branch: String,
    pub target_branch: String,
}

impl PullRequest {
    pub fn new(current_branch: String) -> PullRequest {
        PullRequest {
            title: String::new(),
            description: String::new(),
            source_branch: current_branch,
            target_branch: "main".to_string(),
        }
    }
}
