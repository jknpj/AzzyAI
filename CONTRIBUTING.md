# Contributing to AzzyAI

Thanks for helping. You do not need to write code: bug reports, in-game test
results, translations and clearer docs are all welcome.

## Ways to help

- **Report a bug** with the [bug report form](https://github.com/jknpj/AzzyAI/issues/new/choose).
  Say what homunculus, server and client you use, which skills are enabled and
  what happened. **Never share passwords, account credentials or private files.**
- **Test in game.** Most of this version is checked only on a simulated server. Reports
  from real clients, official or private, are the most valuable thing you can send.
  Say what worked as well as what did not.
- **Translate.** The manuals and READMEs exist in English, Portuguese (BR) and
  Spanish, and the web page has descriptions in all three.
- **Improve the docs**, especially anything that confuses a new player.
- **Fix or add a feature.** For anything bigger than a small fix, open an issue
  first so we can agree on the approach.

## Before you open a pull request

Work from the **main** branch and open the pull request against it.

You need Python 3 and a **Lua 5.1** interpreter (the version inside the RO
client). Put `lua5.1` and `luac5.1` on `PATH`, or set the `LUA` and `LUAC`
environment variables. Then, from the repository root:

```
python tools\harness\run.py                          # syntax check and every scenario
python tools\harness\run.py dieter bayeri_pferd      # only these scenarios
python tools\harness\run.py --verbose sera           # also print every cast
python tools\harness\run.py --profile crowd_dieter   # time and GetV calls per AI tick
python tools\harness\run.py --globals                # globals assigned inside functions
```

The harness mocks the client API, loads the real `USER_AI` files and checks, for
example, that expected skills are cast, forbidden ones are not, no cast happens
without SP or before its cooldown, and that the simulated server would not
refuse it. CI runs the same, plus a syntax check of the web files and a build of
the Windows app.

### What a change should include

- **A behavior change needs a scenario** in `tools\harness\scenarios.lua`
  (`expect`, `forbid`, `checks`). If you fix a bug, add the case that failed.
- **A new option must be read by the AI**: `run.py` fails when a name in
  `USER_AI\Defaults.lua`, `USER_AI\H_Config.lua` or `USER_AI\M_Config.lua` appears nowhere in
  `USER_AI` (an option that changes nothing is a bug; `KiteParanoid` and `MoveStickyFight` were
  both like that). Add the reader before the default.
- **A new option** goes in all of these places:
  - `USER_AI\Defaults.lua`, with a one-line comment;
  - `USER_AI\H_Config.lua` or `USER_AI\M_Config.lua`, if the player is meant to see and change it;
  - the Windows app: `GUI\AzzyAIConfig\HomConf.Additional.cs` (or the matching
    mercenary file);
  - the web page: `webgui\homSchema.js` (and the `AzzyHomHgOnly` list) or
    `merSchema.js`, plus `i18n.desc.pt.js` and `i18n.desc.es.js`;
  - the manual, in the three languages if you can (say so in the pull request if you cannot).
- **Keep it Lua 5.1.** Declare variables `local`; `run.py --globals` lists the
  ones that leak. Keep diffs small and comment only what needs it.
- **Say what you tested**: offline harness only, or also in game, and on which server.

### Generated files

Do not edit these by hand. Regenerate them:

| File | How |
|---|---|
| `USER_AI\MobInfo.lua`, `tools\harness\fixtures\rathena.lua`, `webgui\mobdata.js` | `python tools\harness\rathena_sync.py` (standard library only; behind a proxy set `HTTPS_PROXY` first) |
| `webgui\skilldata.js`, `webgui\img\skills\` | `python skilldata_sync.py` inside `webgui` (needs a free Divine Pride API key in `DIVINEPRIDE_API_KEY`) |

### Do not commit

- your personal `H_Config.lua` / `M_Config.lua` changes, `A_Friends.lua`, the
  `data` folder or any log;
- API keys or credentials of any kind.

## Releases

The AI's release number lives in one place: `CVersion` in `USER_AI\Const_.lua`.
Do not change it in a pull request unless asked: a maintainer raises it when a release
is cut. How the number works, and what to raise, is in
[docs\TECHNICAL.md](docs/TECHNICAL.md#version). Three other numbers (`Version`,
`MainVersion`, `AUVersion`, all `"2.0"` today) are a file check, and the Windows app has
the same release number in `AssemblyInfo.cs`: leave them all alone unless a maintainer
raises them with the release.

## The web page is a separate repository

`webgui` is a git submodule of [jknpj/AzzyAI-webgui](https://github.com/jknpj/AzzyAI-webgui).
Send changes to it there. Then a maintainer updates the submodule pointer here.
Clone with `git clone --recursive`, or run `git submodule update --init`.

## Licensing of contributions

New files in this version are under the GNU GPL v3; inherited files have no license from
their original author. [NOTICE.md](NOTICE.md) explains which is which. By
sending a contribution you confirm that you wrote it, or have the right to send
it, and that it may be distributed under the terms that [NOTICE.md](NOTICE.md)
gives for the files it touches.

## Security

Do not report vulnerabilities in a public issue. See [SECURITY.md](SECURITY.md).
