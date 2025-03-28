pub struct GithubRepository {
    url: String,
    default_branch: String,
}

impl GithubRepository {
    pub fn new() -> GithubRepository {
        GithubRepository {
            url: String::new(),
            default_branch: String::new(),
        }
    }
    pub fn set_url(&mut self, new_url: String) {
        self.url = new_url;
    }

    pub fn get_url(&self) -> &String {
        &self.url
    }

    pub fn set_default_branch(&mut self, branch: String) {
        self.default_branch = branch;
    }
    pub fn get_default_branch(&self) -> &String {
        &self.default_branch
    }
}
