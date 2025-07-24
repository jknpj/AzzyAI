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

## Configuration Guide

### Server Type Configuration
To ensure the AI functions correctly on your server, you must set the `ServerType` variable in your `Defaults.lua` file.
* For **official servers**, set `ServerType = SERVER_OFFICIAL`.
* For **private servers**, set `ServerType = SERVER_PRIVATE`.

### Skill Configuration
To use the new skills, you must add and adjust their settings in your **`H_Config.lua`** file. The AI will automatically disable any skills that are misconfigured or not learned by your homunculus.