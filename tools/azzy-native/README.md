# azzy-native

The AzzyAI configuration GUI in Rust (egui), mirroring the web GUI.

## Milestone 1 (done)

The look follows `webgui/index.html`: the blue gradient navbar with the app name and the
Reload/theme buttons, the nav-tabs joined to the panel, the toolbar with search +
"only changed from default" + "Clear filters", category tabs with counts, the grid card
with the coloured row markers (blue = changed from default, amber = not applied yet, with
the reset button) and the description card on the right, and the footer with the changes
badge, Apply Settings, Reload and Quit. The palette (dark and light) is copied from the
`--ro-*` variables of the web GUI; the navbar uses Segoe UI when Windows has it.

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
