# AzzyAI - RO LATAM HG (Homologation Branch)

This is the source code repository for AzzyAI. 

This is not the official AzzyAI but a fork.
For the original repository please refer to: https://github.com/SpenceKonde/AzzyAI
For more information, see the official website: http://drazzy.com/ro/ai

=== Organization ===

* DEFAULT contains the default AI
* USER_AI contains AzzyAI

## Changes ##
* Skills are updated to their new max-level to reflect [official kRO changes](http://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=6843&curpage=1)
* **Corrected Skill Data:** The `H_SkillList.lua` has been fully updated with accurate data for all skills, including their correct Max Levels, SP costs, Cast Ranges, Durations, Cooldowns, and AoE information.
* New updated GUI (Thanks to [samufacanha2](https://github.com/samufacanha2/AzzyAI))
  
This includes the following skills (Thanks to [SpaceKonde](https://github.com/SpenceKonde/AzzyAI)):
 - Eira
   - Xeno Slasher
   - Eraser Cutter
 - Bayeri
   - Stahl Horn
   - Heilige Stange
 - Sera
   - Needle of Paralyze
   - Pain Killer
 - Dieter
   - Lava Slide
   - Pyroclastic
 - Eleanor
   - Silvervein Rush
   - Midnight Frenzy

Addionally, with the arrival of 4th jobs and new homunculus skills, the following new skills have been added:
 - Dieter
   - Tempering (Thanks to [Kisaro](https://github.com/Kisaro/AzzyAI))
   - Blast Forge (Thanks to [vorpal1337](https://github.com/vorpal1337))
- Eira
   - Twister Cutter
   - Absolute Zephyr
- Eleanor
   - Blazing And Furious
   - The One Fighter Rise
- Sera
   - Toxin Of Mandara
   - Needle Stinger
- Bayeri
   - Glanzen Spies
   - Heilige Pferd
   - Goldene Tone

## Tests & CI

The AI can't run outside the RO client, so `tools/harness/` contains an offline simulator: it mocks the client API (`GetV`, `GetActors`, `SkillObject`, ...), loads the real `USER_AI` files with a fixed config (`tools/harness/fixtures/H_Config.lua`) and runs scenarios (`tools/harness/scenarios.lua`) for every homunculus S, checking that:

- expected skills are cast and forbidden ones aren't;
- no skill is cast without enough SP or before its cooldown;
- the homunculus still does normal attacks;
- official/private server overrides are applied.

Run locally (needs Lua 5.1 on PATH, or `LUA`/`LUAC` env vars, and Python 3):

```
python tools/harness/run.py              # syntax check of all .lua + all scenarios
python tools/harness/run.py dieter sera  # only some scenarios
python tools/harness/run.py --verbose --trace bayeri
```

GitHub Actions (`.github/workflows/ci.yml`) runs on every push/PR:

- Lua syntax check and simulator;
- `node --check` on the web GUI;
- build of the .NET 4.0 config GUI, packaging `USER_AI` + `AzzyAIConfig.exe` as the `AzzyAI` artifact.

Pushing a `v*` tag also publishes `AzzyAI.zip` as a GitHub release.

## Configuration Guide

To use the new skills, you must add and adjust their settings in your **`H_Config.lua`** file.
