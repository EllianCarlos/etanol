{
  description = "Etanol - A declarative engine for managing orchestration.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable"; 
    systems.url = "github:nix-systems/default";
    flake-utils = {
        url = "github:numtide/flake-utils";
        inputs.systems.follows = "systems";
    };
    import-cargo.url = "github:edolstra/import-cargo";
  };

  outputs = { self, systems, nixpkgs, flake-utils, import-cargo }:
    flake-utils.lib.eachDefaultSystem (system:
      let 
        inherit (import-cargo.builders) importCargo;
        pkgs = import nixpkgs { inherit system; };
      in {
        defaultPackage = pkgs.stdenv.mkDerivation {
          pname = "etanol";
          name = "etanol";
          src = self;
          version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;

          preBuild = ''
            cargo generate-lockfile
          '';

          buildInputs = [
            (importCargo { lockFile = ./Cargo.lock; inherit pkgs; }).cargoHome
            pkgs.cargo
            pkgs.rustc
          ];

          buildPhase = ''
            cargo build --release --offline
          '';

          installPhase = ''
            install -Dm775 ./target/release/etanol $out/bin/etanol
          '';
        };

        devShells.default = pkgs.mkShell {
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

