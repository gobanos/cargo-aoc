{
  description = "A helper CLI tool for Rust solutions to Advent of Code";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, fenix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        toolchain = fenix.packages.${system}.stable.toolchain;
        cargoToml = builtins.fromTOML (builtins.readFile ./cargo-aoc/Cargo.toml);
      in
      {
        packages = rec {
          cargo-aoc = (pkgs.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          }).buildRustPackage {
            pname = cargoToml.package.name;
            version = cargoToml.package.version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            buildAndTestSubdir = "cargo-aoc";
          };
          default = cargo-aoc;
        };
      });
}
