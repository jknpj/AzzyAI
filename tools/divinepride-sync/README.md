# divinepride-sync

Rust port of `tools/harness/divinepride_sync.py`: downloads the homunculus skill data AzzyAI uses
from the Divine Pride API and writes the same fixture, byte for byte.

Output: `tools/harness/fixtures/divinepride_<server>.json` (raw API answers keyed by skill ID, plus
the server and the date). It is **git-ignored**: the file holds third-party data and stays local.
It exists so the official values in `USER_AI/H_SkillList.lua` can be compared with what the LATAM
client actually uses.

Usage (needs a free API key from your divine-pride.net profile page):

```
set DIVINEPRIDE_API_KEY=<your key>
cargo run --release --manifest-path tools/Cargo.toml -p divinepride-sync -- [--server LATAM] [--language en] [--delay 1.0] [--refresh]
```

- `--server` Divine Pride server code (default `LATAM`); the file name uses its lowercase form.
- `--language` `Accept-Language` for names and descriptions (default `en`).
- `--delay` seconds between requests (default 1.0); be kind to the API.
- `--refresh` fetches every skill again instead of reusing the answers already in the file.

The crate is a member of the `tools/Cargo.toml` workspace, together with `rathena-sync`:
`cargo build --release --manifest-path tools/Cargo.toml` builds both and they share `tools/target/`.

Only the 53 known homunculus skill IDs are requested (no enumeration), answers already saved are
reused, and `429` answers are retried after `Retry-After` (five attempts, 30 s when the header is
missing). `401` stops with "the API key was refused"; `404` is recorded as "not found" and the run
continues. The key is read from the environment only: it is never printed, logged or written.

Parity with the Python script: `json.dump(indent=1, sort_keys=True, ensure_ascii=False)` is
reimplemented exactly (one space per level, sorted keys, raw UTF-8, no trailing newline). Checking
the port is easy and needs no API key: run it once with a dummy key on a machine that already has
the fixture — every skill is cached, so it makes no request — and `diff` the result against the
saved file; only the `"date"` line may differ.

Dependencies: `reqwest` (blocking, rustls) and `serde_json`. The Python script stays the reference
implementation; if the two ever disagree, the Python output is the expected one. This crate is not
part of CI (it would need a Rust toolchain for a rarely-run sync); the committed fixture is
git-ignored and never tested.
