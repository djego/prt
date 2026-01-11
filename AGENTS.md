# AGENTS.md - PRT Project Guidelines

## Project Overview
PRT (Pull Request TUI) is a terminal user interface for managing GitHub pull requests, built with Rust using `ratatui`, `crossterm`, and `octocrab` for GitHub API interactions.

## Build/Lint/Test Commands

### Basic Commands
- **Build**: `cargo build` - Compile the project
- **Build (release)**: `cargo build --release` - Optimized build
- **Run**: `cargo run` - Run the application
- **Check**: `cargo check` - Fast compilation check without producing binary

### Testing
- **Test all**: `cargo test` - Run all tests
- **Test single**: `cargo test <test_name>` - Run specific test
  - Example: `cargo test test_app_initialization`
  - Example: `cargo test test_reset_function`
- **Test with output**: `cargo test -- --nocapture` - Show println! output during tests
- **Test verbose**: `cargo test -- --show-output` - Display test output

### Code Quality
- **Format**: `cargo fmt` - Auto-format all code
- **Format check**: `cargo fmt -- --check` - Check if code is formatted
- **Lint**: `cargo clippy` - Run linter with helpful suggestions
- **Lint pedantic**: `cargo clippy -- -W clippy::pedantic` - Stricter linting

## Project Structure

```
src/
├── main.rs           # Entry point, event loop, terminal management
├── core/             # Core business logic
│   ├── mod.rs        # Module exports
│   ├── app.rs        # Main App struct and state management
│   ├── app_test.rs   # Unit tests for App
│   ├── config.rs     # Config loading/saving (TOML)
│   ├── errors.rs     # Custom error types
│   ├── git.rs        # Git operations (repo info, branch)
│   ├── github.rs     # GitHub API wrapper
│   ├── input_mode.rs # Input mode enum (Normal/Editing/Creating)
│   └── pull_request.rs # PR data structure
└── ui/               # User interface components
    ├── mod.rs        # UI module exports
    ├── layout.rs     # Ratatui UI rendering
    └── util.rs       # UI utility functions
```

## Code Style Guidelines

### Imports
- **Order**: Group imports in this order:
  1. Standard library (`std::`)
  2. External crates (`octocrab`, `ratatui`, etc.)
  3. Local modules (`crate::core::`, `crate::ui::`)
- **Format**: Use absolute paths for local imports
  ```rust
  use crate::core::app::App;
  use crate::core::errors::PullRequestError;
  ```
- **Spacing**: Add blank line between import groups

### Naming Conventions
- **Functions**: `snake_case`
  - Examples: `get_current_field_mut`, `create_github_pull_request`, `sync_github_repo_info`
- **Structs/Enums**: `PascalCase`
  - Examples: `PullRequestError`, `App`, `InputMode`, `GithubRepository`
- **Variables**: `snake_case`
  - Examples: `repo_owner`, `current_branch`, `config_pat`
- **Constants**: `SCREAMING_SNAKE_CASE` (if used)
- **Private fields**: Use `pub` selectively; most struct fields are public in this codebase

### Error Handling
- **Use `thiserror::Error`** for custom error types:
  ```rust
  #[derive(Debug, Error)]
  pub enum PullRequestError {
      #[error("GitHub API error: {0}")]
      ApiError(#[from] octocrab::Error),
      #[error("Invalid input: {0}")]
      InvalidInput(String),
  }
  ```
- **Return `Result<T, E>`** for all fallible operations
- **Pattern matching**: Use match for comprehensive error handling:
  ```rust
  match result {
      Ok(pr) => Ok(pr),
      Err(e) => {
          if let octocrab::Error::GitHub { source, .. } = &e {
              // Handle specific error codes
          }
      }
  }
  ```
- **Avoid `unwrap()`/`expect()`** except in:
  - Tests where panic is acceptable
  - Initialization where failure is unrecoverable
  - Use `unwrap_or_else()` with defaults when appropriate

### Types and Lifetimes
- **Explicit types**: Use when clarity is needed, especially in function signatures
- **String types**:
  - Use `&str` for string slices and function parameters
  - Use `String` for owned strings and struct fields
  - Use `.to_string()` for conversions
