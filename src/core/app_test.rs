#[cfg(test)]
mod tests {
    use crate::core::app::App;
    use crate::core::input_mode::InputMode;
    use std::env;

    #[test]
    fn test_app_initialization() {
        //let (repo_owner, repo_name) = mock_get_repo_info().unwrap();
        env::set_var("GITHUB_DEFAULT_TARGET_BRANCH", "main");
        let app = App::new();

        //assert_eq!(app.repo_owner, repo_owner);
        //assert_eq!(app.repo_name, repo_name);
        assert_eq!(app.default_branch, "main");
        assert!(app.pull_request.title.is_empty()); // asumiendo que title se inicializa vacío
        assert!(app.pull_request.description.is_empty());
    }

    #[test]
    fn test_enter_edit_mode() {
        let mut app = App::new();
        app.enter_edit_mode(1);

        assert_eq!(app.input_mode, InputMode::Editing);
        assert_eq!(app.current_field, 1);
    }

    #[test]
    fn test_reset_function() {
        let mut app = App::new();
        app.pull_request.title = "Test Title".to_string();
        app.set_error("An error occurred".to_string());

        app.reset();

        assert!(app.pull_request.title.is_empty());
        assert!(app.error_message.is_none());
        assert_eq!(app.input_mode, InputMode::Normal);
        assert_eq!(app.current_field, 0);
        assert!(!app.show_popup);
    }

    #[test]
    fn test_set_error() {
        let mut app = App::new();
        app.set_error("An error occurred".to_string());

        assert_eq!(app.error_message, Some("An error occurred".to_string()));
    }

    #[test]
    fn test_set_success() {
        let mut app = App::new();
        app.set_success("Pull request created successfully".to_string());

        assert_eq!(
            app.success_message,
            Some("Pull request created successfully".to_string())
        );
    }

    #[test]
    fn test_clear_success() {
        let mut app = App::new();
        app.set_success("Success".to_string());

        app.clear_success();

        assert!(app.success_message.is_none());
    }
}
