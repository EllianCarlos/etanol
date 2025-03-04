## First first setup (not needed because now files are versioned):
This repo uses NixOS with the oxalica overlay to rust, this enable us to build and develop easier and create reproducible development environments!

1. `direnv allow` and `nix develop`.
2. `cargo build` (you need the `Cargo.lock`).
3. `nix build` to build the application.
4.
