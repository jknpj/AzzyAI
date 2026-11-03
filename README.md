# AzzyAI HG for Ragnarok Online

[English](README.md) | [Português (BR)](README.pt-BR.md) | [Español](README.es.md)

A smarter brain for your **homunculus or mercenary**. It decides who to
attack, which skill to use and when, and you adjust it in a window or in your
browser, without editing code.

HG is a community fork of [the original AzzyAI](https://github.com/SpenceKonde/AzzyAI),
not an official Gravity product. This branch is being tested for RO LATAM and
private servers; behavior and available skills depend on your server.
Always follow your server's rules for custom AI.

## Why use this one?

The original AzzyAI (the `master` branch) is still a good AI. HG keeps what it
does and adds the following. This is what you would notice in game:

| | Original AzzyAI | HG |
|---|---|---|
| **Which monster is it?** | No monster data. You write a tactics row for each monster yourself. | Knows the element, race and boss/MVP/plant flags of every monster (about 2,700), so it works well on monsters you never configured. |
| **Skills vs. element** | Uses a skill even on a monster that resists it. | Skips a skill that would do less than half damage (adjustable) and uses another attack. Dieter does not throw Lava Slide at a Fire monster. |
| **Area skills** | Counts monsters, whatever their element. | Counts only the monsters that would really be hurt, and aims at them even when the one in front is immune. |
| **Plants and mushrooms** | Guesses plants by how they behave (`AutoDetectPlant`). | Knows exactly which ones are plants (they take 1 damage from everything) and saves SP for the others. |
| **MVPs** | Only what you put in the tactics. | By default it does not *start* a fight with an MVP, but still defends you. Optional: always attack MVPs. |
| **A cast that keeps being interrupted** | A fixed retry limit (`SkillRetryLimit`), nothing smarter. | Pauses that skill for a few seconds after a few interruptions and uses something else. |
| **Skill levels** | Most Homunculus S skills stop at level 5 in its table. | Current max levels (10 for most) and only casts levels your homunculus has learned. |
| **Homunculus S skills** | Eira, Bayeri, Sera, Dieter and Eleanor basics. | Also Twister Cutter, Absolute Zephyr, Toxin of Mandara, Needle Stinger, Glanzen Spies, Heilige Pferd, Goldene Tone, Blazing And Furious, The One Fighter Rises, Blast Forge and Tempering. |
| **Eleanor** | Style change and combos. | Also spheres by the server's rules (what each skill costs), combo timing and the two new skills. |
| **Your server's numbers** | One set of values. | Official data (RO LATAM, Episode 19) or rAthena data, chosen with `ServerType`, so cooldowns, ranges and costs match your server. |
| **Settings** | Windows app. | The Windows app and a **browser page** in English, Português and Español, with skill descriptions, presets per homunculus and suggested monster tactics for a map. |
| **Updates** | Manual. | The Windows app can update the AI and the monster data for you, without touching your settings. |
| **Testing** | No offline test suite. | Each change runs against a simulated server (dozens of scenarios) before it is published. |

The monster-aware features are for **homunculi**. Mercenaries get the tools and
the bug fixes, but not the monster database.

## Should you switch?

**It is a good fit if** you play Eira, Bayeri, Sera, Dieter or Eleanor, you want
to tune behavior without reading code, or you hunt packs and want area skills
used well.

**You may not need it if** your current AI already does what you want, or your
server has different skill rules and you would rather not test.

**Trying it is safe:** back up your `AI` folder first. Going back is just
restoring that backup. Your tactics and friend list can be kept (see the
[update checklist](docs/MANUAL.md#updates-and-backups)).

## What changes by default

These are on from the start; each can be turned off in the configuration tool.

- **Monster database** (`UseMobInfo`): element, plants and bosses are taken into account.
- **MVPs** (`BossAggro`): the homunculus does not start fights with MVPs.
- **Commanded fights** (`CommandedFightStay`): a monster you tell it to attack is fought until it dies.

Optional extras, off by default: smarter target choice (`TargetPriority`),
keeping distance by the monster's real range (`KiteByRange`) and escaping when
HP drops fast (`UseDamageDanger`). Offline tests cover them, but they have not
been verified in game yet.

## Honest status

HG is tested offline and on a local rAthena server. That is not a guarantee for
every official client or server. New skills only work if your server supports
them and your homunculus has learned them. Enabling a skill does not guarantee
it is used on every attack: range, SP, cooldown and element still matter.
Server cooldowns, SP costs and cast interruptions still apply.

## Install in a few steps

1. **Back up your game's entire `AI` folder**, especially if you already use another AI.
2. Obtain a package from [Releases](https://github.com/jknpj/AzzyAI/releases)
   when available, or download the code from the **LATAM-HG** branch.
3. Copy the **contents of this project's [USER_AI folder](USER_AI)** into your
   game's `AI\USER_AI` folder. Avoid creating `AI\USER_AI\USER_AI`.
4. In the game, use **`/hoai`** until the message confirms custom homunculus AI.
   For mercenaries, use **`/merai`**.
5. Summon the homunculus again or reconnect so the game loads the new files.
6. Configure your server and skills, then test on an easy monster.

**Updating an existing installation?** Preserve your settings, tactics and
friend list before copying files. See the [update checklist](docs/MANUAL.md#updates-and-backups).

## Choose your configuration tool

| Tool | How to start |
|---|---|
| Windows GUI | Run [AzzyAIConfig.exe](USER_AI/AzzyAIConfig.exe) **from your game's `AI\USER_AI` folder**. |
| Web GUI | Open [AzzyAI Web Configuration](https://jknpj.github.io/AzzyAI-webgui/), choose **AzzyAI HG** and open your game's `USER_AI` folder. |

Use **Apply** to save. In browsers without direct folder writing, export/download
the configuration and copy it into the game folder. Reload the AI afterward.
The web GUI offers English, Brazilian Portuguese and Spanish; option names
stay in English. The desktop interface uses English labels; translated manuals
explain the main settings.

**Do not edit a downloaded/source folder by mistake:** the game reads the
configuration installed in its own folder, not an unrelated copy.

## Start here

- Choose **Official** or **Private** under `ServerType`.
- Enable only skills you want to use, at levels your homunculus has learned.
- Check attack and area-skill settings before changing advanced options.
- Change one thing at a time and reload the AI between tests.

## Guides and help

| Language | User manual |
|---|---|
| English | [Installation, settings, homunculi and troubleshooting](docs/MANUAL.md) |
| Português (BR) | [Instalação, configurações, homúnculos e problemas comuns](docs/MANUAL.pt-BR.md) |
| Español | [Instalación, ajustes, homúnculos y problemas frecuentes](docs/MANUAL.es.md) |

For a bug report, include the homunculus, server/client, enabled skills and
what happened. **Never share passwords, account credentials or private files.**
Use [Issues](https://github.com/jknpj/AzzyAI/issues).

Developers and advanced users: [release notes and technical reference](docs/RELEASE_NOTES.md).
The older bundled documentation is a legacy reference, not the current HG guide.

## Credits

Based on AzzyAI by Dr. Azzy / [SpenceKonde](https://github.com/SpenceKonde/AzzyAI),
with contributions from [samufacanha2](https://github.com/samufacanha2/AzzyAI),
[Kisaro](https://github.com/Kisaro/AzzyAI) and
[vorpal1337](https://github.com/vorpal1337). Original site: [drazzy.com](http://drazzy.com/ro/ai).
