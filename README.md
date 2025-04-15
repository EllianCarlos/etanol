# Etanol

This is an attempt to create a Kotlin LSP in Rust. I don't know kotlin and I don't know rust, let's see how this works out.

## Installation?
This project uses [devenv](https://devenv.sh). Just do:
```sh
devenv shell
```

and you have a shell with everything ready to go. If you use macos or windows, help setting those environments is welcome! 

You're also gonna need to port the `tree-sitter-kotlin` to `c` to enable interop with Rust. For this do:
```sh 
cd vendor/tree-sitter-kotlin
npm install
npx tree-sitter generate
cd ../..
```

Or just:
```sh
cd vendor/tree-sitter-kotlin
tree-sitter generate
cd ../..
```

In NixOS (or an OS supporting devenv).

## Testing

After entering a devenv shell, open the client folder in vscode, press `F5`, this will open another vscode windows now running in the extension developer mode. After that you should be able to be using the LSP.
If you need to debug enable developer tools after: `View > Outputs` in the toolbar, then select `Kotlin Language Server`
