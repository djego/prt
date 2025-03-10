use std::process::Command;
use std::str;

pub fn get_repo_info() -> Option<(String, String)> {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("remote.origin.url")
        .output()
        .expect("Failed to execute git command");

    if output.status.success() {
        let url = str::from_utf8(&output.stdout).unwrap().trim();
        if let Some((owner, repo)) = parse_git_url(url) {
            return Some((owner.to_string(), repo.to_string()));
        }
    }

    None
}

fn parse_git_url(url: &str) -> Option<(&str, &str)> {
    if url.starts_with("https://") || url.starts_with("git@") {
        let parts: Vec<&str> = url.rsplitn(2, '/').collect();
        if parts.len() == 2 {
            let repo = parts[0].trim_end_matches(".git");

            let owner = if parts[1].contains("//") {
                parts[1].rsplitn(2, '/').collect::<Vec<&str>>()[0]
            } else {
                parts[1].rsplitn(2, ':').collect::<Vec<&str>>()[0]
            };
            return Some((owner, repo));
        }
    }
    None
}

pub fn get_current_branch() -> Option<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("Failed to execute git command");

    if output.status.success() {
        let branch = str::from_utf8(&output.stdout).unwrap().trim();
        return Some(branch.to_string());
    }

    None
}
