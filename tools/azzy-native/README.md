# azzy-native (prototype)

Native configuration GUI for AzzyAI in Rust with `egui` - the "full rewrite" path, as a slice.

Scope of the spike: load a configuration through `azzy-config`, list its options grouped by the
file's comment sections, search, edit values in place, show how many changed, Save and Reload.

```
cargo run --release --manifest-path tools/Cargo.toml -p azzy-native -- [file] [--write]
```

- default file `USER_AI/H_Config.lua`; Save writes `<file>.new` unless `--write` is given.
- `cargo test -p azzy-native` covers the editing loop headlessly (load, change one value, save to
  `.new`, original untouched).
- Release binary: ~6.3 MB, no runtime to install (egui + glow, no GTK/WebView).

What a real port would still need, and why this is not "the app already works": tactics and PVP
grids (the C# `DataGridView` tables), the per-type presets editor, the updater (git/zip), the
diagnostics dialog, translations and the option descriptions (the web GUI has them in
`i18n.desc.*.js`; the C# app in attributes). The C# app is ~24k lines, most of it form
infrastructure that the schema-driven approach would replace - but that replacement is the project,
not this prototype.

The `egui` dependency tree (~200 crates) is why the workspace keeps this crate out of
`default-members`: a plain `cargo build` at `tools/` builds everything else.
