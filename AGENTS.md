# Agent Guidelines for PRT (Pull Request Tool)

## Development Commands
- **Build**: `cargo build`
- **Run**: `cargo run`
- **Test all**: `cargo test`
- **Test single**: `cargo test test_name` (replace test_name with actual test function name)
- **Format**: `cargo fmt`
- **Lint**: `cargo clippy`

## Code Style Guidelines

### Project Structure
- Use `src/core/` for business logic, `src/ui/` for UI components
- Test files named `*_test.rs` alongside implementation files
- Module declarations in `mod.rs` files

### Imports
- Group imports: std → external crates → local modules
- Use absolute paths for local imports: `use crate::core::module::Type`

### Naming Conventions
- **Functions/Methods**: snake_case (`get_current_branch`, `create_github_pull_request`)
- **Types/Structs**: PascalCase (`PullRequest`, `App`, `InputMode`)
- **Constants**: SCREAMING_SNAKE_CASE
- **Fields**: snake_case, public fields preferred

### Error Handling
- Use `thiserror::Error` for custom error types
- Return `Result<T, Error>` for fallible operations
- Use `?` operator for error propagation
- Custom errors: `PullRequestError` enum with descriptive variants

### Async Programming
- Use `tokio` runtime for async operations
- Block on async calls in main: `runtime.block_on(async_fn())`
- Handle async results with `match` statements

### Testing
- Unit tests in `#[cfg(test)]` modules
- Test functions marked with `#[test]`
- Use descriptive test names: `test_app_initialization`, `test_enter_edit_mode`
- Assert expected vs actual values with clear messages

### Dependencies
- **UI**: ratatui, crossterm, tui-textarea
- **GitHub**: octocrab
- **Async**: tokio
- **Serialization**: serde, toml
- **Error handling**: thiserror

### Code Patterns
- Struct initialization with `Default::default()` or explicit constructors
- Option handling with `if let Some()` and `unwrap_or_else()`
- String operations with `.to_string()`, `.join()`, `.push()`
- Enum matching with exhaustive `match` statements