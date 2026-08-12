# gasoline fixtures

The `.kt` files in this directory are copied verbatim from
[EllianCarlos/gasoline](https://github.com/EllianCarlos/gasoline) (commit
`1b02f1fe6163c177e13b376eb1da1a3b70e87cbe`), a real Kotlin test framework.

They are used as a "does real-world Kotlin code break the LSP" smoke corpus
by `src/test_util.rs::load_gasoline_fixtures`, covering annotations,
generics, sealed classes, extension functions, companion objects, and
reflection-heavy metaprogramming. They are not part of etanol's own logic
and must not be edited to fix a failing test — a failure here means the
parser or handler under test needs to change, not the fixture.

To refresh a file, copy it again from the source repository at the commit
noted above (or a newer one) and update this note.
