{
  description = "Etanol - A declarative engine for managing orchestration.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable"; 
    systems.url = "github:nix-systems/default";
    cargo2nix.url = "github:cargo2nix/cargo2nix/unstable";
    nixpkgs.follows = "cargo2nix/nixpkgs";
    flake-utils = {
        url = "github:numtide/flake-utils";
        follows = "cargo2nix/flake-utils";
        inputs.systems.follows = "systems";
    };
  };

  outputs = { self, systems, nixpkgs, flake-utils, cargo2nix }:
    flake-utils.lib.eachDefaultSystem (system:
      let 
        pkgs = import nixpkgs { inherit system; overlays = [ cargo2nix.overlays.default ]; };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.75.0";
          packageFun = import ./Cargo.nix;
          extraRustComponents = [ "clippy" "rustfmt" ];
        };
      in rec {
        packages = {
          etanol = (rustPkgs.workspace.etanol {});
          default = packages.etanol;
        };

        devShells.default = rustPkgs.workspaceShell {
            buildInputs = with pkgs; [
              rustc
              cargo
              rustfmt
              clippy

              nil
              nixpkgs-fmt
              direnv
            ];
        };

      formatter = pkgs.nixfmt-rfc-style;
      ## packages.default = pkgs.callPackage ./build.nix {};
    });
}

