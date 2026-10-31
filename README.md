# AzzyAI - RO LATAM HG (Homologation Branch)

This is the source code repository for AzzyAI.

This is not the official AzzyAI but a fork.
For the original repository please refer to: [https://github.com/SpenceKonde/AzzyAI](https://github.com/SpenceKonde/AzzyAI)
For more information, see the official website: [http://drazzy.com/ro/ai](http://drazzy.com/ro/ai)

## Organization

* `DEFAULT` contains the default AI
* `USER_AI` contains AzzyAI
* `webgui` is a git submodule with the browser configuration GUI ([jknpj/AzzyAI-webgui](https://github.com/jknpj/AzzyAI-webgui)). Clone with `git clone --recursive`, or run `git submodule update --init` in an existing clone.

## Key Features

* **Multi-Server Compatibility:** Includes a `ServerType` feature that allows the AI to adapt its entity identification logic, making it compatible with both official and private servers.
* **Automatic Homunculus Base Type Detection:** The AI now automatically detects the original base type (Lif, Amistr, Filir, Vanilmirth) of a Homunculus S. This ensures inherited skills from the original form are used correctly without requiring manual configuration of `OldHomunType`.
* **Smart Skill Configuration (Linting):** On startup, the AI intelligently validates the user's `H_Config.lua` file. It automatically disables skills that do not belong to the current homunculus type and adjusts skill levels if they are set higher than what the homunculus has actually learned. This prevents errors and simplifies configuration.
* **Updated GUI:** Includes a new updated GUI (Thanks to [samufacanha2](https://github.com/samufacanha2/AzzyAI)).
* **Skills updated to new max-level:** Reflects [official kRO changes](http://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=6843&curpage=1).
* **Corrected Skill Data:** The `H_SkillList.lua` has been fully updated with accurate data for all skills, including their correct Max Levels, SP costs, Cast Ranges, Durations, Cooldowns, and AoE information.

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
    * Light of Regenerate (optional autobuff, off by default: `UseEiraLightOfRegene`, `EiraLightOfRegeneLevel`, `EiraLightOfRegeneOwnerHP`). Keeps the buff up so the Eira resurrects the owner if they die. It needs Loyal intimacy and lowers intimacy on every cast.
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

### Windows desktop GUI development

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

The AI can't run outside the RO client, so `tools/harness/` contains an offline simulator: it mocks the client API (`GetV`, `GetActors`, `SkillObject`, ...), loads the real `USER_AI` files with a fixed config (`tools/harness/fixtures/H_Config.lua`) and runs scenarios (`tools/harness/scenarios.lua`) for every homunculus S, checking that:

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
```

Run locally (needs Lua 5.1 on PATH, or `LUA`/`LUAC` env vars, and Python 3):

```
python tools/harness/run.py              # syntax check of all .lua + all scenarios
python tools/harness/run.py dieter sera  # only some scenarios
python tools/harness/run.py --verbose --trace bayeri
python tools/harness/run.py --skip-syntax --profile crowd_bayeri --ticks 600
```

`--profile` reports time per `AI()` call, the hottest functions (sampled), `GetV` calls per tick by type, Lua heap after GC and global tables that keep growing (leak check). The `crowd_*` scenarios put 40 monsters on screen to stress the per-tick loops. Absolute times are relative to the mock, not the RO client.

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
- build of the .NET 4.0 config GUI, packaging `USER_AI` + `AzzyAIConfig.exe` as the `AzzyAI` artifact.

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
the `Build`, the `ServerType` and `Logs enabled`. No `Build: LATAM-HG` line means the client
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
To use the new skills, you must add and adjust their settings in your **`H_Config.lua`** file. The AI will automatically disable any skills that are misconfigured or not learned by your homunculus.