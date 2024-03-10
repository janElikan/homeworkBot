{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils"; # cross-platform compatibility
  };
  outputs = { self, nixpkgs, flake-utils }:
  flake-utils.lib.eachDefaultSystem # architectures
    (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [ cargo rustc rustfmt clippy bacon ];
        };
      }
    );
}
