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
- Options the shipped file does not have yet are shown with their schema default (not as an empty
  field marked "changed") and are appended only when set to something other than the default,
  exactly like the web GUI; resetting one removes the line again.

```
cargo run --release --manifest-path tools/Cargo.toml -p azzy-native -- [USER_AI folder] [--write]
```

## Milestone 2 (done)

- **Tactics, Mercenary Tactics, PVP Tactics and Merc PVP Tactics** are loaded together with the
  settings and edited in their own tabs: the entry list on the left, name/id and the "Overall
  Tactics" form on the right, with the same fields/options as `webgui/tactics.js` (basic behavior,
  skill mode, debuff, kiting, cast reaction, class, snipe, rescue, SP, kill-steal, weight, chase).
- `azzy_config::tactics` parses `MyTact[...]`/`MyPVPTact[...]` and preserves every unedited line;
  edits rewrite only their entry, new entries are appended, removed entries are dropped, and a
  file created from scratch gets `MyTact={}`.
- `webgui/mobdata.js` is dumped to `data/mobs.json` by the same script as the schema (2675
  monsters), so the editor shows the monster name/level/element/race under the id.
- Apply Settings now writes every file with pending edits (settings, tactics and extras).
- **Extra Options** tab: a monospaced free-form editor for `H_Extra.lua`/`M_Extra.lua` (the files the
  GUI does not model), with a Homunculus/Mercenary selector and the same "not applied" badge.
- **Suggested tactics**: "Suggest..." in a tactics tab opens the panel from `tactSuggest.js`; a
  map or monster name/id adds the rows the monster database implies (plant, MVP, boss, aggressive,
  immobile, passive), "Add all plants and mushrooms" does the bulk case, and "Replace rows that
  already exist" controls duplicates. The search lists the first matching monsters by name; the
  window is anchored to the right and closes after a successful add so it never covers the list.
- The entry list navigates with **Up/Down** (one row), **PageUp/PageDown** (ten) and **Home/End**
  when no text field or popup has the keyboard, and scrolls the selected row into view.

## Milestone 3 (in progress)

- **Per-homunculus presets**: the Homunculus tab has "Settings for: Global (all homunculi) /
  Lif / Amistr / Filir / Vanilmirth / Eira / Bayeri / Sera / Dieter / Eleanor". With a type
  selected the grid shows the merged view (the type override, otherwise the global value) and
  edits go to `Types/<Name>.lua`; a value equal to the global one removes the override, so only
  the differences are saved. "Use global values" removes every override of that type. The file
  keeps its comments/formatting and the footer path follows the selected type.
- A config now tracks line additions/removals (`is_modified()`), so removing an override or an
  appended option still enables Apply and rewrites the file.
- **Files** (navbar): export every configuration file (including unapplied edits) to a folder,
  import a folder (the known files are replaced in the editor and marked dirty; Apply writes them
  verbatim), and export a diagnostics folder with the paths, counts and the tail of
  `azzy-native.log`. It is folder-based, so no native file-dialog dependency is needed.

## Icons

The window and the navbar use the real `webgui/azzyai.ico`, converted once to
`data/azzyai-48.rgba` (48x48, raw RGBA) so egui can use it without an image-loading dependency.
The remaining icons (skill icons from `webgui/img/skills`, element icons) are part of M4.

## Still missing for parity with the web GUI

- The update flow from GitHub (`azzy-gui` already has the zip logic).
- UI translations (pt/es) and the option descriptions in those languages (`i18n.desc.*.js`).

`cargo test -p azzy-native` covers the schema load (185+78 options, enums, defaults), the tactics
editor (including a simulated row click and the suggestion rules) and a real save into a temporary
folder; `cargo test -p azzy-config` covers the config and tactics round-trips.
