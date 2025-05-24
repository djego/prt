#[cfg(test)]
mod tests {
    use crate::core::app::App;
    use crate::core::input_mode::InputMode;

    #[test]
    fn test_app_initialization() {
        let app = App::new();

        // Existing checks
        assert_eq!(app.pull_request.target_branch, "main", "Default target branch should be main");
        assert!(app.pull_request.title.is_empty(), "Initial PR title should be empty");
        assert!(app.pull_request.description.is_empty(), "Initial PR description should be empty");

        // New assertions based on expected initial state in a test environment
        assert_eq!(app.repo_owner, "-", "Default repo_owner should be '-' in test env");
        assert_eq!(app.repo_name, "-", "Default repo_name should be '-' in test env");
        assert_eq!(app.pull_request.source_branch, "-", "Default source_branch should be '-' in test env");
        assert_eq!(app.config_pat, "", "Initial config_pat should be an empty string");
        assert_eq!(app.input_mode, InputMode::Normal, "Initial input_mode should be Normal");
        assert_eq!(app.current_field, 0, "Initial current_field should be 0");
        assert!(!app.show_confirm_popup, "show_confirm_popup should be false initially");
        assert!(!app.show_pat_popup, "show_pat_popup should be false initially");
        assert!(!app.show_exit_popup, "show_exit_popup should be false initially");
        assert!(app.error_message.is_none(), "Initial error_message should be None");
        assert!(app.success_message.is_none(), "Initial success_message should be None");
        assert!(app.description_text_area.is_empty(), "Initial description_text_area should be empty");
        assert!(app.pat_input.is_empty(), "Initial pat_input should be empty");
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

        // Setup: Modify fields that should be reset
        app.pull_request.title = "Test Title".to_string();
        app.pull_request.description = "Test Description".to_string();
        app.set_error("An error occurred".to_string());
        app.set_success("Test Success".to_string());
        app.description_text_area.insert_str("Initial text in textarea");
        app.input_mode = InputMode::Editing; // Change from default
        app.current_field = 1; // Change from default
        app.show_confirm_popup = true; // Change from default

        // Call the reset function
        app.reset();

        // Assertions: Verify fields are reset to their default/initial states
        assert!(app.pull_request.title.is_empty(), "PR Title should be empty after reset");
        assert!(app.pull_request.description.is_empty(), "PR Description should be empty after reset");
        assert!(app.error_message.is_none(), "Error message should be None after reset");
        assert!(app.success_message.is_none(), "Success message should be None after reset");
        assert_eq!(app.input_mode, InputMode::Normal, "Input mode should be Normal after reset");
        assert_eq!(app.current_field, 0, "Current field should be 0 after reset");
        assert!(!app.show_confirm_popup, "Show confirm popup should be false after reset");
        assert!(app.description_text_area.is_empty(), "Description textarea should be empty after reset");
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

        app.clear_message();

        assert!(app.success_message.is_none());
    }

    #[test]
    fn test_get_current_field_mut_title() {
        let mut app = App::new();
        app.current_field = 0;
        let title_field = app.get_current_field_mut();
        *title_field = "New Title".to_string();
        assert_eq!(app.pull_request.title, "New Title");
    }

    #[test]
    fn test_get_current_field_mut_description() {
        let mut app = App::new();
        app.current_field = 1;
        let description_field = app.get_current_field_mut();
        *description_field = "New Description".to_string();
        assert_eq!(app.pull_request.description, "New Description");
    }

    #[test]
    fn test_get_current_field_mut_source_branch() {
        let mut app = App::new();
        app.current_field = 2;
        let source_branch_field = app.get_current_field_mut();
        *source_branch_field = "feature/new-branch".to_string();
        assert_eq!(app.pull_request.source_branch, "feature/new-branch");
    }

    #[test]
    fn test_get_current_field_mut_target_branch() {
        let mut app = App::new();
        app.current_field = 3;
        let target_branch_field = app.get_current_field_mut();
        *target_branch_field = "develop".to_string();
        assert_eq!(app.pull_request.target_branch, "develop");
    }

    #[test]
    fn test_confirm_pull_request() {
        let mut app = App::new();
        app.confirm_pull_request();

        assert_eq!(app.input_mode, InputMode::Creating);
        assert!(app.show_confirm_popup);
    }

    #[test]
    fn test_is_editing_description() {
        let mut app = App::new();

        // Scenario 1: current_field is 1
        app.current_field = 1;
        assert!(app.is_editing_description(), "Should be true when current_field is 1");

        // Scenario 2: current_field is not 1
        app.current_field = 0;
        assert!(!app.is_editing_description(), "Should be false when current_field is 0");

        app.current_field = 2;
        assert!(!app.is_editing_description(), "Should be false when current_field is 2");
        
        // Test with another value just to be sure
        app.current_field = 3;
        assert!(!app.is_editing_description(), "Should be false when current_field is 3");
    }
}
