use std::process::Command;
use std::str;

#[derive(Clone)]
pub struct GitClient;

impl Default for GitClient {
    fn default() -> Self {
        Self::new()
    }
}

impl GitClient {
    pub fn new() -> Self {
        Self
    }

    pub fn get_repo_info(&self) -> Option<(String, String)> {
        let output = Command::new("git")
            .arg("config")
            .arg("--get")
            .arg("remote.origin.url")
            .output()
            .ok()?;

        if output.status.success() {
            let url = str::from_utf8(&output.stdout).ok()?.trim();
            self.parse_git_url(url)
        } else {
            None
        }
    }

    pub fn get_current_branch(&self) -> Option<String> {
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output()
            .ok()?;

        if output.status.success() {
            Some(str::from_utf8(&output.stdout).ok()?.trim().to_string())
        } else {
            None
        }
    }

    fn parse_git_url(&self, url: &str) -> Option<(String, String)> {
        if url.starts_with("https://") || url.starts_with("git@") {
            let parts: Vec<&str> = url.rsplitn(2, '/').collect();
            if parts.len() == 2 {
                let repo = parts[0].trim_end_matches(".git");

                let owner = if parts[1].contains("//") {
                    parts[1].rsplitn(2, '/').collect::<Vec<&str>>()[0]
                } else {
                    parts[1].rsplitn(2, ':').collect::<Vec<&str>>()[0]
                };
                return Some((owner.to_string(), repo.to_string()));
            }
        }
        None
    }
}