# AzzyAI HG for Ragnarok Online

[English](README.md) | [Português (BR)](README.pt-BR.md) | [Español](README.es.md)

> [!IMPORTANT]
> Get AzzyAI HG only from [jknpj/AzzyAI](https://github.com/jknpj/AzzyAI) (branch **LATAM-HG**) or its [Releases](https://github.com/jknpj/AzzyAI/releases). A similar repository name or a copied README does not make a download official. **Back up your game's `AI` folder before installing**, and follow your server's rules for custom AI.

[![AzzyAI HG: homunculus and mercenary AI for Ragnarok Online](docs/hg-banner.svg)](#quick-start)

* * *

A smarter brain for your **homunculus or mercenary**. It decides who to attack,
which skill to use and when, and you adjust it in a window or in your browser,
without editing code.

HG is a community fork of [the original AzzyAI](https://github.com/SpenceKonde/AzzyAI),
not an official Gravity product. This branch is being tested for RO LATAM and
private servers; behavior and available skills depend on your server.

> **[Open the web configuration](https://jknpj.github.io/AzzyAI-webgui/)** to set it up in your browser, or read the **[user manual](docs/MANUAL.md)** for the full guide.

## Quick start

Three steps: install, configure, test. Change one thing at a time.

### 1. Install

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

### 2. Configure

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

First settings to check:

- Choose **Official** or **Private** under `ServerType`.
- Enable only skills you want to use, at levels your homunculus has learned.
- Check attack and area-skill settings before changing advanced options.
- Change one thing at a time and reload the AI between tests.

### 3. Ask an AI to help you tune it

Not sure why a skill is never used, or what to change? Ask any AI chat
(ChatGPT, Copilot, Gemini, Claude...) to read your files and explain. You need no
extra software: open a chat, attach your files and paste the request.

<details>
<summary><b>Show the files to attach and the request to paste</b></summary>

**Attach** (from your game's `AI\USER_AI` folder, or from the game folder for the logs):

- `H_Config.lua`, your homunculus settings. For a type preset, also `Types\<Name>.lua`.
- `AAIStartH.txt`, the startup report. It shows whether the monster data loaded.
- `AAI_ERROR.log`, if it exists.

**Do not attach** `A_Friends.lua`, the `data` folder or anything with passwords or
account details. Open the files in a text editor first if you are unsure.

**Paste this**, and fill in the last three lines:

```text
You are helping me configure AzzyAI HG, a Ragnarok Online homunculus AI.
Read the attached files and this manual first:
https://github.com/jknpj/AzzyAI/blob/LATAM-HG/docs/MANUAL.md
(if you cannot open the link, tell me and I will paste the relevant part).

Rules for your answer:
1. Use simple words. I am not a programmer.
2. For each suggestion give: the exact option name from my H_Config.lua, its
   current value, the value you suggest, and why, quoting the line or the manual
   section that supports it.
3. Only suggest options that exist in my files or in the manual. Never invent one.
4. Say clearly what you READ in my files and what you are GUESSING.
5. If you cannot tell, say so and ask me ONE question instead of guessing.
6. Suggest one change at a time, and remind me to back up the AI folder and reload
   the AI in game before testing.

My homunculus: <Eira / Bayeri / Sera / Dieter / Eleanor / other>
My server: <official RO LATAM / private>
What is wrong or what I want: <for example: "Dieter never uses Lava Slide on a pack">
```

</details>

The AI cannot see your game or run the AI, so treat its answer as a suggestion:
check the evidence it quotes, change one thing, test on an easy monster, and go
back to your backup if it gets worse.

### Update HG

The Windows app has an **Update** menu: **Update AI from git...** pulls your local
git clone, or downloads the `LATAM-HG` branch from GitHub (no git needed), and
copies it over your installation. It keeps your
settings, tactics, friend list and extra options, and saves every file it
replaces in `backup\update-<time>\`. Reload the AI in game afterwards. Updating by
hand? Follow the [update checklist](docs/MANUAL.md#updates-and-backups).

## How HG works

[![How AzzyAI HG decides what to do: a monster appears, HG looks it up, decides and casts within the server rules](docs/hg-how-en.svg)](docs/hg-how-en.svg)

The homunculus reads which monster it is looking at and checks it in a list of
about 2,700 monsters. Your settings and tactics then decide who to attack and
which skill to use, and the server data keeps every cast within what your server
accepts. [Open the full-size figure](docs/hg-how-en.svg).

## What you can configure

| Homunculus | Skills HG can use | Worth knowing |
|---|---|---|
| **Eira** | Xeno Slasher, Eraser Cutter, Silent Breeze, Twister Cutter, Absolute Zephyr, Overed Boost, Light of Regenerate | Light of Regenerate is optional and off by default: it needs Loyal intimacy and lowers it on every cast. |
| **Bayeri** | Stahl Horn, Heilige Stange, Stein Wand, Goldene Ferse, Angriffs Modus, Glanzen Spies, Heilige Pferd, Goldene Tone | The Bayeri preset picks between Stahl Horn, Spies and the area skills by the target's DEF. |
| **Sera** | Needle of Paralyze, Poison Mist, Summon Legion, Pain Killer, Toxin of Mandara, Needle Stinger | Toxin of Mandara and Needle Stinger have casts that can be interrupted. |
| **Dieter** | Magma Flow, Granitic Armor, Lava Slide, Pyroclastic, Volcanic Ash, Blast Forge, Tempering | His attacks are area skills, so his preset attacks even a single monster. Fire skills are skipped against Fire monsters. |
| **Eleanor** | Style Change, Sonic Claw, Silvervein Rush, Midnight Frenzy, Tinder Breaker, CBC, EQC, Blazing And Furious, The One Fighter Rises | Sonic Claw needs fighting style; Tinder Breaker, CBC and EQC need grappling style. The AI switches style for you. |
| **Lif, Amistr, Filir, Vanilmirth** | Their own skills, and the inherited ones in a Homunculus S | HG detects the base type of a Homunculus S by itself. |
| **Mercenaries** | Tactics, skills and kiting, as in the original AzzyAI | They do not use the monster database. |

Skills only work if your server offers them and your homunculus has learned
them. The [user manual](docs/MANUAL.md#6-notes-for-each-homunculus-s) has notes for each one.

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

### Should you switch?

**It is a good fit if** you play Eira, Bayeri, Sera, Dieter or Eleanor, you want
to tune behavior without reading code, or you hunt packs and want area skills
used well.

**You may not need it if** your current AI already does what you want, or your
server has different skill rules and you would rather not test.

**Trying it is safe:** back up your `AI` folder first. Going back is just
restoring that backup. Your tactics and friend list can be kept (see the
[update checklist](docs/MANUAL.md#updates-and-backups)).

### What changes by default

These are on from the start; each can be turned off in the configuration tool.

- **Monster database** (`UseMobInfo`): element, plants and bosses are taken into account.
- **MVPs** (`BossAggro`): the homunculus does not start fights with MVPs.
- **Commanded fights** (`CommandedFightStay`): a monster you tell it to attack is fought until it dies.

Optional extras, off by default: smarter target choice (`TargetPriority`),
keeping distance by the monster's real range (`KiteByRange`) and escaping when
HP drops fast (`UseDamageDanger`). Offline tests cover them, but they have not
been verified in game yet.

### Honest status

HG is tested offline and on a local rAthena server. That is not a guarantee for
every official client or server. New skills only work if your server supports
them and your homunculus has learned them. Enabling a skill does not guarantee
it is used on every attack: range, SP, cooldown and element still matter.
Server cooldowns, SP costs and cast interruptions still apply.

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
