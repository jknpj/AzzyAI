# AzzyAI HG user manual

[English](MANUAL.md) | [Português (BR)](MANUAL.pt-BR.md) | [Español](MANUAL.es.md)

[Back to README](../README.md) | [Technical release notes](RELEASE_NOTES.md)

## 1. Before you begin

Use the AI only where your server permits it. HG configures behavior; it does
not unlock skills, bypass cooldowns or make casts immune to damage.
Server support and client behavior vary. Test on easy monsters first.

The homunculus and mercenary have separate settings. Setting names remain in
English so they match the options shown in the tools.

## 2. Install and activate

1. Close configuration windows and back up the game's entire `AI` folder.
2. Download a release package when available, or the **LATAM-HG** branch.
3. Put the project's `USER_AI` contents in the game's `AI\USER_AI`.
   The final path should contain `AI\USER_AI\AI.lua`, not another nested folder.
4. Type `/hoai` in game until custom homunculus AI is selected.
   Use `/merai` for custom mercenary AI.
5. Summon again or reconnect. Repeat the reload after saving configuration changes.

Installing HG replaces the AI in that folder. If you want to use a different AI
for one unit type, preserve its entry file: `AI.lua` for homunculi or `AI_M.lua`
for mercenaries. Do not mix versions of HG's supporting files.

### Updates and backups

Before updating, keep a copy of:

| Files | What they contain |
|---|---|
| `H_Config.lua`, `M_Config.lua` | Homunculus and mercenary settings |
| `H_Tactics.lua`, `M_Tactics.lua` | Per-monster behavior |
| `H_PVP_Tact.lua`, `M_PVP_Tact.lua` | Player/PVP behavior |
| `H_Extra.lua`, `M_Extra.lua` | Extra options and custom code |
| `A_Friends.lua` and `data` | Friend list and saved runtime information |

Keep the original full backup somewhere outside the game's AI folder.
Do not overwrite personalized files blindly or replace new code with an old
full folder. Compare/import settings into the updated installation, save, reload
and test.

The Windows GUI can do this for you (menu **Update**, section 3). It never
overwrites the files in the table above (nor `Types\*.lua`, unless you tick the
box) and backs up everything it replaces.

## 3. Configure with a GUI

### Windows application

Run `AzzyAIConfig.exe` from the installed `AI\USER_AI` folder. It edits files
beside its executable, even if launched from a shortcut. Check the displayed
configuration location before saving.

- **Homunculus / Mercenary:** general behavior, skills and buffs.
- **Homunculus Tactics / Mercenary Tactics:** behavior for each monster.
- **PVP Tactics / Merc PVP Tactics:** behavior for players.
- **Extra Options:** advanced options and custom Lua; avoid editing code if unsure.

Search finds option names and descriptions. Type filters make options easier
to find; **hiding a setting does not turn it off**. Use **Clear filters** to see
everything. Click **Apply** to save; **Revert** discards pending edits.
The desktop labels are in English.

**Update menu (Windows application only):**

