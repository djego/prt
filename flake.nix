{
  description = "PRT: Pull Request TUI - Development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.cargo
            pkgs.rustc
            pkgs.clippy
            pkgs.rustfmt
            
            # Development tools
            pkgs.git
          ];

          shellHook = ''
            echo "🦀 PRT Development Environment"
            echo "Rust version: $(rustc --version)"
            echo ""
            echo "Available commands:"
            echo "  build          - Build the project"
            echo "  build-release  - Build optimized release"
            echo "  run            - Run the application"
            echo "  test           - Run all tests"
            echo "  test-one       - Run specific test (usage: test-one <test_name>)"
            echo "  check          - Quick check without building"
            echo "  fmt            - Format code"
            echo "  clippy         - Lint code"
            echo "  clippy-pedantic - Pedantic linting"
          '';

          packages = with pkgs; [
            (writeShellScriptBin "build" "cargo build")
            (writeShellScriptBin "build-release" "cargo build --release")
            (writeShellScriptBin "run" "cargo run")
            (writeShellScriptBin "test" "cargo test")
            (writeShellScriptBin "test-one" ''cargo test "$@"'')
            (writeShellScriptBin "check" "cargo check")
            (writeShellScriptBin "fmt" "cargo fmt")
            (writeShellScriptBin "clippy" "cargo clippy")
            (writeShellScriptBin "clippy-pedantic" "cargo clippy -- -W clippy::pedantic")
          ];
        };
      }
    );
}
