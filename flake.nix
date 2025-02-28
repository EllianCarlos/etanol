{
  description = "Etanol - A declarative engine for managing orchestration.";

  inputs = {
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
    };

    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable"; 
    nixpkgs.follows = "rust-overlay/nixpkgs";
    systems.url = "github:nix-systems/default";
    flake-utils = {
        url = "github:numtide/flake-utils";
        inputs.systems.follows = "systems";
    };
  };

  outputs = { self, systems, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let 
        pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlays.default ]; };
      in rec {
        packages = {
          etanol = pkgs.rustPlatform.buildRustPackage {
            name = "etanol";
            pname = "etanol";
            version = "0.0.0";
            src = self;

            cargoLock.lockFile = ./Cargo.lock;

            nativeBuildInputs = with pkgs; [ pkg-config ];
            buildInputs = with pkgs; [ protobuf ];

            buildPhase = ''
              cargo build
            '';

            meta = {
              description = "Etanol - A declarative engine for managing orchestration.";
              license = pkgs.lib.licenses.mit;
              maintainers = with pkgs.lib.maintainers; [ yourname ];
            };
          };
          default = packages.etanol;
        };

        devShells.default = pkgs.mkShell {
          shellHook = ''
            rustup default stable
          '';

          buildInputs = with pkgs; [
            rustup
            rustc
            cargo
            rustfmt
            clippy

            protobuf

            nil
            nixpkgs-fmt
            direnv
          ];
        };

      formatter = pkgs.nixfmt-rfc-style;
      ## packages.default = pkgs.callPackage ./build.nix {};
    });
}

