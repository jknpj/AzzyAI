# AzzyAI HG for Ragnarok Online

[English](README.md) | [Português (BR)](README.pt-BR.md) | [Español](README.es.md)

Configure how your **homunculus or mercenary** follows you, attacks monsters
and uses skills, without having to edit code.

HG is a community fork of [the original AzzyAI](https://github.com/SpenceKonde/AzzyAI),
not an official Gravity product. This branch is being tested for RO LATAM and
private servers; behavior and available skills depend on your server.
Always follow your server's rules for custom AI.

## What can it do?

- Configure skills and buffs for Eira, Bayeri, Sera, Dieter and Eleanor.
- Set different behavior for different monsters.
- Choose when to attack, follow, keep distance or use area skills.
- Temporarily try other attacks when an offensive cast keeps getting interrupted.
- Edit settings through a **Windows app** or a **browser**.

New skills only work if your server supports them and your homunculus has
learned them. Enabling a skill does not guarantee it will be selected on every
attack: range, SP, cooldown, element and other settings still matter.

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

Server cooldowns, SP costs, cast interruptions and skill requirements still
apply. HG does not remove them.

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
