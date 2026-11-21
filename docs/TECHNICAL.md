# AzzyAI: technical reference

> **Pre-Renewal build.** This document describes the AI engine, which is shared, but
> the data and skills are the Pre-Renewal ones: the sections about `SkillDamage`,
> and the S options do not apply here. The build's own description, data sources
> and limitations are in [PRERENEWAL.md](PRERENEWAL.md).


[User README](../README.md) | [Português (BR)](../README.pt-BR.md) | [Español](../README.es.md) | [Changelog](../CHANGELOG.md)

This document keeps the technical material previously in the README. It is
intended for maintainers and advanced users, not as the installation guide.
Paths and commands below are relative to the repository root unless stated
otherwise. The version history is in [CHANGELOG.md](../CHANGELOG.md).

## Version

The AI has its own release number, in [semantic versioning](https://semver.org/)
form (`MAJOR.MINOR.PATCH`). It is set once, as `CVersion` in `USER_AI/Const_.lua`,
and shown on the `Release:` line of `AAIStartH.txt` and `AAIStartM.txt` (the startup
reports), so a bug report can say exactly which version it is about. `cargo run
--release -p azzy-harness --` fails if that number and the "Current release" line at the
top of [CHANGELOG.md](../CHANGELOG.md) differ.

| Part | Raise it when |
|---|---|
| MAJOR | A change can break an existing installation or its settings: a renamed or removed option, a changed file layout, a different default that alters behavior for everyone. |
| MINOR | A new feature, option or skill that works without touching existing settings. |
| PATCH | Only fixes, data updates (`MobInfo.lua`, skill values) and documentation. |

**2.0.0 is the first release of this series.** The original AzzyAI ended at 1.56, so
the new series starts a major number higher. Everything in the 2.0.0 entry of
[CHANGELOG.md](../CHANGELOG.md) is part of it.

**File check.** `AI.lua` and `AI_M.lua` read the `MAJOR.MINOR` out of `CVersion` and
compare it with three more numbers, to detect files copied from different installations.
If one differs, `File version error` is written in the startup report. The three are
`Version` (in `AI.lua` and `AI_M.lua`), `MainVersion` (`AI_main.lua`) and `AUVersion`
(`AzzyUtil.lua`), and each holds just the `MAJOR.MINOR`: `"2.0"` for 2.0.x, `"2.1"` for
2.1.x. The original AzzyAI used `"1.56"` in all four, so mixing old and new files is
reported instead of failing in strange ways. Raise the three together when MAJOR or MINOR
changes, never for a PATCH; `azzy-harness` fails if one does not match `CVersion`.

The desktop app shows `CVersion` itself in **Help > About**, read from the loaded USER_AI, so
there is no second copy to keep in sync. The web page has no number of its own.

To publish a release: change `CVersion`, the three file-check numbers if MAJOR or MINOR
changed, and the "Current release" line in `CHANGELOG.md`, add the changes to
`CHANGELOG.md`, run the harness, commit, and tag the
commit `v<number>` (for example `v2.0.1`). The CI rebuilds and commits
`USER_AI/AzzyAIConfig.exe` on every push to `main`, so a local
`cargo build --release -p azzy-native --manifest-path tools/Cargo.toml` is only needed to test the app before committing. The tag run builds
`AzzyAI.zip` and publishes it as a release, using the matching `## <version>` section of
`CHANGELOG.md` as the release body — that section is the release note, so write it for
the player.

## Project and compatibility background

This is a community continuation of [SpenceKonde/AzzyAI](https://github.com/SpenceKonde/AzzyAI).
Original project website: [drazzy.com](http://drazzy.com/ro/ai).

## Organization

* `DEFAULT` is the stock (original client) AI, kept to compare against what this version
  changes. It is **not** in the release ZIP: the game client already has those files in `AI\`,
  and the install only copies into `AI\USER_AI`.
* `USER_AI` contains AzzyAI
* `webgui` is a git submodule with the browser configuration GUI ([jknpj/AzzyAI-webgui](https://github.com/jknpj/AzzyAI-webgui)). Clone with `git clone --recursive`, or run `git submodule update --init` in an existing clone.

## Key Features

* **Server profiles:** `ServerType` selects official/private actor identification and skill data. This supports multiple server families, but individual clients and server revisions still require validation.
* **Smart Skill Configuration (Linting):** On startup, the AI intelligently validates the user's `H_Config.lua` file. It automatically disables skills that do not belong to the current homunculus type and adjusts skill levels if they are set higher than what the homunculus has actually learned. This prevents errors and simplifies configuration.
* **Updated GUI:** Includes a new updated GUI (Thanks to [samufacanha2](https://github.com/samufacanha2/AzzyAI)).
* **Skills updated to new max-level:** Reflects [official kRO changes](http://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=6843&curpage=1).
* **Updated Skill Data:** `H_SkillList.lua` records maximum levels, SP costs, ranges, durations, cooldowns and area information from the sources described below. These profiles are reference data, not a guarantee that every server uses identical values.

## Included Skills

This includes the following skills (Thanks to [SpenceKonde](https://github.com/SpenceKonde/AzzyAI)):
    * Xeno Slasher
    * Eraser Cutter
    * Stahl Horn
    * Heilige Stange
    * Needle of Paralyze
    * Pain Killer
    * Lava Slide
    * Pyroclastic
    * Silvervein Rush
    * Midnight Frenzy

Additionally, with the arrival of 4th jobs and new homunculus skills, the following new skills have been added:
    * Tempering (Thanks to [Kisaro](https://github.com/Kisaro/AzzyAI))
    * Blast Forge (Thanks to [vorpal1337](https://github.com/vorpal1337))
    * Twister Cutter
    * Absolute Zephyr
    * Light of Regenerate (optional autobuff, off by default: `UseEiraLightOfRegene`, `EiraLightOfRegeneLevel`, `EiraLightOfRegeneOwnerHP`). Attempts to maintain the resurrection buff; successful resurrection still depends on server conditions. It needs Loyal intimacy and lowers intimacy on every cast.
    * Blazing And Furious
    * The One Fighter Rises
    * Toxin Of Mandara
    * Needle Stinger
    * Glanzen Spies
    * Heilige Pferd
    * Goldene Tone

## Tests & CI

While a requested cast is within its expected casting time or the client reports
a casting motion, automatic movement, attacks and skill
requests pause to avoid cancelling the cast; explicit player commands still work.
A cast ending before its expected completion is not assumed successful without
SP confirmation. This does not prevent interruptions from incoming damage.

Homunculus offensive casts now back off per skill after `CastInterruptLimit`
confirmed consecutive interruptions (default 3); `CastInterruptPause` controls
the pause in milliseconds (default 5000). Set the limit to 0 to disable it.
Both settings are in AutoSkill Options in the desktop and web GUIs and can be
overridden in `H_Extra.lua`. Only casts observed starting and then ending early
with damage evidence and a living, in-range target count; successful casts reset
the counter. Buffs, combos, manual skill requests and unconfirmed refusals do not
trigger the pause. Other enabled usable skills or normal attacks can be selected
during the pause, and the paused skill returns automatically afterward.
Counters and pauses reset when the client reloads the AI; server cooldowns and
skill mechanics are unchanged.

`CastInterruptUnconfirmed` (default 0 / disabled) optionally includes offensive
failures without an observed cast or confirmed completion, provided the target
is alive and in range and SP is sufficient. This can include server refusals:
the client API cannot reliably distinguish them from hidden interruptions.
It uses the same limit/pause and is available in both GUIs. With this option
enabled, a gap between AI
calls alone is not completion evidence for a cast never observed starting.

### Native desktop GUI (Rust)

The desktop configuration app is `tools/azzy-native`, a Rust/egui window that mirrors the web GUI.
The web page stays the reference for the schema, translations, skill data and preset templates:
The web GUI publishes the schema, translations, skills and monster data as JSON under
`webgui/data/` (`node tools/dump-data.js` inside `webgui/`); the native window embeds those files, so the two
never drift.

Build and test from the repository root (Rust stable):

```
cargo test -p azzy-config -p azzy-update -p azzy-gui -p azzy-native --manifest-path tools/Cargo.toml
cargo build --release -p azzy-native --manifest-path tools/Cargo.toml
```

The window ships as `USER_AI/AzzyAIConfig.exe`, the name the old C# app used, so existing
shortcuts keep working; the development build is `tools/target/release/azzy-native.exe`. Run it
with the USER_AI folder, or put it inside USER_AI and run it without arguments:

```
tools\target\release\azzy-native.exe USER_AI
tools\target\release\azzy-native.exe USER_AI --new
```

Save writes the files themselves. `--new` runs in sandbox mode and writes `<file>.new` instead,
for testing a change without touching the player's configuration.

**Update AI from GitHub...** downloads the official `main` zip, keeps the files the player
edits, backs up the replaced ones under `USER_AI/backup/update-<time>/` and stages a newer
executable as `.exe.new`; **Restart now** swaps it. The startup check compares the latest release
tag with `CVersion` and offers an Update banner.

CI builds the Windows executable, runs the Rust tests, packages the ZIP (`USER_AI` with the freshly
built `AzzyAIConfig.exe`, `docs/`, `LICENSES/`, `webgui/` and the READMEs) and, on pushes to
`main`, commits `USER_AI/AzzyAIConfig.exe` back when it differs (short `build:` commit with
`[skip ci]`). Never write `[skip ci]` in the message of a commit that must run CI: GitHub looks for
the marker anywhere in the message and silently skips the workflow.

The Rust workspace is `tools/Cargo.toml`: `azzy-config` reads and writes the Lua files preserving
formatting, `azzy-update` is the updater shared with the local web GUI host, and `azzy-gui` serves
the web GUI locally for development.

The AI can't run outside the RO client, so `tools/harness/` contains an offline simulator: it mocks the client API (`GetV`, `GetActors`, `SkillObject`, ...), loads the real `USER_AI` files with a fixed config (`tools/harness/fixtures/H_Config.lua`, and `fixtures/M_Config.lua` for the mercenary scenarios) and runs scenarios (`tools/harness/scenarios.lua`) for every homunculus S and for a mercenary, checking that:

- expected skills are cast and forbidden ones aren't;
- no skill is cast without enough SP or before its cooldown;
- the homunculus still does normal attacks;
- official/private server overrides are applied;
- casts the server would refuse stay within `maxRefused`, also across a simulated map change (`reloadAt`).

A file under `fixtures/Types/` replaces the matching `Types/<homunculus>.lua` in the sandbox, so the scenarios always exercise the generic presets even when a branch ships a tuned one.

Combo follow-ups wait for the previous cast's result. Dance movement pauses during the combo window, and a failed follow-up clears the chain without assuming that all spheres are gone.
Confirmed Style Change casts update the tracked style whether requested manually or automatically; `eleanor_manual_style` verifies a manual switch followed by the grapple chain.
`SphereTrackInterval` (advanced option in both GUIs under AutoSkill Options, default 800 ms) controls how often fighting-style attack attempts count toward the estimate. GUIs save it in `H_Config.lua`; an `H_Extra.lua` override takes precedence. Use a positive integer and a shorter interval only when the homunculus actually attacks faster; underestimating its attack delay can cause refused skills. The max-level rAthena fixture tests 100 ms attacks with a conservative 200 ms estimate interval. Both GUIs allow `AutoSkillDelay=0` to remove the extra AI margin without removing server cooldowns; the built-in default remains 400 ms (the shipped `H_Config.lua` uses 200 and `M_Config.lua` uses 400).


```
cargo run --release --manifest-path tools/Cargo.toml -p rathena-sync --              # latest master
cargo run --release --manifest-path tools/Cargo.toml -p rathena-sync -- --ref <sha>  # a specific commit
cargo run --release --manifest-path tools/Cargo.toml -p rathena-sync -- --fixture-only  # only fixtures/rathena.lua
```

`tools/rathena-sync` is the regeneration in Rust: `cargo run --release --manifest-path tools/Cargo.toml -p rathena-sync -- [--ref <sha>] [--fixture-only]`; the Rust tool crates share the `tools/Cargo.toml` workspace and `tools/target/`. It parses the databases, walks the spawn scripts and writes the ~2.7k-monster files about 5x faster than the old Python implementation (12 s against 69 s from here, most of it network), which was removed with the other Python tools; CI runs the Rust binary.

Run locally (needs Lua 5.1 on PATH, or `LUA`/`LUAC` env vars):

```
cargo run --release -p azzy-harness --manifest-path tools/Cargo.toml --              # static checks (below) + all scenarios
cargo run --release -p azzy-harness --manifest-path tools/Cargo.toml -- amistr_fresh  # only some scenarios
cargo run --release -p azzy-harness --manifest-path tools/Cargo.toml -- --verbose --trace amistr_fresi
cargo run --release -p azzy-harness --manifest-path tools/Cargo.toml -- --skip-syntax --profile crowd_bayeri --ticks 600
```

Before the scenarios, `azzy-harness` runs the static checks and fails the run when one of them breaks:
syntax of every `.lua` file; the release number against `CHANGELOG.md`; and **every option in
`Defaults.lua`, `H_Config.lua` and `M_Config.lua` must be
read by some AI file** (`options: N in ..., M unread`). That last one exists because an option
nothing reads is a switch the player can change with no effect at all, which happened several
times (the `Berzerk_*` mis-spelling of the mercenary options, `KiteParanoid`, `MoveStickyFight`).
`--skip-syntax` skips these three checks for the benchmark runs.

`--profile` reports time per `AI()` call, the hottest functions (sampled), `GetV` calls per tick by type, Lua heap after GC and global tables that keep growing (leak check). The `crowd_*` scenarios put 40 monsters on screen to stress the per-tick loops. Absolute times are relative to the mock, not the RO client.

### Performance

Per-tick time was compared by running the same harness against two copies of `USER_AI`,
alternating them on the same machine and repeating the pairs (median reported). Absolute times are
relative to the mock: the client's `GetV` is a native call, so the mock understates the cost of AI
code that queries it. Gameplay is unchanged in every comparison below (same attacks and casts).

Against the pre-2.0 AI (`legacy/classic`, CVersion 1.552), which is the state before the 2.0 rework:

| Scenario | 1.552 | 2.0.1 | Change |
|---|---|---|---|
| `amistr_fresh`, 3000 ticks | ~0.404 ms | **~0.242 ms** | **-40%** |
| `amistr_fresh_single`, 2000 ticks | ~0.236 ms | **~0.139 ms** | **-41%** |
| `GetV` per `AI()` call (fresh / single) | 270 / 147 | **57 / 40** | **-79% / -73%** |

The old AI queried `V_HOMUNTYPE` on every `IsHomun` check (122 of the 270 `GetV` calls in
`amistr_fresh`), including once per actor in the actor loop. The current one remembers its own
answer (`IsHomunSelf`), hoists it in the loop and reuses resolved tactics, which is where most of
the reduction comes from.

Against the 2.0.0 release (tag `v2.0.0`), which is the state before the 2.0.1 performance work:

| Scenario | 2.0.0 | 2.0.1 | Change |
|---|---|---|---|
| `crowd_dieter`, 3000 ticks | 1.640 ms | **1.414 ms** | **-14%** |
| `eleanor_pack_aoe`, 3000 ticks | 0.392 ms | **0.342 ms** | **-13%** |
| Idle probe (20 ignored monsters, 400 ticks) | 0.248 ms | **0.149 ms** | **-40%** |
| `GetV` per call (crowd / idle) | 220 / 43 | 219 / 42 | ~equal |

The 2.0.1 gain comes from doing less work per call rather than making fewer API calls: resolved
tactics cached per tick, `TACT_CAST` skipped for ignored targets, per-tick tables allocated instead
of cleared, a trimmed actor loop and `TraceAI` arguments that are not built while the traces are off
(see Developer traces below).

### Damage model

On private-server scenarios the harness also prices every accepted cast and normal attack, so a change in what the AI casts can be compared by expected damage and DPS:

```
set HARNESS_DAMAGE=1                      # PowerShell: $env:HARNESS_DAMAGE='1'
cargo run --release -p azzy-harness --manifest-path tools/Cargo.toml -- --skip-syntax eira_dummy
```

`damage.lua` is a Lua port of rAthena's Renewal pipeline for a homunculus attacking a monster: `status_calc_homunculus_`/`status_calc_misc` (ATK, MATK, hit), `battle_calc_weapon_attack` and `battle_calc_magic_attack` (skill ratio, RES/MRES, DEF/MDEF, element, hit count) and `is_attack_hitting`. The skill ratios come straight from each skill's `calculateSkillRatio()` in `src/map/skills/homunculus/*.cpp` (translated to Lua by `rathena-sync`), and the skill types, elements, hit counts and monster stats from `skill_db.yml` and `mob_db.yml`, all in `fixtures/rathena.lua`. `cargo run --release --manifest-path tools/Cargo.toml -p rathena-sync -- --ref <sha> --fixture-only` regenerates only that file.

It is an expected-value model: ATK and MATK use their mean, there are no critical hits, and monsters without rAthena data (`{ x, y }`) count as the test server's training dummy (a Neutral Poring with 0 DEF, MDEF and FLEE). A scenario monster can override fields as `{ x, y, mobId, { def = 0, element = "Neutral", elv = 1 } }`, and `stats = { ... }` replaces the homunculus profile. `fixtures/homun_stats.lua` holds the level 275 stats of the local test server's homunculi (not an rAthena default), `castRatio` sets the server's own cast reduction, and `HarnessDamage` is available to scenario checks.

Assumptions and limits: the server's `hom_setting` makes Min-MATK equal Max-MATK (`homunc.conf` 0x3D); Sonic Claw and Blazing And Furious hit once per spirit sphere left after their cost; ground units (Lava Slide, Blast Forge, Poison Mist) are priced as Duration / Interval hits on every monster in the area for the whole duration. Not modelled: status effects (Bleeding, Stun, Tinder Breaker's DEF reduction), buffs, the owner's skills and monster attacks. `damage_model` checks the formulas against values worked out by hand from the rAthena source, and the same scenario compares them with the local benchmark (`log/azzy-benchmark.csv`): Sonic Claw averaged 9.3 spheres at 8,150 per sphere, and Xeno Slasher's 186,330 per cast is 0.46% above the model's 185,478. `eira_dummy` simulates that benchmark with the test client's configuration: 121,472 expected DPS over 60 s against 116,584 measured; the measured session also had four Twister Cutter casts, which fits the cast backoff pausing Xeno Slasher at its start (not confirmed: the detailed log was off). Still unexplained: Twister Cutter's measured 99,077 per cast is exactly half of the model's 197,254, and a later Xeno Slasher session averaged 113,128 per cast, only 61% of the expected damage per cast.

### Default preset tuning


<!-- preset-table:start -->

| Homunculus | 1 monster | 2 | 3 | 5 | Individual settings (`Types/<Name>.lua`) |
|---|---|---|---|---|---|

<!-- preset-table:end -->



What this does not prove: the stats are hypothetical and the real DPS depends on the server (the RO LATAM homunculi are much stronger than rAthena's), so the ranking of two configurations can change with the real stats and SP; the model is validated against measured damage only for Sonic Claw and Xeno Slasher, the SP regeneration of the other homunculi is not measured (their decisions held from 30 to 400 SP/s), `AutoSkillDelay` made no difference in these runs, and none of it was run on an official server, whose skill data differ (`ServerType` stays `SERVER_OFFICIAL`). The GUIs' own defaults and `Defaults.lua` were not changed. Two corrections to the damage model came out of this work: Poison Mist's unit only applies a status (it deals no damage; later corrected: it hits with magic damage every tick) and Tinder Breaker, CBC and EQC add a flat 2,500, 4,000 and 6,000 per level after DEF and element.

In a scenario, `"HLIF_HEAL:3"` means the skill is learned at Lv2; the mock server refuses casts above the learned level. `probeFiller = { "MH_LAVA_SLIDE" }` makes the simulated client answer the "not learned" filler for that skill (range + 1) and keeps `TrustSkillProbe` at the official default, which is how `dieter_official_probe_filler` reproduces the LATAM report.

### Learned skill levels

The AI reads the learned level of each homunculus skill with `GetV(V_SKILLATTACKRANGE_LEVEL)`. For skills whose low levels all have range 1 (Toxin of Mandara, Heilige Pferd, The One Fighter Rises Lv1-4, Moonlight) the client can't tell the learned level apart, so the AI:

- never casts above the configured level or the highest level the client confirms;
- lowers the level by one after 2 refused casts at it (never below a level that already worked);
- saves that level in `data/H_<owner>Skills.lua`, so map changes don't start over;
- tries one level higher at most once per hour (`LEARNED_SKILL_RECHECK_SECONDS`), in case the skill was levelled.

The official client answers the probe in its own way: a learned skill reports its table range, an
unlearned one reports range + 1 (or a flat 8 where the table range is off), and the client's value
can differ from the AI's table. The LATAM client answered 8 for every level of Lava Slide, which
the AI read as "not learned": `LintSkillOptions` disabled `UseDieterLavaSlide` at startup and
`SERVER_OFFICIAL` the probe is not trusted to rule a skill out: an enabled skill starts at the
configured level and the step-down above is what finds the real level, so the server is the only
authority. The level is saved, so the walk down happens once per owner. A skill that the server
keeps refusing at every level ends at 0 and is reported as "the server refused every level" in the
startup report; `TrustSkillProbe = 1` trusts the probe again (the behavior before 2.0.1), `0`
distrusts it on private servers too.

`SkillLevelProbing` (AutoSkill Options in both GUIs) is the blunt switch for a server whose probe
answers are wrong in both directions. `0` never asks the client: `GetSkillLevelCap` returns the
skill's maximum for the unit, `DoSkill` requests the configured level as-is and `SkillCastResult`
neither steps a level up nor down (`LintUserOptions` skips the level comparison too). The trade-off
is that a configured level above the learned one is refused and stays refused until the player
fixes the setting. The default is `1`, the behaviour above.

Skill cooldowns are also saved in `data/H_<owner>Timeouts.lua`, so the AI doesn't recast them right after a map change, including short cooldowns such as Toxin of Mandara.

The simulator follows rAthena's rules (`clif_parse_UseSkillToId_homun`, `skill_check_condition_mercenary`): homunculus/mercenary skills need **more** SP than their cost, so `DoSkill` never sends a skill at exactly its SP cost. rAthena silently lowers a level above the learned one, so the level step-down above only matters on servers that refuse such casts.

GitHub Actions (`.github/workflows/ci.yml`) runs on every push/PR:

- Lua syntax check and simulator;
- `node --check` on the web GUI (checked out from the `webgui` submodule);
- Rust tests and the Windows build of the native desktop GUI (`azzy-native`), and the `AzzyAI`
  artifact: `USER_AI` (without `.pdb`) with the freshly built `AzzyAIConfig.exe`, plus `docs/`,
  `LICENSES/`, `webgui/`, the three READMEs, `CHANGELOG.md`, `NOTICE.md`, `SECURITY.md` and
  `CONTRIBUTING.md`;
- on pushes to `main`, a follow-up job commits that build back to
  `USER_AI/AzzyAIConfig.exe` when it changed (short `build:` commit, `[skip ci]`).

Pushing a `v*` tag also publishes `AzzyAI.zip` as a GitHub release, with the matching
`## <version>` section of `CHANGELOG.md` as the release body.

## Configuration Guide

### Server Type Configuration
To ensure the AI functions correctly on your server, set the `ServerType` variable in `H_Config.lua` (and `M_Config.lua` for mercenaries).
* For **official servers**, set `ServerType = SERVER_OFFICIAL`.
* For **private servers**, set `ServerType = SERVER_PRIVATE`.

With the wrong value on a private server every monster looks like a player and the homunculus never attacks. To avoid that, `AutoDetectServerType = 1` (default) switches to `SERVER_PRIVATE` by itself as soon as it sees a monster with an rAthena/Hercules ID (110000000 or more) and logs it in `AAI_ERROR`. It is skipped if the config sets `MagicNumber`/`MagicNumber2`/`MagicNumber3`.

`ServerType` also selects the homunculus skill data. `H_SkillList.lua` holds the official values, targeting LATAM Episode 19 (Isgard, October 2026): cast times, after-cast delays, cooldowns, durations, AoE sizes and sphere costs come from the LATAM client data on Divine Pride (`cargo run --release --manifest-path tools/Cargo.toml -p divinepride-sync -- [--server LATAM] [--refresh]`; it needs a free API key in `DIVINEPRIDE_API_KEY` and writes a git-ignored JSON), with kRO's client text and bROWiki filling the few gaps. `ServerSkillOverrides` lists only the rAthena Renewal values that differ (a few ranges, cooldowns, durations, AoE sizes, CBC's sphere cost and EQC's after-cast delay) and replaces them under `SERVER_PRIVATE`. The comments above both tables record the sources, the audit date and the differences that were deliberately not copied.

### Monster Database
`USER_AI/MobInfo.lua` lists the race, element, element level, size, boss/MVP/plant/immobile flags and attack range of every monster, the element damage table and the element of each homunculus attack skill. It is generated from rAthena Renewal (`db/re/mob_db.yml`, `attr_fix.yml`, `skill_db.yml`) by `tools/rathena-sync`. The homunculus reads a monster's ID with `GetV(V_HOMUNTYPE)`, so it uses the database without any setup; mercenaries only know monster IDs from MobID files and are not affected.

* **Packs of mixed elements**: area skills count each monster by the damage its element lets through (an Earth 2 monster counts 1.75 against fire, a Fire 2 one 0), both to decide whether the skill is worth casting (`AutoMobCount`) and where to aim it. An area skill may still be used when the current target is immune, if the monsters around it make up for it; it is then aimed at them.
* **Plants** (`PlantSkills`, default 0): plants and mushrooms take 1 damage from everything, so no skills are used on them and they don't count for area skills.
* **Bosses** (`BossAggro`, default 1): the homunculus never starts a fight with an MVP (`2` = also mini-bosses), whatever the tactics say; it still defends you and itself when they attack. `0` lets the tactics decide.
* **Target priority** (`TargetPriority`, default 0): with `1`, the next target is chosen by more than distance. The tactics priority still decides first, with plants always last. Among targets of the same priority: monsters attacking you or a friend, then ones at least one of your elemental skills can hurt, then the one in the middle of the biggest group an area skill would hit (monsters counted like in *Packs of mixed elements*), then the closest. `0` keeps the old order (tactics priority, then closest).
* **Kiting by range** (`KiteByRange`, default 0, needs `KiteMonsters` and the monster's kite tactic): with `1`, kiting uses the monster's real attack range instead of a fixed distance. A melee monster still uses `KiteThreshold`; a ranged one (an archer with range 9) makes the homunculus start kiting at `KiteThreshold` + range − 1 cells and take a long enough step to leave that range. Monsters that never chase (immobile ones) only matter inside their range, and a monster whose range reaches `KiteBounds` is not kited at all, since there is no way out of it. Needs a regenerated `MobInfo.lua` (`cargo run --release --manifest-path tools/Cargo.toml -p rathena-sync --`); an older one has no ranges and the fixed thresholds stay.

`UseMobInfo = 0` turns all of this off. `AAIStartH.txt` shows whether the database was loaded. Tactics rows still apply first; the database only adjusts attacking tactics for bosses and the skill/weight of plants.

The web GUI uses the same rAthena data (`webgui/mobdata.js`, also written by `rathena-sync`, with the maps each monster spawns on) to suggest tactics rows: pick a map and it adds a row for every monster there, based on whether it is a plant, a boss, aggressive or unable to move.

### Troubleshooting automatic attacks

Some clients report the player's own homunculus/mercenary as a monster through `IsMonster()`.
The AI used to add itself to its target list, pick itself as the closest "monster" and send
attacks the server ignores, so the homunculus only walked around. The actor loop now never
treats the owner, the homunculus itself or friends/retainers as monsters.

If manual attacks work but automatic attacks still do not, enable targeting diagnostics by
adding `LogEnable["AAI_TARGETING"] = 1` to the installed `H_Extra.lua`. Reload the
custom AI (relog if unsure), stand near an untouched monster for 10-15 seconds,
and collect `AAI_TARGETING.log` and `AAIStartH.txt` from the Ragnarok client folder.
The targeting log records a snapshot every 5 seconds, with up to 20 actors:
native monster detection, player classification, target-list membership, tactics,
kill-steal checks, distance, and passive/standby/HP/SP settings.
If no log is created, open `AAIStartH.txt` (rewritten on every AI load). Its last lines show
the `Release`, the `Build`, the `ServerType` and `Logs enabled`. No `Release:` or `Build: main` line means the client
is still running older `USER_AI` files; `AAI_TARGETING` missing from `Logs enabled` means the
`H_Extra.lua` line is still commented out (`--`). If the file isn't updated at all, check the
custom-AI selection (`/hoai`) and folder write permissions (clients under `Program Files` may
write to `%LOCALAPPDATA%\VirtualStore` instead). Disable the option after testing; logs are appended.
The offline harness cannot verify what a particular client reports or whether the
server accepts AI-issued attack commands.

An unlearned defensive base-homunculus buff (such as Amistr Bulwark)
must not block following or normal attacks. This can first appear after a level-up
raises SP enough for the configured buff, or when SP equals its cost. Skipped
casts now leave the buff timer unchanged and let normal AI behavior continue;
the harness covers both cases at realistic client uptime.

### Skill Configuration
To use new skills, enable them and choose learned levels through a GUI or
`H_Config.lua`. Startup linting and learned-level checks catch supported cases;
they cannot diagnose every server refusal or configuration conflict.

### Developer traces
The client's `TraceAI` calls are developer diagnostics. They are **off by default**
(`TraceAIEnabled = 0` in `Const_.lua`): the arguments are not even evaluated, which saves a few
percent in combat. Set `TraceAIEnabled = 1` in `H_Extra.lua` to get them back; in the simulator,
`cargo run --release -p azzy-harness --manifest-path tools/Cargo.toml -- --trace <scenario>` enables them for the run.
