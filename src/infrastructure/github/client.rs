use octocrab::Octocrab;
use std::sync::Arc;

#[derive(Clone)]
pub struct GitHubClient {
    client: Arc<Octocrab>,
}

impl GitHubClient {
    pub fn new(token: String) -> Result<Self, octocrab::Error> {
        let client = Octocrab::builder().personal_token(token).build()?;
        Ok(Self {
            client: Arc::new(client),
        })
    }

    pub fn get_client(&self) -> &Octocrab {
        &self.client
    }
}