- **Update AI from git...** pulls your local git clone (`git pull --ff-only`) or
  downloads the `LATAM-HG` branch from GitHub (no git needed) and copies its
  `USER_AI` over the folder the GUI runs from. Your settings, tactics, extra and
  avoid lists are kept; with *Also overwrite the homunculus presets* ticked,
  `Types\*.lua` are replaced too. Replaced files are copied to
  `backup\update-<time>\`. A newer `AzzyAIConfig.exe` is saved as `.exe.new` and
  swapped in when you accept the restart. Reload the AI in game afterwards.
- **Update monster/skill data from rAthena...** runs `tools\harness\rathena_sync.py`
  in your clone (needs Python 3 and internet) and copies the regenerated
  `MobInfo.lua` here. The clone also receives the harness fixtures and
  `webgui\mobdata.js`, which you commit if you maintain the repository.

The clone folder is remembered in `AzzyAIConfig.update.txt`.

### Browser application

Open [the web GUI](https://jknpj.github.io/AzzyAI-webgui/).
Choose **AzzyAI HG**, your language, then **File > Open USER_AI folder** and
select the folder installed in the game.

- Chrome/Edge can write directly when folder access is supported and permitted.
- If direct writing is unavailable, use the downloaded/exported configuration
  ZIP and extract its configuration files into the game's `AI\USER_AI`.
- Without opening your folder, the page is not reading your installed settings.
- Do not assume a remembered browser configuration matches the files now in game.

Click **Apply**, confirm the save/export and reload the AI in game.
Some browser resources need an internet connection.

### Settings per homunculus type

Both GUIs have a **Settings for** selector on the Homunculus tab. *Global* edits `H_Config.lua`; choosing a type (Lif, Amistr, Filir, Vanilmirth, Eira, Bayeri, Sera, Dieter, Eleanor) edits `Types/<Name>.lua`, which only stores the values that differ from the global ones and is applied after `H_Config.lua` when that homunculus starts. **Use global values** removes the type's overrides. The selector also replaces the old Homunculus S / Base homunculus filters: a Homunculus S lists its own skill settings, a base homunculus lists only its own, and *Global* lists everything. Deleting a type file disables its preset. Dieter ships with `AutoMobCount = 1`, because all his attacks are area skills and the global value of 2 left him almost idle against a single monster.

## 4. Basic behavior

| Option | Practical meaning |
|---|---|
| `ServerType` | **Official** for official servers; **Private** for rAthena/Hercules. |
| `UseAttackSkill` | Allows automatic offensive skills. |
| `UseHomunSSkillAttack` / `UseHomunSSkillChase` | Allows Homunculus S skills while attacking / chasing. |
| `AggroHP` / `AggroSP` | HP/SP percentage needed to seek targets automatically. Raising these can reduce aggression. |
| `SuperPassive` | Prevents normal combat behavior; check it if nothing attacks. |
| `DoNotChase` | Stops chasing targets. A distant target may never enter skill range. |
| `KiteMonsters` | Enables keeping distance when tactics also allow it. |
| `UseDamageDanger` | Off by default. If the homunculus loses `DamageDangerDrop`% (30) of its max HP within 3 s while below `DamageDangerHP`% (60) HP, it kites away even with `KiteMonsters` off, and Bayeri casts Stein Wand. It only sees its own HP, not enemy buffs. |
| `UseDanceAttack` | Enables attack movement. It can affect positioning; it is not a skill-cooldown remover. |

Automatic server detection helps on some private clients, but select your
server explicitly when troubleshooting. Base-homunculus detection and learned
skill checks help with inherited skills; they do not replace learning them.

## 5. Skills, area attacks and SP

Enable the desired skill and set its level no higher than what the homunculus
has learned. A skill can remain unused because of:

- Insufficient SP or your SP reserve.
- Cooldown, range or the target's element.
- Area-skill requirements or monster tactics.
- Another usable skill having higher priority.
- A pending cast, combo or temporary failure pause.

`AutoMobMode` controls automatic area attacks. `AutoMobCount` sets the required
monster amount; a value of **1 can make area skills compete on a single target**.
Element effectiveness can also affect the count. `AoEMaximizeTargets` helps aim
area attacks; caster-centered skills still need monsters near the homunculus.

`AttackSkillReserveSP` leaves SP available instead of spending everything on
offense. `AutoSkillDelay` is an extra wait added by the AI. **0 removes only that
extra wait**, not server cooldowns, cast time or interruptions. Smaller values
are not always better. The bundled preset and the AI's built-in fallback values can
differ; the installed file's value is what matters.

## 6. Notes for each Homunculus S

| Homunculus | What to check |
|---|---|
| Eira | Enable the attacks you want, such as Twister Cutter or Absolute Zephyr. Light of Regenerate is optional and disabled by default: it requires Loyal intimacy, reduces intimacy on cast, and sacrifices Eira for the owner's resurrection. |
| Bayeri | **Glanzen Spies** is a targeted attack; **Heilige Pferd** is centered on Bayeri. The Bayeri preset enables Pferd and Spies and picks by the target's DEF (from `MobInfo.lua`): Spies only above about DEF 320, Stahl Horn otherwise; with exactly two monsters Stange/Pferd are used only from DEF 110. |
| Sera | Toxin of Mandara and Needle Stinger have casts that may be interrupted. Keep the target in range and check the failure-pause options below. Pain Killer and Summon Legion have their own switches. |
| Dieter | Enable the desired attacks/buffs individually. Fire resistance or immunity can prevent fire skills from being selected. |
| Eleanor | Sonic Claw and the fighting chain need fighting style; Tinder Breaker/CBC/EQC need grappling style. Spheres, SP and combo timing matter. The One Fighter Rises and Blazing And Furious are area skills: they follow `AutoMobMode`/`AutoMobCount` instead of being used on a single target, except The One Fighter Rises against a lone target with DEF 300 or more. With 3+ monsters she skips Sonic Claw chains, and Blazing And Furious is only cast with 6+ spheres. |
| Vanilmirth | **Caprice** (also inherited by Vanilmirth-based Homunculus S) is controlled by `UseVanilCaprice` (on by default). Chaotic Blessing follows `UseAutoHeal`. Self Destruct is never used automatically. |
These are configuration tips, not a ranking or an optimal build. Server rules
may differ, including skill ranges and targeting.

The bundled preset sets `AutoMobCount` to 2 and leaves **Twister Cutter** (Eira),
and **Needle Stinger** (Sera) off (the per-type presets in `Types/` turn on Bayeri's
Spies and Eleanor's Blazing And Furious), because the AI prefers them to cheaper skills that did at least as
well in offline tests, and turns **Blast Forge** (Dieter) on. These choices come
from a simulation, not from an official server: enable any of them if you prefer
it. Dieter's attacks are all area skills, so with `AutoMobCount` 2 he does not attack
a single monster with them: set `AutoMobCount` to 1 if you mostly fight one monster
at a time as Dieter (Bayeri and Sera then lose damage against single monsters).
Details are in the [technical notes](RELEASE_NOTES.md#default-preset-tuning) and the
[comparison tables](preset-tuning/README.md).

### Eleanor: combos or Sonic Claw only

For combos, set `AutoComboMode` to **Always** or **Tactics** and review
`AutoComboSpheres`: it is the sphere threshold to **start** an automatic chain,
not a requirement repeated for every follow-up. A high threshold means waiting
longer. New offensive skills can compete with the chain.

For a Sonic-only offensive experiment:

1. Enable `UseEleanorSonicClaw`.
2. Set `AutoComboMode` to **Never** and `Berserk_ComboAlways` off.
3. Disable Tinder Breaker, CBC, EQC, Blazing And Furious and The One Fighter Rises.
4. Put Eleanor in **fighting style** and reload the AI after saving.

Normal attacks and enabled buffs remain active. Manual style tracking depends
on what the client reports. If the game and AI disagree after a manual style
change, reload and test again; do not assume the
visible sphere icons are an exact real-time count.

`SphereTrackInterval` is an advanced estimate setting (default **800 ms**),
not an attack speed or cooldown setting. Reduce it only after verifying faster
normal attacks; too short can cause refused skills. An `H_Extra.lua` override
takes precedence over the GUI value.

## 7. Repeated cast interruptions

Find these options under **AutoSkill Options**:

| Option | Default | What it does |
|---|---|---|
| `CastInterruptLimit` | 3 | Temporarily skips a skill after this many counted failures. **0 disables** the feature. |
| `CastInterruptPause` | 5000 | Pause length in milliseconds: **5000 = 5 seconds**. |
| `CastInterruptUnconfirmed` | Off | Also counts failures without a visible cast/completion, with a living in-range target and sufficient SP. |

Normally, only observed offensive casts interrupted with damage evidence
count. Success resets that skill's counter. During its pause, another usable
skill or normal attacks can be chosen. Buffs, combos and manual skill requests
do not trigger this pause.

Use `CastInterruptUnconfirmed` only if the client hides casts and repeated
failures are not counted. **It may count server refusals as well as interruptions.**
Do not use it to conceal incorrect SP, range or skill settings.
Counters and pauses reset when the game reloads the AI.
These options do not make casts uninterruptible or change skill rules.

## 8. Monster tactics

Use the tactics tabs to decide which monsters to ignore, prioritize or fight,
and whether to use skills against them. Check both the default row and any
specific row for your target.

The web GUI can **Suggest tactics** by map or monster. Review suggestions before
applying: they come from a monster database that may differ from your server.

- `UseMobInfo`: uses the included monster information.
- `PlantSkills`: disabled by default to avoid wasting skills on plants.
- `BossAggro`: by default prevents initiating combat with MVPs, while allowing defense.
- `TargetPriority`: optional smarter ordering among targets.
- `KiteByRange`: optional range-aware distance behavior, with kiting enabled.

For a controlled dummy test, set that monster to **Ignore** for automatic
selection, then command the homunculus with **Alt + right-click**. This affects
all monsters sharing the same tactics ID, not just one dummy.

## 9. Troubleshooting

| Symptom | Check first |
|---|---|
| Nothing attacks | `/hoai`, installed folder, reload, passive mode, server type, tactics and HP/SP thresholds. |
| Manual attack works; automatic attack does not | Aggression settings, target's tactics, range and kill-steal restrictions. |
| New skill never appears | Learned level, enable switch, SP, range, area threshold and competing skills. |
| Cast starts repeatedly but fails | Damage interruptions, range, SP and the cast-pause options; do not assume every failure is an interruption. |
| GUI changes have no effect | Apply/export, actual game folder, reload and overrides in `H_Extra.lua`. |
| Eleanor uses the wrong chain | Style, combo settings and spheres; reload after changes. |

`AAIStartH.txt` in the client folder records homunculus AI startup.
`AAI_ERROR.log` can show errors and temporary offensive-skill pauses.
If startup files do not update, check folder permissions and custom-AI activation.
Detailed diagnostic steps are in the [technical notes](RELEASE_NOTES.md#troubleshooting-automatic-attacks).

For a report, include server/client, homunculus/base type, enabled skills,
reproduction steps and relevant sanitized logs. Remove account details,
passwords and private information. Never upload credential files.
Report through [Issues](https://github.com/jknpj/AzzyAI/issues).
