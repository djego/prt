{ pkgs, lib, ... }:

{
  languages.rust = {
    enable = true;
    channel = "nightly";

    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  packages = lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk; [
    frameworks.Security
    pkgs.git
  ]);
}
