# AzzyAI - Private Servers

This is the source code repository for AzzyAI. 

This is not the official AzzyAI but a fork.
For the original repository please refer to: https://github.com/SpenceKonde/AzzyAI
For more information, see the official website: http://drazzy.com/ro/ai

=== Organization ===

* DEFAULT contains the default AI
* USER_AI contains AzzyAI

## Changes ##

* Private Server Compatibility
This fork has been specifically modified to function correctly on private servers that use modern emulators like Hercules and rAthena.

* **The Problem:**
The original AzzyAI was designed exclusively for official servers (Aegis), which use a specific method for assigning identification numbers (IDs) to in-game entities like monsters and players. The AI's core logic relies on hardcoded "magic numbers" to distinguish between these entities.

Private server emulators (Hercules, rAthena) use a different, more modern ID allocation system as a deliberate architectural improvement. This fundamental difference causes the original AzzyAI's targeting logic to fail, rendering the homunculus unable to identify or attack monsters.

* **The Fix:**
To resolve this incompatibility, the following changes have been implemented:

* Actor ID Ranges Updated: The hardcoded "magic numbers" in Defaults.lua have been changed to match the ID ranges used by Hercules/rAthena emulators.

* Targeting Logic Patched: The conditional logic in AI_MAIN.lua and AzzyUtil.lua has been updated to correctly use these new ID ranges, ensuring monsters and other players are identified properly.

With these patches, the AI can now function reliably on private servers.

## Other Changes ##

* Skills are updated to their new max-level to reflect [official kRO changes](http://ro.gnjoy.com/news/notice/View.asp?BBSMode=10001&seq=6843&curpage=1)
* **Corrected Skill Data:** The `H_SkillList.lua` has been fully updated with accurate data for all skills, including their correct Max Levels, SP costs, Cast Ranges, Durations, Cooldowns, and AoE information.

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

## Configuration Guide

To use the new skills, you must add and adjust their settings in your **`H_Config.lua`** file.
