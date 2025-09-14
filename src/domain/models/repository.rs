#[derive(Debug, Clone)]
pub struct Repository {
    pub owner: String,
    pub name: String,
    pub url: String,
    pub default_branch: String,
}

impl Repository {
    pub fn new(owner: String, name: String, url: String, default_branch: String) -> Self {
        Self {
            owner,
            name,
            url,
            default_branch,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}