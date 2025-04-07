{
  description = "feframe";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
          extensions = [ "clippy" ];
        };

        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          openssl

          cargo-shuttle
          trunk
        ];

        buildInputs = with pkgs; [ openssl ];
      in
      {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;
          packages = with pkgs; [ pre-commit ];

          shellHook = ''
            pre-commit install
            export FLAKE_ROOT="$PWD"

            build() {
              cd "$FLAKE_ROOT/frontend"
              trunk build --release --features local
              cd "$FLAKE_ROOT"
              cargo build --features local
            }
            export -f build
          '';
        };
      }
    );
}
