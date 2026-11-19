# azzy-native

The AzzyAI configuration GUI in Rust (egui), mirroring the web GUI.

## Milestone 1 (done)

- **Homunculus and Mercenary tabs** driven by the *same schema the web GUI uses*: `tools/webgui-data.js`
  dumps `homSchema.js`/`merSchema.js` (263 options with categories, types, enums, defaults,
  min/max and the English descriptions) to `data/options.json`, which is committed and checked for
  drift in CI (`node tools/webgui-data.js --check`).
- Search over names **and** descriptions, category filter, "only non-default" filter, changed
  counter, and a description panel for the selected option (with category, type, range, default).
- Typed editors: `bool` checkbox, `int`/`double` text field with the same min/max clamp as the web
  GUI, `enum` and `serverType` combo boxes with the web GUI's labels.
- Apply/Revert through `azzy-config`, which keeps the file byte for byte: only the values you
  edited change, comments/tabs/order/line endings stay. Save writes `<file>.new` unless `--write`.

```
cargo run --release --manifest-path tools/Cargo.toml -p azzy-native -- [USER_AI folder] [--write]
```

## Still missing for parity with the web GUI

- Tactics, PVP tactics and Extra Options tabs (the DataGridView-like grids; needs `mobdata.js` for
  monster names and maps).
- Per-type presets (`Types/<Name>.lua`, the "Settings for" selector).
- Import/export dialogs, the update flow (`azzy-gui` already has the zip logic) and the
  diagnostics bundle (the Windows app already has it).
- UI translations (pt/es) and the option descriptions in those languages (`i18n.desc.*.js`).

`cargo test -p azzy-native` covers the schema load (185+78 options, enums, defaults) and the
editing loop; `cargo test -p azzy-config` covers the file round-trip.
