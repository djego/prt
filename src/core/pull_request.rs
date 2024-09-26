pub struct PullRequest {
    pub title: String,
    pub description: String,
    pub source_branch: String,
    pub target_branch: String,
}

impl PullRequest {
    pub fn new() -> PullRequest {
        PullRequest {
            title: String::new(),
            description: String::new(),
            source_branch: String::new(),
            target_branch: std::env::var("GITHUB_DEFAULT_BRANCH")
                .unwrap_or_else(|_| "main".to_string()),
        }
    }
}
