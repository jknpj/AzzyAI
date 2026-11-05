# Security policy

AzzyAI runs as Lua inside the Ragnarok Online client and ships a Windows
configuration app and a browser configuration page. This policy covers those
parts. For game-rule questions, see your server's rules; they are not security
issues.

## Supported versions

Only the latest commit of the **LATAM-HG** branch and the latest
[release](https://github.com/jknpj/AzzyAI/releases). Older versions are not patched.

## Get AzzyAI only from official sources

- This repository, [jknpj/AzzyAI](https://github.com/jknpj/AzzyAI), branch **LATAM-HG**, and its Releases.
- The web configuration at <https://jknpj.github.io/AzzyAI-webgui/>.

A repository with a similar name, or a copied README, does not make a download
official. The AI and the configuration app run on your computer and read and
write files in your game folder, so do not run copies you cannot trace back to
the sources above.

## What to report privately

Anything that could let someone run code, read or overwrite files they should
not, or learn private information through AzzyAI. For example:

- **The Update menu** of the Windows app (it downloads and copies files, can run
  `rathena_sync.py` and stages a new `AzzyAIConfig.exe`): a downloaded archive
  that writes outside `USER_AI`, a missing integrity check, or an update that
  replaces your settings, tactics or friend list.
- **Lua loading**: the AI reads settings, `H_Extra.lua` and files in `data\`. A
  way to make it run code you did not put there.
- **The web page**: a configuration, tactics or ZIP file that makes the page
  write outside the folder you opened, run script (XSS), or leak data.
- **Logs and exports** that include personal information.

## What is not a security issue

Open a normal [issue](https://github.com/jknpj/AzzyAI/issues) instead for:

- the AI doing something odd in game (a skill not used, a wrong target);
- server rules, botting policies or bans, which are decided by your server;
- vulnerabilities in the Ragnarok Online client or in a server.

## How to report

1. On the repository page, open **Security** and choose **Report a vulnerability**
   (GitHub private reporting): <https://github.com/jknpj/AzzyAI/security/advisories/new>.
2. If that button is not available, open an issue titled **Security contact
   request** with **no technical details**, and a private way to continue will be arranged.

Include the version or commit, what you did, what happened, what you expected
and the impact. **Never post exploit details, passwords, account files or logs
with personal data in a public issue.**

This is a small volunteer project. Reports are read as time allows, with no
guaranteed response time. Please allow a reasonable period to fix an issue
before you publish it.

## Protect yourself

- Back up your game's `AI` folder before installing or updating. Going back is
  restoring that backup.
- Treat `H_Extra.lua` and `M_Extra.lua` as what they are: Lua code that the game
  client runs. Do not paste code into them that you do not understand, and do
  not take them from strangers.
- Run `AzzyAIConfig.exe` only from your own `AI\USER_AI` folder, from a source
  you trust.
- Never share passwords, account credentials or the `data` folder when you ask
  for help.
