#[cfg(test)]
mod tests {
    use crate::core::errors::PullRequestError;
    use octocrab::GitHubError;
    use reqwest::Error as ReqwestError;
    use std::backtrace::Backtrace;

    #[test]
    fn test_api_error_display() {
        // Test with source
        let github_error = GitHubError {
            message: "API call failed".to_string(),
            documentation_url: Some("http://doc.url".to_string()),
            errors: None, // Simplified for test
        };
        let error = PullRequestError::ApiError {
            message: "API call failed".to_string(),
            source: Some(github_error.clone()), // clone it as it's moved if not
        };
        // The debug format of GitHubError includes the struct name, so we match that.
        // Note: The exact formatting of Some(GitHubError {...}) can be sensitive to Rust versions or changes in Debug impl.
        // If this test becomes brittle, a more robust check might involve parsing or checking substrings.
        let expected_string = format!(
            "GitHub API error: API call failed (Source: Some(GitHubError {{ message: \"API call failed\", documentation_url: Some(\"http://doc.url\"), errors: None }}))"
        );
        assert_eq!(error.to_string(), expected_string);

        // Test without source
        let error_no_source = PullRequestError::ApiError {
            message: "Another API error".to_string(),
            source: None,
        };
        assert_eq!(
            error_no_source.to_string(),
            "GitHub API error: Another API error (Source: None)"
        );
    }

    #[test]
    fn test_pull_request_validation_failed_display() {
        let error =
            PullRequestError::PullRequestValidationFailed("Branch not selected".to_string());
        assert_eq!(
            error.to_string(),
            "Validation failed: Branch not selected"
        );
    }

    #[test]
    fn test_invalid_input_display() {
        let error = PullRequestError::InvalidInput {
            field: "branch_name".to_string(),
            reason: "Cannot be empty".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Invalid input: branch_name - Cannot be empty"
        );
    }

    #[test]
    fn test_repo_not_found_display() {
        let error = PullRequestError::RepoNotFound {
            path: "owner/repo".to_string(),
        };
        assert_eq!(error.to_string(), "Repository not found: owner/repo");
    }

    // Test for the From<octocrab::Error> for PullRequestError
    #[test]
    fn test_from_octocrab_error_github_variant() {
        let github_err_source = GitHubError {
            message: "Resource not accessible by integration".to_string(),
            documentation_url: None,
            errors: None,
        };
        let octocrab_error = octocrab::Error::GitHub {
            source: Box::new(github_err_source.clone()),
            backtrace: Backtrace::disabled(),
        };
        let pr_error: PullRequestError = octocrab_error.into();

        // Format the expected string based on how ApiError's Display impl works
        let expected_display_string = format!(
            "GitHub API error: Resource not accessible by integration (Source: Some(GitHubError {{ message: \"Resource not accessible by integration\", documentation_url: None, errors: None }}))"
        );
        assert_eq!(pr_error.to_string(), expected_display_string);

        // Also test the fields directly if needed for more robustness, though Display is key here
        if let PullRequestError::ApiError { message, source } = pr_error {
            assert_eq!(message, "Resource not accessible by integration");
            assert_eq!(source, Some(github_err_source));
        } else {
            panic!("Incorrect error variant, expected ApiError");
        }
    }

    #[test]
    fn test_from_octocrab_error_other_variant() {
        // Create a dummy std::io::Error
        let io_error = std::io::Error::new(std::io::ErrorKind::Other, "network error");
        // Create a reqwest::Error from the std::io::Error
        // This is a bit indirect; reqwest::Error::from() typically takes a reqwest::error::ErrorImpl.
        // For testing, we might need to simulate a more realistic reqwest error or accept some limitations.
        // A simple way for testing display is to ensure it captures the underlying error's string form.
        let simulated_reqwest_error =
            ReqwestError::from(std::io::Error::new(std::io::ErrorKind::Other, "network error"));

        let octocrab_error_instance = octocrab::Error::Http {
            source: simulated_reqwest_error,
            backtrace: Backtrace::disabled(),
        };

        let pr_error: PullRequestError = octocrab_error_instance.into();

        // The Display for PullRequestError::ApiError when source is None (because Http is not GitHubError)
        // should use the octocrab_error.to_string() as the message.
        // Recreate the error for consistent string comparison
        let comparison_reqwest_error =
            ReqwestError::from(std::io::Error::new(std::io::ErrorKind::Other, "network error"));
        let expected_message = format!("{}", octocrab_error_to_string_for_comparison(&octocrab::Error::Http {
            source: comparison_reqwest_error,
            backtrace: Backtrace::disabled(),
        }));

        let expected_display_string = format!(
            "GitHub API error: {} (Source: None)",
            expected_message
        );
        assert_eq!(pr_error.to_string(), expected_display_string);

        if let PullRequestError::ApiError { source, .. } = pr_error {
            assert!(source.is_none());
        } else {
            panic!("Incorrect error variant, expected ApiError");
        }
    }

    // Helper function to manage the reqwest error string comparison, as direct creation is tricky
    fn octocrab_error_to_string_for_comparison(err: &octocrab::Error) -> String {
        // This helper should ideally match how octocrab::Error's Display trait works for Http errors.
        // For now, we'll keep it simple, but for full accuracy, one might need to inspect octocrab's source.
        // The primary goal is that *our* PullRequestError::ApiError correctly incorporates this string.
        err.to_string()
    }
}
