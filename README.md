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

Long cooldowns (10 s or more) are also saved in `data/H_<owner>Timeouts.lua`, so the AI doesn't recast them right after a map change.

GitHub Actions (`.github/workflows/ci.yml`) runs on every push/PR:

- Lua syntax check and simulator;
- `node --check` on the web GUI;
- build of the .NET 4.0 config GUI, packaging `USER_AI` + `AzzyAIConfig.exe` as the `AzzyAI` artifact.

Pushing a `v*` tag also publishes `AzzyAI.zip` as a GitHub release.

## Configuration Guide

### Server Type Configuration
To ensure the AI functions correctly on your server, you must set the `ServerType` variable in your `Defaults.lua` file.
* For **official servers**, set `ServerType = SERVER_OFFICIAL`.
* For **private servers**, set `ServerType = SERVER_PRIVATE`.

### Skill Configuration
To use the new skills, you must add and adjust their settings in your **`H_Config.lua`** file. The AI will automatically disable any skills that are misconfigured or not learned by your homunculus.