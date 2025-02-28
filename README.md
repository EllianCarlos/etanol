## First first setup (not needed because now files are versioned):
1. `direnv allow` and `nix develop`.
2. `cargo build` (you need the Cargo.lock).
3. `nix run github:cargo2nix/cargo2nix` to create `Cargo.nix`.
4. `nix build` to build the files.
