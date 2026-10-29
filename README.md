# AzzyAI - RO LATAM HG (Homologation Branch)

This is the source code repository for AzzyAI.

This is not the official AzzyAI but a fork.
For the original repository please refer to: [https://github.com/SpenceKonde/AzzyAI](https://github.com/SpenceKonde/AzzyAI)
For more information, see the official website: [http://drazzy.com/ro/ai](http://drazzy.com/ro/ai)

## Organization

* `DEFAULT` contains the default AI
* `USER_AI` contains AzzyAI

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
- casts the server would refuse ("Skill failed": level not learned) stay within `maxRefused`, also across a simulated map change (`reloadAt`).

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
- `node --check` on the web GUI;
- build of the .NET 4.0 config GUI, packaging `USER_AI` + `AzzyAIConfig.exe` as the `AzzyAI` artifact.

Pushing a `v*` tag also publishes `AzzyAI.zip` as a GitHub release.

## Configuration Guide

### Server Type Configuration
To ensure the AI functions correctly on your server, set the `ServerType` variable in `H_Config.lua` (and `M_Config.lua` for mercenaries).
* For **official servers**, set `ServerType = SERVER_OFFICIAL`.
* For **private servers**, set `ServerType = SERVER_PRIVATE`.

With the wrong value on a private server every monster looks like a player and the homunculus never attacks. To avoid that, `AutoDetectServerType = 1` (default) switches to `SERVER_PRIVATE` by itself as soon as it sees a monster with an rAthena/Hercules ID (110000000 or more) and logs it in `AAI_ERROR`. It is skipped if the config sets `MagicNumber`/`MagicNumber2`/`MagicNumber3`.

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