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

## Configuration Guide

To use the new skills, you must add and adjust their settings in your **`H_Config.lua`** file.
