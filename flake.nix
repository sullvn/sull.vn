{
  description = "sull.vn";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.systems.url = "github:nix-systems/default";
  inputs.rust-overlay = {
    url = "github:oxalica/rust-overlay";
    inputs.nixpkgs.follows = "nixpkgs";
  };
  inputs.flake-utils = {
    url = "github:numtide/flake-utils";
    inputs.systems.follows = "systems";
  };

  outputs =
    { nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        rust-bin = rust-overlay.lib.mkRustBin { } pkgs;
        crateVersions = pkgs.lib.listToAttrs (
          map (crate: pkgs.lib.nameValuePair crate.name crate.version) (pkgs.lib.importTOML ./Cargo.lock).package
        );
        rust-toolchain = rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        # wasm-bindgen requires the CLI and library to be the same version
        wasm-bindgen-cli = pkgs."wasm-bindgen-cli_${pkgs.lib.replaceString "." "_" crateVersions.wasm-bindgen}";

        ci-shell = pkgs.mkShell {
          packages = [
            pkgs.nodejs_latest
            pkgs.pnpm
            pkgs.biome
            pkgs.actionlint
            pkgs.gh-actions-lock
            pkgs.zizmor
            rust-toolchain
            wasm-bindgen-cli
            pkgs.binaryen
          ];
        };
      in
        {
          devShells.ci = ci-shell;

          devShells.default = pkgs.mkShell {
            inputsFrom = [ ci-shell ];
            packages = [
              pkgs.colima
              pkgs.docker-client
            ];
          };
        }

    );
}
