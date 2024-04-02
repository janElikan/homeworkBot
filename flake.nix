{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, fenix, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system: {
      packages.default =
        let
          pkgs = nixpkgs.legacyPackages.${system};
          target = "x86_64-unknown-linux-musl";
          toolchain = with fenix.packages.${system}; combine [
            minimal.cargo
            minimal.rustc
            targets.${target}.latest.rust-std
          ];
        in

        (naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        }).buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [ pkgsStatic.stdenv.cc ];
          CARGO_BUILD_TARGET = target;
          CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
        };

      devShell = with import nixpkgs { inherit system; }; mkShell {
        buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy bacon ];
        RUST_SRC_PATH = rustPlatform.rustLibSrc;
      };
    });
}