- **Lifetimes**: Use `'static` for TextArea fields (e.g., `TextArea<'static>`)
- **Options**: Use `Option<T>` for optional values (e.g., `error_message: Option<String>`)

### Async/Await
- **Runtime**: Use `tokio` with "full" features
- **Async functions**: Mark with `async fn` and return `Result<T, E>`
  ```rust
  pub async fn create_github_pull_request(&self) -> Result<OctocrabPullRequest, PullRequestError>
  ```
- **Calling async**: Use `.await` and handle `Result`:
  ```rust
  let pr_result = octocrab.pulls(&owner, &repo).create(...).await;
  ```
- **Blocking**: Use `runtime.block_on()` in synchronous contexts (main loop)

### Struct and Impl Patterns
- **Struct definition**: Public fields for main app state
  ```rust
  pub struct App {
      pub error_message: Option<String>,
      pub pull_request: PullRequest,
      // ...
  }
  ```
- **Constructor**: Use `new()` method:
  ```rust
  impl App {
      pub fn new() -> App { /* ... */ }
  }
  ```
- **Methods**: Group related functionality in impl blocks
- **Getters/Setters**: Use explicit methods (e.g., `set_error()`, `get_current_field_mut()`)

### Code Structure
- **Functions**: Keep focused and single-purpose (typically < 50 lines)
- **Pattern matching**: Use extensively for enums and Options:
  ```rust
  match app.input_mode {
      InputMode::Normal => { /* ... */ }
      InputMode::Editing => { /* ... */ }
      InputMode::Creating => { /* ... */ }
  }
  ```
- **Immutability**: Prefer immutable bindings; use `mut` only when necessary
- **Early returns**: Use for validation and error cases:
  ```rust
  if self.pull_request.source_branch.is_empty() {
      return Err(PullRequestError::InvalidInput("Source branch is empty".to_string()));
  }
  ```

### Testing
- **Test modules**: Use `#[cfg(test)]` and `mod tests`
- **Test names**: Descriptive with `test_` prefix (e.g., `test_app_initialization`)
- **Assertions**: Use specific assertion messages:
  ```rust
  assert_eq!(app.input_mode, InputMode::Normal, "Initial input_mode should be Normal");
  ```
- **Setup**: Initialize test state explicitly
- **Coverage**: Test happy path, edge cases, and error conditions

### UI/Terminal (Ratatui)
- **Layout**: Use `Layout::default()` with constraints
- **Widgets**: `Block`, `Paragraph`, `Clear` for popups
- **Styling**: Use `Style::default()` with color/modifiers
- **Rendering**: `f.render_widget()` for static, `f.render_stateful_widget()` for stateful

### Dependencies (Cargo.toml)
Current key dependencies:
- `crossterm = "0.28.1"` - Terminal manipulation
- `octocrab = "0.43.0"` - GitHub API client
- `ratatui = "0.29.0"` - Terminal UI framework
- `serde = { version = "1.0", features = ["derive"] }` - Serialization
- `thiserror = "2.0.12"` - Error handling
- `tokio = { version = "1.40.0", features = ["full"] }` - Async runtime
- `toml = "0.8.19"` - Config file parsing
- `tui-textarea = "0.7.0"` - Text area widget

**Guideline**: Keep dependencies minimal and well-maintained. Prefer established crates.

## Common Patterns

### State Management
- App state is centralized in `App` struct
- Input modes control event handling logic
- Popups are managed with boolean flags

### Event Handling
- Read events with `crossterm::event::read()`
- Match on `KeyCode` for keyboard input
- Different behavior based on `input_mode`

### GitHub API Calls
- Build `Octocrab` instance with PAT
- Use async/await for API calls
- Handle specific HTTP status codes (404, 422)
- Return custom `PullRequestError` types

## Development Workflow

1. **Make changes** to source files
2. **Format**: `cargo fmt`
3. **Check**: `cargo check` (fast feedback)
4. **Lint**: `cargo clippy`
5. **Test**: `cargo test`
6. **Run**: `cargo run` (manual testing)
7. **Build release**: `cargo build --release` (for distribution)

## Git Workflow
- Main branch: `main`
- Feature branches: descriptive names
- Commits: Clear, concise messages