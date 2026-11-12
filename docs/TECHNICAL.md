# AzzyAI: technical reference

[User README](../README.md) | [Português (BR)](../README.pt-BR.md) | [Español](../README.es.md) | [Changelog](../CHANGELOG.md)

This document keeps the technical material previously in the README. It is
intended for maintainers and advanced users, not as the installation guide.
Paths and commands below are relative to the repository root unless stated
otherwise. The version history is in [CHANGELOG.md](../CHANGELOG.md).

## Version

The AI has its own release number, in [semantic versioning](https://semver.org/)
form (`MAJOR.MINOR.PATCH`). It is set once, as `CVersion` in `USER_AI/Const_.lua`,
and shown on the `Release:` line of `AAIStartH.txt` and `AAIStartM.txt` (the startup
reports), so a bug report can say exactly which version it is about. `python
tools\harness\run.py` fails if that number and the "Current release" line at the
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
changes, never for a PATCH; `run.py` fails if one does not match `CVersion`.

The Windows configuration app has the same number: `AssemblyVersion` and
`AssemblyFileVersion` in `GUI/AzzyAIConfig/Properties/AssemblyInfo.cs` are set to the
release, **Help > About** shows it, and `run.py` checks it. The web page has no number
of its own.

To publish a release: change `CVersion`, the three file-check numbers if MAJOR or MINOR
changed, the two numbers in `AssemblyInfo.cs` and the "Current release" line in
`CHANGELOG.md`, add the changes to `CHANGELOG.md`, run the harness, rebuild the app
(`tools\build-gui.cmd`), commit, and tag the commit
`v<number>` (for example `v2.0.0`). The CI builds the ZIP for tags that start with `v`.

## Project and compatibility background

This is a community continuation of [SpenceKonde/AzzyAI](https://github.com/SpenceKonde/AzzyAI).
Original project website: [drazzy.com](http://drazzy.com/ro/ai).

## Organization

* `DEFAULT` contains the default AI
* `USER_AI` contains AzzyAI
* `webgui` is a git submodule with the browser configuration GUI ([jknpj/AzzyAI-webgui](https://github.com/jknpj/AzzyAI-webgui)). Clone with `git clone --recursive`, or run `git submodule update --init` in an existing clone.

## Key Features

* **Server profiles:** `ServerType` selects official/private actor identification and skill data. This supports multiple server families, but individual clients and server revisions still require validation.
* **Automatic Homunculus Base Type Detection:** The AI now automatically detects the original base type (Lif, Amistr, Filir, Vanilmirth) of a Homunculus S. This ensures inherited skills from the original form are used correctly without requiring manual configuration of `OldHomunType`.
* **Smart Skill Configuration (Linting):** On startup, the AI intelligently validates the user's `H_Config.lua` file. It automatically disables skills that do not belong to the current homunculus type and adjusts skill levels if they are set higher than what the homunculus has actually learned. This prevents errors and simplifies configuration.
* **Updated GUI:** Includes a new updated GUI (Thanks to [samufacanha2](https://github.com/samufacanha2/AzzyAI)).
* **Skills updated to new max-level:** Reflects [official kRO changes](http://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=6843&curpage=1).
* **Updated Skill Data:** `H_SkillList.lua` records maximum levels, SP costs, ranges, durations, cooldowns and area information from the sources described below. These profiles are reference data, not a guarantee that every server uses identical values.

## Included Skills

This includes the following skills (Thanks to [SpenceKonde](https://github.com/SpenceKonde/AzzyAI)):
* **Eira**
    * Xeno Slasher
    * Eraser Cutter
* **Bayeri**
    * Stahl Horn
    * Heilige Stange
* **Sera**
    * Needle of Paralyze
    * Pain Killer
* **Dieter**
    * Lava Slide
    * Pyroclastic
* **Eleanor**
    * Silvervein Rush
    * Midnight Frenzy

Additionally, with the arrival of 4th jobs and new homunculus skills, the following new skills have been added:
* **Dieter**
    * Tempering (Thanks to [Kisaro](https://github.com/Kisaro/AzzyAI))
    * Blast Forge (Thanks to [vorpal1337](https://github.com/vorpal1337))
* **Eira**
    * Twister Cutter
    * Absolute Zephyr
    * Light of Regenerate (optional autobuff, off by default: `UseEiraLightOfRegene`, `EiraLightOfRegeneLevel`, `EiraLightOfRegeneOwnerHP`). Attempts to maintain the resurrection buff; successful resurrection still depends on server conditions. It needs Loyal intimacy and lowers intimacy on every cast.
* **Eleanor**
    * Blazing And Furious
    * The One Fighter Rise
* **Sera**
    * Toxin Of Mandara
    * Needle Stinger
* **Bayeri**
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

### Windows desktop GUI development

To build the app without PowerShell, run `tools\build-gui.cmd`: it builds the Release
x86 executable and copies it to `USER_AI\AzzyAIConfig.exe`, the copy that is committed and
that players run. The PowerShell commands below do the same build and also run and test the app.
The CI does this too: on a push to `main` a follow-up job commits the freshly built
`USER_AI/AzzyAIConfig.exe` when it differs, so a C# change never leaves the committed copy
behind (it commits with `[skip ci]`, and a byte-identical build is not committed).

The desktop app is a C# WinForms project targeting **.NET Framework 4.0 Client
Profile, x86**. Keep that target for compatibility; a modern .NET SDK alone
cannot build it.

Use an existing Visual Studio 2022 installation with .NET desktop development,
or install Build Tools with the .NET desktop build tools workload:

```powershell
winget install --id Microsoft.VisualStudio.2022.BuildTools --exact --source winget --override "--wait --passive --norestart --add Microsoft.VisualStudio.Workload.ManagedDesktopBuildTools --includeRecommended"
code --install-extension ms-dotnettools.csharp
```

Accept administrator prompts and restart VS Code after installation. The existing
.NET Framework 4.8 runtime runs this .NET 4.0 app; do not install the obsolete 4.0
runtime. NuGet restores the pinned 4.0 reference assemblies on the first build
(internet access to nuget.org is required). If C: is short on space, the installer
supports `--installPath D:\DevTools\VSBuildTools2022 --path cache=D:\DevTools\VSCache`,
but shared Windows components still need space on C:.

From the repository root:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\gui.ps1 -Action Build
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\gui.ps1 -Action Test
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\gui.ps1 -Action Run
```

Use `-Configuration Release` for a release build. Build outputs and intermediate
files go into ignored `.artifacts\gui`, not the legacy tracked `bin`/`obj` folders.
Run, Test, and F5 build the app and copy the current `USER_AI` configuration into
a disposable sandbox. **Each launch refreshes these copies; Apply never writes
to the source `USER_AI` files.** Close the sandbox GUI before launching again.
The smoke test opens/closes the form, visits all seven tabs, checks configuration
bindings, and verifies that startup did not change the copied Lua files.

The Homunculus editor includes official/private server selection, newer
Homunculus S skills, Eleanor's grappling switches, and inherited Blood Lust/SBR44
options. Skill selectors support the updated level-10 caps where applicable.
Search matches names and descriptions; the type filters only hide controls,
not disable skills. Use **Clear filters** to return to all settings. The layout
resizes with the window, and the footer shows which configuration is being edited.
Apply saves changes; Quit or the window close button prompts for unsaved edits.
The smoke test also checks resizing, filtering, level limits, imports/defaults,
and saving/reloading settings without changing unedited values.

To test the compiled desktop GUI in a game client, close any existing config
window and back up its executable, then copy only
`.artifacts\gui\Release\AzzyAIConfig.exe` into that client's `AI\USER_AI` folder.
Run it there to edit the installed configuration. The GUI always uses its own
executable directory, even when launched from a shortcut with another working
directory. Copying the executable does not replace the client's Lua files;
clicking Apply does save configuration changes, so back those up before editing.

In VS Code, use **Ctrl+Shift+B** to build, **F5** to debug, or **Tasks: Run Task**
for `GUI: Run` / `GUI: Smoke test`. The workspace enables OmniSharp for this
legacy project, using its .NET Framework host so no additional .NET SDK is
required; C# Dev Kit is not required. WinForms visual designer support
requires full Visual Studio with .NET desktop development, not VS Code or
Build Tools. This setup does not add a designer.

The AI can't run outside the RO client, so `tools/harness/` contains an offline simulator: it mocks the client API (`GetV`, `GetActors`, `SkillObject`, ...), loads the real `USER_AI` files with a fixed config (`tools/harness/fixtures/H_Config.lua`, and `fixtures/M_Config.lua` for the mercenary scenarios) and runs scenarios (`tools/harness/scenarios.lua`) for every homunculus S and for a mercenary, checking that:

- expected skills are cast and forbidden ones aren't;
- no skill is cast without enough SP or before its cooldown;
- the homunculus still does normal attacks;
- official/private server overrides are applied;
- casts the server would refuse stay within `maxRefused`, also across a simulated map change (`reloadAt`).

When automatic Eleanor combos are enabled, their starters wait for `AutoComboSpheres`, so Sonic Claw does not spend the spheres needed for its follow-ups. With automatic combos disabled, starters only require their normal sphere cost. New skills retain their priority and can consume spheres before a combo starts.
Combo follow-ups wait for the previous cast's result. Dance movement pauses during the combo window, and a failed follow-up clears the chain without assuming that all spheres are gone.
Confirmed Style Change casts update the tracked style whether requested manually or automatically; `eleanor_manual_style` verifies a manual switch followed by the grapple chain.
`SphereTrackInterval` (advanced option in both GUIs under AutoSkill Options, default 800 ms) controls how often fighting-style attack attempts count toward the estimate. GUIs save it in `H_Config.lua`; an `H_Extra.lua` override takes precedence. Use a positive integer and a shorter interval only when the homunculus actually attacks faster; underestimating its attack delay can cause refused skills. The max-level rAthena fixture tests 100 ms attacks with a conservative 200 ms estimate interval. Both GUIs allow `AutoSkillDelay=0` to remove the extra AI margin without removing server cooldowns; the default remains 400 ms.

The simulated server follows rAthena's rules (`skill_isNotOk_hom`, `unit_skilluse_id2`, `skill_check_condition_mercenary`): a cast is refused when the level isn't learned, the skill is still in cooldown (counted from the end of the cast), the target is beyond the skill range (square distance, the homunculus doesn't walk into range), Goldene Ferse and Angriffs Modus overlap, or Eleanor lacks the style, combo or spirit spheres (one sphere every second hit dealt in fighting style or taken in grappling style, at most 10; the homunculus swings once per 800 ms, however often the AI asks for an attack; costs from `SkillSphereCost`, or rAthena's on private scenarios). Eleanor's fighting-style sphere estimate credits at most one normal attack attempt per 800 ms, scaled by `SphereTrackFactor`; it does not require the client to expose her attack animation. This is an estimate, not a read of the server's sphere count; refused sphere skills correct it downward. `eleanor_hidden_motion` tests a client that hides those animations. On `SERVER_PRIVATE` scenarios its SP costs, cast times, delays, cooldowns and ranges come from rAthena's `db/re/skill_db.yml`; on official ones from AzzyAI's official profile. The rAthena data lives in `tools/harness/fixtures/rathena.lua`, generated from a pinned rAthena commit and committed so the harness needs no network. It also gives scenarios real monsters (`{ x, y, mobId }`: aggressive ones attack the owner, passive ones fight back, plants never move) and lets `rathena_skill_tree` check `SkillList`/`SkillInfo` against `db/re/homunculus_db.yml`. To move to a newer rAthena (standard library only; behind a proxy set `HTTPS_PROXY` first):

```
python tools/harness/rathena_sync.py              # latest master
python tools/harness/rathena_sync.py --ref <sha>  # a specific commit
python tools/harness/rathena_sync.py --fixture-only  # only fixtures/rathena.lua (no MobInfo.lua / mobdata.js)
```

Run locally (needs Lua 5.1 on PATH, or `LUA`/`LUAC` env vars, and Python 3):

```
python tools/harness/run.py              # static checks (below) + all scenarios
python tools/harness/run.py dieter sera  # only some scenarios
python tools/harness/run.py --verbose --trace bayeri
python tools/harness/run.py --skip-syntax --profile crowd_bayeri --ticks 600
```

Before the scenarios, `run.py` runs the static checks and fails the run when one of them breaks:
syntax of every `.lua` file; the release number against `CHANGELOG.md` and the GUI's
`AssemblyInfo.cs`; and **every option in `Defaults.lua`, `H_Config.lua` and `M_Config.lua` must be
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

Against the pre-2.0 AI (`classic`, CVersion 1.552), which is the state before the 2.0 rework:

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
python tools/harness/run.py --skip-syntax eira_dummy
```

`damage.lua` is a Lua port of rAthena's Renewal pipeline for a homunculus attacking a monster: `status_calc_homunculus_`/`status_calc_misc` (ATK, MATK, hit), `battle_calc_weapon_attack` and `battle_calc_magic_attack` (skill ratio, RES/MRES, DEF/MDEF, element, hit count) and `is_attack_hitting`. The skill ratios come straight from each skill's `calculateSkillRatio()` in `src/map/skills/homunculus/*.cpp` (translated to Lua by `rathena_sync.py`), and the skill types, elements, hit counts and monster stats from `skill_db.yml` and `mob_db.yml`, all in `fixtures/rathena.lua`. `python tools/harness/rathena_sync.py --ref <sha> --fixture-only` regenerates only that file.

It is an expected-value model: ATK and MATK use their mean, there are no critical hits, and monsters without rAthena data (`{ x, y }`) count as the test server's training dummy (a Neutral Poring with 0 DEF, MDEF and FLEE). A scenario monster can override fields as `{ x, y, mobId, { def = 0, element = "Neutral", elv = 1 } }`, and `stats = { ... }` replaces the homunculus profile. `fixtures/homun_stats.lua` holds the level 275 stats of the local test server's homunculi (not an rAthena default), `castRatio` sets the server's own cast reduction, and `HarnessDamage` is available to scenario checks.

Assumptions and limits: the server's `hom_setting` makes Min-MATK equal Max-MATK (`homunc.conf` 0x3D); Sonic Claw and Blazing And Furious hit once per spirit sphere left after their cost; ground units (Lava Slide, Blast Forge, Poison Mist) are priced as Duration / Interval hits on every monster in the area for the whole duration. Not modelled: status effects (Bleeding, Stun, Tinder Breaker's DEF reduction), buffs, the owner's skills and monster attacks. `damage_model` checks the formulas against values worked out by hand from the rAthena source, and the same scenario compares them with the local benchmark (`log/azzy-benchmark.csv`): Sonic Claw averaged 9.3 spheres at 8,150 per sphere, and Xeno Slasher's 186,330 per cast is 0.46% above the model's 185,478. `eira_dummy` simulates that benchmark with the test client's configuration: 121,472 expected DPS over 60 s against 116,584 measured; the measured session also had four Twister Cutter casts, which fits the cast backoff pausing Xeno Slasher at its start (not confirmed: the detailed log was off). Still unexplained: Twister Cutter's measured 99,077 per cast is exactly half of the model's 197,254, and a later Xeno Slasher session averaged 113,128 per cast, only 61% of the expected damage per cast.

### Default preset tuning

`USER_AI/H_Config.lua` was tuned with `python tools/harness/homun_bench.py <homunculus> [--mobs N] [--spregen SP/s] "<overrides>" ...`, which runs the `hbench_*` scenarios (180 s against a zero-defense Neutral dummy, or a pack of up to five) on top of the shipped preset and prints expected DPS, damage per SP and the casts of each configuration. For Eira, Bayeri and Sera the AI scores its enabled attacks (see "Skill choice by score" below); other skills follow the skill-class priority, so the preset turns skills on or off and sets the area threshold. **Read the numbers as a comparison between configurations, not as the DPS you will see in game.** The simulation uses hypothetical homunculus stats (`fixtures/homun_stats.lua`, level 275) and rAthena's skill data, and the real DPS depends on the server: the RO LATAM homunculi are much stronger than rAthena's, so absolute values, SP pools and SP regeneration differ, and a configuration that wins here can lose there when SP or sphere counts change the balance. Each homunculus has its own settings: `USER_AI/H_Config.lua` is the shared base and `USER_AI/Types/<Name>.lua` overrides only what differs. The table is the previous preset (`H_Config.lua` alone) against those current individual settings, with no overrides, expected DPS over 180 s and the change against the previous preset, at 60 SP/s (Eleanor: 22 SP/s). `python tools/harness/preset_report.py --old-ref 0cc1f35` regenerates it (and `docs/preset-tuning/`) in place:

<!-- preset-table:start -->

| Homunculus | 1 monster | 2 | 3 | 5 | Individual settings (`Types/<Name>.lua`) |
|---|---|---|---|---|---|
| Eira | 126,909 (+0%) | 250,482 (+3%) | 373,183 (+2%) | 618,585 (+2%) | `UseEiraTwisterCutter` 1, `UseEiraAbsoluteZephyr` 1 |
| Bayeri | 184,809 (+0%) | 184,809 (+0%) | 226,169 (+18%) | 390,505 (+37%) | `UseBayeriHeiligePferd` 1, `AutoMobCount` 2, `UseBayeriGlanzenSpies` 1 |
| Sera | 219,256 (+0%) | 318,949 (+0%) | 419,754 (+0%) | 621,387 (+0%) | `UseSeraNeedleStinger` 1, `AutoMobCount` 1 |
| Dieter | 45,841 (+464%) | 90,616 (+522%) | 135,390 (+552%) | 225,190 (+569%) | `AutoMobCount` 1, `UseDieterAshAsAttack` 0 |
| Eleanor | 100,393 (+25%) | 120,869 (-13%) | 213,518 (+0%) | 345,750 (+0%) | `UseEleanorTheOneFighterRises` 1, `UseEleanorBlazingAndFurious` 1, `AutoMobCount` 2 |

<!-- preset-table:end -->

- **`AutoMobCount` 1 → 2 in the shared preset.** At 1 an area skill outranks the single-target ones even against one monster. Eira's DPS is the same but Xeno Slasher (130 SP) is replaced by Eraser Cutter (70 SP), so damage per SP goes from 1,438 to 2,670; Sera against one monster gains 14% (146,405 → 167,256 DPS). Bayeri no longer changes either way: his DEF-ignoring area skills are only cast against a target with enough DEF, so he casts Stahl Horn at 1 and 2 monsters with either value. The area skill is used when the monsters around the target number at least `AutoMobCount`.
- **One shared value would hurt Dieter against a single monster, so he has his own.** Every Dieter attack is an area skill, so with `AutoMobCount` 2 he casts nothing offensive against one monster (2,192 DPS). `Types/Dieter.lua` sets `AutoMobCount = 1`: 45,841 DPS against one monster and 90,616 against two, against 8,135 and 14,577 with the previous preset (his old figures of 23,000 and 54,786 no longer reproduce after the AI changes). See [the option table](preset-tuning/README.md#dieter-one-option-at-a-time) for the rows.
- **Skill choice by score.** Eira, Bayeri and Sera no longer cast a newer attack just because it is enabled. Each attack has its damage data in `SkillDamage` (`H_SkillList.lua`: rAthena's ratio at base level 250 and whether DEF applies), and among the ready skills the AI picks the highest score: ratio × element against the target × monsters hit × DEF reduction, divided by the cast time; below `SkillPickLowSPPercent` (40%) SP it divides by the SP cost instead. On private servers the 1 s after-cast delay of the newer attacks is gone (rAthena has none), and their cooldown now starts after the observed cast, so they are not retried too early. Previously, forcing the newer skill lost DPS: Twister Cutter instead of Eraser Cutter 83,372 against 122,526 DPS on one monster, Absolute Zephyr instead of Xeno Slasher 250,881 against 365,708 on three. With the score the presets enable them: Eira 126,909 on one monster and 373,183 on three, Sera with Needle Stinger and `AutoMobCount` 1 219,256 on one monster (167,256 before). Glanzen Spies is on in `Types/Bayeri.lua` and replaces Stahl Horn only where it scores higher. With the official server type the 1 s delay stays, so Xeno Slasher, Heilige Stange and Needle of Paralyze usually win.
- **Blast Forge on** is most of Dieter's damage: turning it off costs 82-85% (90,616 → 14,577 DPS against two monsters), since it has its own cooldown next to Lava Slide. Its ground unit is priced as hitting every monster in the area on every tick, so a monster that walks out of the area earns less. Magma Flow is priced as one splash hit per cast and its in-game effect, a buff that triggers on attacks, is not modelled, so its rows (turning it off looks like +4% against three monsters) are not reliable.
- **Eleanor.** `Types/Eleanor.lua` keeps Blazing And Furious on: the AI only casts it with 6 or more spheres (`ELEANOR_BLAZING_MIN_SPHERES` in `AzzyUtil.lua`), so against one monster it is never cast (identical rows with it off) and against packs it adds 5-7% (213,518 against 199,288 DPS at 22 SP/s against three, 345,750 against 328,535 against five, 300,606 against 283,592 at 40 SP/s against three). The One Fighter Rises is what makes packs worth it: turning it off loses 52% against three monsters and changes nothing against one. `AutoComboSpheres` of 3 or 6 instead of the current 1 is within 1% against one monster and no different against three. Open: against two monsters the current settings are 15% below the previous preset (118,183 against 139,502 DPS); the casts differ (Sonic Claw 37, The One Fighter Rises 28, Blazing And Furious 3, against 14, 28 and 13 before). `AutoMobCount` is not the cause (1 and 2 give identical rows with two monsters). The previous preset also had `AutoComboSpheres` 10 where the current one has 1, which is the likely cause and has not been tested. The 22 SP/s regeneration is the one input measured on a single server.

The complete comparison is in [`docs/preset-tuning/`](preset-tuning/README.md): the previous preset against the current settings (shared preset plus each `Types/` file) for every homunculus with 1, 2, 3 and 5 monsters, and rows with one option changed at a time (DPS, damage per SP, refused casts and the casts of each run). The same data is in two spreadsheets, [preset-before-after.csv](preset-tuning/preset-before-after.csv) and [preset-options.csv](preset-tuning/preset-options.csv). `tools\update-benchmarks.cmd` (or `python tools/harness/preset_report.py --old-ref 0cc1f35`) regenerates all three files and the table above in under a minute (`--old-ref <sha>` selects the preset to compare against; the tables here compare against `0cc1f35`).

What this does not prove: the stats are hypothetical and the real DPS depends on the server (the RO LATAM homunculi are much stronger than rAthena's), so the ranking of two configurations can change with the real stats and SP; the model is validated against measured damage only for Sonic Claw and Xeno Slasher, the SP regeneration of the other homunculi is not measured (their decisions held from 30 to 400 SP/s), `AutoSkillDelay` made no difference in these runs, and none of it was run on an official server, whose skill data differ (`ServerType` stays `SERVER_OFFICIAL`). The GUIs' own defaults and `Defaults.lua` were not changed. Two corrections to the damage model came out of this work: Poison Mist's unit only applies a status (it deals no damage; later corrected: it hits with magic damage every tick) and Tinder Breaker, CBC and EQC add a flat 2,500, 4,000 and 6,000 per level after DEF and element.

### Eleanor benchmark

`python tools/harness/eleanor_bench.py` runs the `eleanor_bench_*` scenarios (180 simulated seconds on the training dummy, the max-level Eleanor of `fixtures/homun_stats.lua`, 3,381 SP, no refill) for a no-skill baseline, Sonic Claw only, and the matrix of the test server's benchmark: fighting or grappling style crossed with Blazing And Furious and The One Fighter Rises. The runs are deterministic. Scenario options added for it: `swingMs` (the homunculus keeps swinging by itself while the AI asks for the attack), `castPauseMs` (every accepted cast pauses the swinging), `incomingHitMs` (an attacker the AI never sees hits her, which is what feeds grappling spheres) and `startStyle`. Two values are calibrated on the measured Sonic Claw sessions, not taken from the server code: 129 ms per swing (11,500 DPS without skills is about 7.8 swings/s) and 550 ms of swinging lost per cast (with Sonic Claw she swung only about 3.9 times/s). SP regeneration is 22 SP/s (20.2 and 22.8 SP/s measured).

Checked against the measured Sonic Claw sessions, the benchmark is 1.7% above one (75,747 against 74,476 DPS, `AutoSkillDelay` 0) and 10.6% above the other (65,941 against 59,602, `AutoSkillDelay` 200): the simulated client has no latency per cast. Only Sonic Claw alone is validated that way. The combo rows predict casts about twice as often as the earlier measured sessions (61 Sonic Claw > Silvervein Rush > Midnight Frenzy cycles in 180 s against 23), but those ran before the Eleanor sphere and chain fixes and with 200 ms of extra delay, so they say nothing yet about the current AI. Sensitivity of the combo advantage over Sonic Claw alone (+43% by default) to the two calibrated values: +33% without swing pauses at 100 ms per swing, +45% at 200 ms per swing; with a 1,000 ms pause per cast Sonic Claw alone collapses to 12,401 DPS, because her spheres come from her own swings.

In a scenario, `"MH_TOXIN_OF_MANDARA:2"` means the skill is learned at Lv2; the mock server refuses casts above the learned level.

### Learned skill levels

The AI reads the learned level of each homunculus skill with `GetV(V_SKILLATTACKRANGE_LEVEL)`. For skills whose low levels all have range 1 (Toxin of Mandara, Heilige Pferd, The One Fighter Rises Lv1-4, Moonlight) the client can't tell the learned level apart, so the AI:

- never casts above the configured level or the highest level the client confirms;
- lowers the level by one after 2 refused casts at it (never below a level that already worked);
- saves that level in `data/H_<owner>Skills.lua`, so map changes don't start over;
- tries one level higher at most once per hour (`LEARNED_SKILL_RECHECK_SECONDS`), in case the skill was levelled.

Skill cooldowns are also saved in `data/H_<owner>Timeouts.lua`, so the AI doesn't recast them right after a map change, including short cooldowns such as Toxin of Mandara.

The simulator follows rAthena's rules (`clif_parse_UseSkillToId_homun`, `skill_check_condition_mercenary`): homunculus/mercenary skills need **more** SP than their cost, so `DoSkill` never sends a skill at exactly its SP cost. rAthena silently lowers a level above the learned one, so the level step-down above only matters on servers that refuse such casts.

GitHub Actions (`.github/workflows/ci.yml`) runs on every push/PR:

- Lua syntax check and simulator;
- `node --check` on the web GUI (checked out from the `webgui` submodule);
- build of the .NET 4.0 config GUI, packaging `USER_AI` + `AzzyAIConfig.exe`,
  the three READMEs, manuals, technical notes and historical changelog as the
  `AzzyAI` artifact;
- on pushes to `main`, a follow-up job commits that build back to
  `USER_AI/AzzyAIConfig.exe` when it changed (short `build:` commit, `[skip ci]`).

Pushing a `v*` tag also publishes `AzzyAI.zip` as a GitHub release.

## Configuration Guide

### Server Type Configuration
To ensure the AI functions correctly on your server, set the `ServerType` variable in `H_Config.lua` (and `M_Config.lua` for mercenaries).
* For **official servers**, set `ServerType = SERVER_OFFICIAL`.
* For **private servers**, set `ServerType = SERVER_PRIVATE`.

With the wrong value on a private server every monster looks like a player and the homunculus never attacks. To avoid that, `AutoDetectServerType = 1` (default) switches to `SERVER_PRIVATE` by itself as soon as it sees a monster with an rAthena/Hercules ID (110000000 or more) and logs it in `AAI_ERROR`. It is skipped if the config sets `MagicNumber`/`MagicNumber2`/`MagicNumber3`.

`ServerType` also selects the homunculus skill data. `H_SkillList.lua` holds the official values, targeting LATAM Episode 19 (Isgard, October 2026): cast times, after-cast delays, cooldowns, durations, AoE sizes and sphere costs come from the LATAM client data on Divine Pride (`tools/harness/divinepride_sync.py`, needs a free API key in `DIVINEPRIDE_API_KEY`; its output is git-ignored), with kRO's client text and bROWiki filling the few gaps. `ServerSkillOverrides` lists only the rAthena Renewal values that differ (a few ranges, cooldowns, durations, AoE sizes and CBC's sphere cost) and replaces them under `SERVER_PRIVATE`. The comments above both tables record the sources, the audit date and the differences that were deliberately not copied.

### Monster Database
`USER_AI/MobInfo.lua` lists the race, element, element level, size, boss/MVP/plant/immobile flags and attack range of every monster, the element damage table and the element of each homunculus attack skill. It is generated from rAthena Renewal (`db/re/mob_db.yml`, `attr_fix.yml`, `skill_db.yml`) by `tools/harness/rathena_sync.py`. The homunculus reads a monster's ID with `GetV(V_HOMUNTYPE)`, so it uses the database without any setup; mercenaries only know monster IDs from MobID files and are not affected.

* **Skills by element** (`SkillElementMinRate`, default 50): an attack skill is left out when the target's element would cut its damage below this percent, and always when the target is immune or healed by it. Dieter won't cast Lava Slide on Fire monsters, for example. The AI then picks another skill or attacks normally. Poison Mist and Lava Slide used as AoE buffs (`PoisonMistMode`/`LavaSlideMode`) wait until at least one monster near you would take enough damage.
* **Packs of mixed elements**: area skills count each monster by the damage its element lets through (an Earth 2 monster counts 1.75 against fire, a Fire 2 one 0), both to decide whether the skill is worth casting (`AutoMobCount`) and where to aim it. An area skill may still be used when the current target is immune, if the monsters around it make up for it; it is then aimed at them.
* **Plants** (`PlantSkills`, default 0): plants and mushrooms take 1 damage from everything, so no skills are used on them and they don't count for area skills.
* **Bosses** (`BossAggro`, default 1): the homunculus never starts a fight with an MVP (`2` = also mini-bosses), whatever the tactics say; it still defends you and itself when they attack. `0` lets the tactics decide.
* **Target priority** (`TargetPriority`, default 0): with `1`, the next target is chosen by more than distance. The tactics priority still decides first, with plants always last. Among targets of the same priority: monsters attacking you or a friend, then ones at least one of your elemental skills can hurt, then the one in the middle of the biggest group an area skill would hit (monsters counted like in *Packs of mixed elements*), then the closest. `0` keeps the old order (tactics priority, then closest).
* **Kiting by range** (`KiteByRange`, default 0, needs `KiteMonsters` and the monster's kite tactic): with `1`, kiting uses the monster's real attack range instead of a fixed distance. A melee monster still uses `KiteThreshold`; a ranged one (an archer with range 9) makes the homunculus start kiting at `KiteThreshold` + range − 1 cells and take a long enough step to leave that range. Monsters that never chase (immobile ones) only matter inside their range, and a monster whose range reaches `KiteBounds` is not kited at all, since there is no way out of it. Needs a regenerated `MobInfo.lua` (`python tools\harness\rathena_sync.py`); an older one has no ranges and the fixed thresholds stay.

`UseMobInfo = 0` turns all of this off. `AAIStartH.txt` shows whether the database was loaded. Tactics rows still apply first; the database only adjusts attacking tactics for bosses and the skill/weight of plants.

The web GUI uses the same rAthena data (`webgui/mobdata.js`, also written by `rathena_sync.py`, with the maps each monster spawns on) to suggest tactics rows: pick a map and it adds a row for every monster there, based on whether it is a plant, a boss, aggressive or unable to move.

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
`python tools/harness/run.py --trace <scenario>` enables them for the run.
