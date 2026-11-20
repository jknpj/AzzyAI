# rathena-sync

Regenerates the rAthena-derived data files of
AzzyAI from the same databases, with the same flags and byte-identical output (only the
generation date in the header differs).

Outputs:

| File | Used by |
|---|---|
| `tools/harness/fixtures/rathena.lua` | the offline harness ("server truth" for private-server scenarios, skill trees, damage model) |
| `USER_AI/MobInfo.lua` | the AI itself (monster race/element/size/flags, element table, skill elements) |
| `webgui/mobdata.js` | the web GUI's tactics suggestions (needs the `webgui` submodule checked out) |

Usage:

```
cargo run --release --manifest-path tools/Cargo.toml -p rathena-sync -- [--ref <sha>] [--fixture-only]
```

`--ref` is an rAthena commit, branch or tag (default `master`); `--fixture-only` rewrites only
`fixtures/rathena.lua`. HTTPS_PROXY is honored through reqwest.

Both AzzyAI tool crates live in one workspace (`tools/Cargo.toml`), so
`cargo build --release --manifest-path tools/Cargo.toml` builds `rathena-sync` and
`divinepride-sync` together, sharing `tools/target/` and the dependency build. Running one of them
with `--manifest-path tools/rathena-sync/Cargo.toml` still works.

What it ports, one to one:

- the block-style YAML parser (nested mappings/sequences, plain/quoted scalars, comments; the
  `- Key: value` line rewrite that makes sequence entries mappings);
- the `db/re` database extraction (skills 8001-8099, the homunculus skill trees, the 21 scenario
  monsters, `attr_fix.yml`, `mob_db.yml` + `mob_db2.yml`);
- the `calculateSkillRatio` translation from `src/map/skills/homunculus/*.cpp` to Lua
  expressions (the `idiv` truncating division, `st_`/`blv`/`lv` renaming, and the single
  `base_skillratio +=` guard that refuses anything else);
- the `npc/re/mobs/**.txt` spawn scan for `mobdata.js`.

The Python script stays the reference implementation; when the two ever disagree, the Python
output is the expected one and the Rust port is the bug. The checks that keep them in sync:
run both against the same `--ref` and diff the three outputs, ignoring the `generated <date>`
lines (line endings too: Python writes CRLF on Windows, this writes LF).

Dependencies: `reqwest` (blocking, rustls), `serde_json`, `regex`. No Lua, no Python at
runtime. The crate is not part of CI: building it there would need a Rust toolchain for one
rarely-run regeneration, and the committed outputs are what CI actually tests.
