# PRT: Pull request TUI

## Description
TUI for managing pull requests on GitHub.

## Installation

### Using Homebrew

Just tap the repository and install the application.
```
brew tap djego/prt
brew install prt
```

### build from source

Go to releases and download the latest version for your platform.

```
chmod a+x prt
sudo mv prt /usr/local/bin
```

## Usage

You should have a GitHub token in order to use this application (PAT)

You can create one [here](https://github.com/settings/tokens).

After that, you can run the application with the following command:
````bash
prt
````
Insert PAT and you are ready to go!

## Development

### Nix Development Environment (Recommended)

This project includes a complete Nix flakes environment with Rust 1.92.0, Git, and all development tools pre-configured.

**Prerequisites**: 
- [Nix](https://nixos.org/download.html) with flakes enabled
- [direnv](https://direnv.net/) (optional, for automatic environment loading)

**Quick Start**:

```bash
# Clone the repository
git clone https://github.com/djego/prt.git
cd prt

# Option 1: With direnv (automatic)
direnv allow
# Environment loads automatically!

# Option 2: Manual
nix develop
```

**Custom Development Commands**:

The Nix environment includes convenient shortcuts:

```bash
build              # Build the project
build-release      # Build optimized release
run                # Run the application
test               # Run all tests
test-one <name>    # Run specific test
check              # Quick compilation check
fmt                # Format code
clippy             # Lint code
clippy-pedantic    # Strict linting
```

**Example Workflow**:

```bash
cd prt
check              # Quick verification
build              # Compile
test               # Run tests
run                # Execute
```

📖 **Full Documentation**: See [NIX_SETUP.md](NIX_SETUP.md) for complete setup guide, troubleshooting, and advanced usage.

### Traditional Development (without Nix)

If you prefer to use standard Rust tools without Nix:

```bash
# Clone the repository
git clone https://github.com/djego/prt.git
cd prt

# Install dependencies and run
cargo update
cargo run
```

## Demo
![prt demo](https://github.com/user-attachments/assets/dda30cbf-7e9f-47fe-b091-dbb1d630d4a8)

Enjoy creating pull requests from TUI with PRT 🚀!!
