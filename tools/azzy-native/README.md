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

- **Homunculus and Mercenary tabs** driven by the *same schema the web GUI uses*: the web GUI
  repository publishes `data/options.json` (263 options with categories, types, enums, defaults,
  min/max and the English descriptions), `data/i18n.json` (its translations) and
  `data/skilldata.json` (skill names, descriptions and stats); the native window embeds those files
  from the `webgui` submodule, so the two never drift.
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
  Lif / Amistr / Filir / Vanilmirth". With a type
  selected the grid shows the merged view (the type override, otherwise the global value) and
  edits go to `Types/<Name>.lua`; a value equal to the global one removes the override, so only
  the differences are saved. "Use global values" removes every override of that type. The file
  keeps its comments/formatting and the footer path follows the selected type.
- The same selector **filters the grid like the web GUI**: choosing a Homunculus S lists its own
  settings plus every base homunculus's, a base homunculus lists only its own (every S hidden),
  and Global lists everything.
- A config now tracks line additions/removals (`is_modified()`), so removing an override or an
  appended option still enables Apply and rewrites the file.
- **Files** (navbar): export every configuration file (including unapplied edits) to a folder,
  import a folder (the known files are replaced in the editor and marked dirty; Apply writes them
  verbatim), and export a diagnostics folder with the paths, counts and the tail of
  `azzy-native.log`. It is folder-based, so no native file-dialog dependency is needed.

## Languages

The language selector in the navbar offers **English**, **Português (BR)** and **Español (LatAm)**,
the same three choices as the web GUI, and the choice is remembered between runs (in
`%APPDATA%\azzy-native\lang` on Windows, `$XDG_CONFIG_HOME/azzy-native/lang` or
`~/.config/azzy-native/lang` elsewhere; on the first run the system locale decides, English
otherwise). It never writes anything into USER_AI.

- UI strings are translated through `data/i18n.json`, generated from the web GUI's
  `i18n.strings.js` and keyed by the English text, so a label the web GUI translates is translated
  here too.
- Setting descriptions come from `i18n.desc.pt.js`/`i18n.desc.es.js` for all 263 options; search
  matches both the English and the translated description.
- Labels that only this window has (Reload, Files, the language list, the suggestion window) live
  in `src/i18n.rs`; anything without a translation stays English.

## Icons

The window and the navbar use the real `webgui/azzyai.ico`, converted once to
`data/azzyai-48.rgba` (48x48, raw RGBA) so egui can use it without an image-loading dependency.
The executable also carries the same `.ico` as a Windows resource (`build.rs`/`winres`), so Explorer
and the file's properties show the AzzyAI icon and version instead of a blank document.
The **skill icons** from `webgui/img/skills` (50 PNGs) are packed once into
`data/skill-icons.rgba` (24x24 tiles, 4 columns, one per `webgui/img/skills` PNG: 13 icons) with `data/skills.json` holding the ids, the
homunculus, the names and the same `match` substrings as `webgui/skills.js`. The Homunculus tab
has the **Homunculus skills** card (grouped by type): click a skill to show only its settings,
click the chip to clear. The grid also shows the icon before each mapped setting name, and the
tooltip/description names the skill.
- **Skill card**: for a setting that belongs to a skill, the Description tab shows the same header
  as the web GUI — the icon, the official name, the numbers at max level (SP, cast, cooldown,
  duration) and the in-game description from Divine Pride, which lists the values per level. The
  `^RRGGBB` client colour codes are rendered as coloured text. The data comes from
  `webgui/skills.js` + `webgui/skilldata.js`, dumped to `data/skilldata.json` by the same script as
  the schema. Nothing is loaded at runtime: the files are embedded like the monster database.

## Update check

On startup the window asks GitHub for the latest release tag (`/releases/latest`, falling back to
`/tags`) and compares it with the `CVersion` in the loaded USER_AI. When it is newer, a slim bar
under the navbar says **New version vX.Y.Z is available** and offers **Update** (opens the update
window) and **Dismiss**. The check runs in the background: offline, rate-limited or without a
release it stays silent and never blocks the window.

## Updating from GitHub

The navbar's **Update AI from GitHub...** button runs the same flow as the Windows app and the
`azzy-gui` host (`tools/azzy-update`, shared by both): it downloads the `prerenewal` branch zip, keeps
the files the player edits (`H_Config.lua`, `M_Config.lua`, the tactics/extra files, `H_Avoid.lua`,
`A_Friends.lua`, `Mob_ID.lua`), replaces the shipped ones after copying them to
`USER_AI/backup/update-<timestamp>/`, and stages a newer executable as `.exe.new` (a running exe
cannot be replaced). `Types/*.lua` presets are kept unless **Replace Types/ presets too** is
checked. The window shows the counts and the backup path; if nothing was edited it reloads the
files, otherwise it asks for a Reload.

The window can update itself too: the archive is searched for a binary with the running
executable's file name, wherever it is kept (`tools/azzy-native/` while it is developed next to the
C# GUI, `USER_AI/` once it replaces `AzzyAIConfig.exe`), and when the bytes differ it is staged as
`azzy-native.exe.new` next to the running file. **Restart now** closes
the window, waits for it to exit, swaps the binary and starts it again (the same trick the old
Windows app uses, because a running exe cannot be replaced). When the archive does not ship that
build the window says so and only the AI files are updated.

The update source is fixed to the official repository's `main` zip. The **Folder to update:** field
is editable and starts at the folder where the executable is when that folder holds
`H_Config.lua` (the shipped layout, with the exe next to the AI files, replacing the old
`AzzyAIConfig.exe`); otherwise it starts at the USER_AI the window was opened with. The exe also
finds the configuration next to itself, so in that layout no command-line argument is needed.

`cargo test -p azzy-native` covers the schema load (185+78 options, enums, defaults), the tactics
editor (including a simulated row click and the suggestion rules) and a real save into a temporary
folder; `cargo test -p azzy-config` covers the config and tactics round-trips.
