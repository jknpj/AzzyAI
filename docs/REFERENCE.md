# AzzyAI reference: commands, options and tactics

**English** | [Português (Brasil)](REFERENCE.pt-BR.md) | [Español](REFERENCE.es.md)

[Back to the manual](MANUAL.md) | [README](../README.md) | [Technical release notes](RELEASE_NOTES.md)

> **Getting started?** Use the [user manual](MANUAL.md) for installation, configuration and
> troubleshooting. This page is the detailed lookup for commands, options and tactics.

This reference updates material from the original AzzyAI 1.50 manual for release 2.0.0:

- Every option name, default and value was checked against `USER_AI/Defaults.lua`,
  `USER_AI/H_Config.lua` and `USER_AI/Const_.lua`. Where the original manual was wrong or out
  of date, this page follows the code (the differences are listed in [section 7](#7-what-changed-from-the-original-manual)).
- **Default** is the value in the shipped `H_Config.lua` (homunculus). `Defaults.lua` holds a
  second, built-in value that is used only when a config file lacks the option. Mercenaries have
  their own `M_Config.lua`.
- **New** marks an option that does not exist in the original AzzyAI.
- Your own `H_Extra.lua` / `M_Extra.lua` is loaded last and overrides what the GUI saved.
- Nothing here was re-tested in game for this page; the behavior is read from the code and from
  the original manual.

## Contents

1. [Commands in game](#1-commanding-in-game)
2. [Friends](#2-friends)
3. [Options](#3-options)
   - [Engaging and fighting](#options-engaging)
   - [Movement and idle behavior](#options-movement)
   - [Attack skills](#options-attack-skills)
   - [Buff skills](#options-buffs)
   - [Other skills](#options-other-skills)
   - [Berserk mode](#options-berserk)
   - [Kiting](#options-kiting)
   - [Timing and other](#options-timing)
4. [Tactics](#4-tactics)
   - [File format](#tactics-format)
   - [Basic tactic](#tactics-basic)
   - [Other tactic fields](#tactics-fields)
5. [Advanced settings](#5-advanced-settings)
6. [Installation notes](#6-installation-notes-from-the-original)
7. [Changes from the original](#7-what-changed-from-the-original-manual)
8. [Homunculus scripting primer](#8-homunculus-scripting-primer)

## 1. Commanding in game

The client lets you send a few commands to the AI. The AI receives commands, never chat text, so
there is no way to type a command to it.

| What you do | What the AI does | Related options |
|---|---|---|
| **Standby** key: **Alt+T** (homunculus), **Ctrl+T** (mercenary) | Toggles standby: the unit stops being aggressive. Press again to leave it. | `DefendStandby` (keeps defending you in standby), `StickyStandby` (goes back to standby after a defensive fight) |
| **Alt + right-click** on a cell | Moves there. Also how you [friend](#2-friends) someone. A move more than 15 cells away from you is ignored and logged as an error. | `MoveSticky`, `MoveStickyFight` |
| **Alt + double right-click** on a monster | Drops everything and attacks that monster. | `CommandedFightStay` (**new**), `UseBerserkAttack` |
| Use a skill **on a monster** | Attacks the target and opens with that skill. | `UseBerserkSkill` |
| Use a skill **on a player**, outside PVP | Moves into range and casts it on them. | |
| Use an untargeted or ground skill | Cast by the client without telling the AI, so the AI can lose track of that skill's delay and duration. | |
| **Sit** | Becomes non-aggressive and stays next to you. | `DoNotUseRest`, `RestXOff`, `RestYOff` |

The original manual names Alt+R for standby in one place; the key list above is the one the
shipped `A_Friends.lua` and the `DefendStandby` description use. Shortcuts can be changed in the
client, so check yours.

With `CommandedFightStay` on (default), a monster you ordered it to attack is chased and fought
until it dies or you give another order, even if you walk away. The move-bounds give-up and
`AttackTimeLimit` do not apply to that target. Set it to 0 to go back to the old behavior.

`AllowSBR44` (default 0) makes the AI block a manual cast of SBR44, to protect against misclicks.
It cannot do the same for Self Destruct.

## 2. Friends

A friended player is defended as if they were the owner, until you unfriend them. Friending
another player's mercenary or homunculus only lasts until they relog, teleport or summon it,
because its ID changes.

**Walk method** (`MirAIFriending` = 1, the default):

1. Command the homunculus to walk to the cell **one north or south** of the player.
2. It walks a **circle** around the target if the friend was added, or moves **back and forth in
   a line** if the friend was removed.

**Standby method** (`StandbyFriending` = 1, the older AzzyAI way): put the unit in standby, then
**you** sit one cell **west** of the person to add them, or one cell **east** to remove them.

**By hand:** `A_Friends.lua` holds the list, one line per player, using the account ID (from a
site such as ROPD):

```lua
MyFriends[2322797]=1       -- a friend
MyFriends[2322797]=KOS     -- PVP lists: NEUTRAL, KOS (kill on sight), ENEMY (hostile, not attacked), ALLY (friendly)
```

Keep `A_Friends.lua` when you update; it is not rebuilt.

| Option | Default | What it does |
|---|---|---|
| `AssumeHomun` | 1 | The homunculus and your mercenary friend each other automatically. Set 0 only if you do not want them to kill-steal each other. |
| `NewAutoFriend` | 1 | Automatic friending of your own mercenary and homunculus. In `H_Extra.lua`, set 0 if you use another AI for the other unit; then summon both and relog or teleport to trigger it. |
| `FriendAttack[...]` | see [5](#5-advanced-settings) | Which of a friend's motions (attacking, casting...) make the AI attack the monster they target. |

## 3. Options

Names are the ones used in the config files; the GUIs show the same names and have a search box.

<a id="options-engaging"></a>
### 3.1 Engaging and fighting

The unit normally goes after monsters its [tactics](#4-tactics) allow, when they are near you.

| Option | Default | What it does |
|---|---|---|
| `AggroHP`, `AggroSP` | 0, 0 | The unit only engages while its HP / SP percentage is **above** these values. 100 means it never starts a fight and only defends. (The built-in fallback for `AggroHP` is 60.) |
| `StationaryAggroDist`, `MobileAggroDist` | 12, 12 | Distance from you, in cells, within which it engages monsters while you stand still / move. |
| `SuperPassive` | 0 | 1 = never fight unless directly commanded. |
| `DoNotChase` | 0 | 1 = never walk to a monster. It still attacks at range with a ranged skill, and `CHASE_ALWAYS` tactics override it. |
| `DoNotAttackMoving` | 0 | 1 = ignore monsters that are moving (helps against kill-stealing and fast monsters; bad for AFK leveling). |
| `OpportunisticTargeting` | 0 | 1 = while chasing, re-select targets and switch to a better one if it shows up. |
| `AttackTimeLimit` | 10000 | Milliseconds spent attacking before it decides the target is unreachable and drops it. 0 = never. Not applied to a commanded target (`CommandedFightStay`). |
| `RescueOwnerLowHP` | 0 | Changes rescue behavior depending on your HP (see [Rescue tactics](#rescue)). |
| `FleeHP` | 0 | Only kite below this HP percentage. 0 ignores the condition. |
| `UseDanceAttack` | 1 | Homunculus only: "dance" while attacking normally for slightly faster attacks. Dancing stops SP regeneration while fighting, so with an attack skill you are usually better off without it. |
| `DanceMinSP` | 0 | Minimum SP to dance. Negative = percentage of max SP. |
| `TankMonsterLimit` | 4 | Most monsters it tries to hold for another killer with the `TACT_TANK` tactic. |
| `AutoDetectPlant` | 1 | **Mercenary only.** Treat monsters it never saw move or attack as plants and handle them with `MyTact[11]`. Set 0 when fighting stationary monsters. (The homunculus uses the monster database instead.) |
| `AttackLastFullSP` | 0 | 1 = `TACT_ATTACK_LAST` monsters are only attacked at full SP. |
| `UseAvoid` | 0 | **Homunculus only.** 1 = the client exits when a monster listed in `H_Avoid.lua` appears (summonable MVPs and some dangerous monsters by default). Test it first: on some Windows versions the exit shows an error window instead. |
| `UseDamageDanger`, `DamageDangerDrop`, `DamageDangerHP` | 0, 30, 60 | **New.** Kite away after a sudden HP drop, even with `KiteMonsters` off. See the [manual](MANUAL.md#4-basic-behavior). |
| `UseMobInfo`, `SkillElementMinRate`, `PlantSkills`, `BossAggro`, `AttackMVPs`, `AttackOnlyMVPs`, `TargetPriority` | 1, 50, 0, 1, 0, 0, 0 | **New.** The monster database options; explained in the [manual](MANUAL.md#what-the-monster-database-changes-homunculus-only). |

<a id="options-movement"></a>
### 3.2 Movement and idle behavior

| Option | Default | What it does |
|---|---|---|
| `FollowStayBack` | 2 | Cells it stays behind you when following. |
| `StationaryMoveBounds`, `MobileMoveBounds` | 14, 9 | How far from you it may go to react (you standing / moving). Past this it drops everything and returns. Keep both at **14 or less**. |
| `MoveSticky` | 0 | 1 = after a move command, stay there until told to move again. |
| `MoveStickyFight` | 0 | 1 = in that sticky state it still fights normally. |
| `StayAwayDistance` | 0 | **New.** Keep about this many cells from you while idle (limited by the move bounds). 0 = off. Useful in instances where the homunculus tanks. |
| `StayAwayHold` | 0 | **New.** 1 = park at one spot and return to it after fights instead of following you. |
| `UseIdleWalk` | 4 | Walk around while idle, when HP is above `AggroHP`, SP is above `AggroSP` and above `IdleWalkSP` percent. 0 off; 1 circle around you; 2 cross; 3 rectangle; 4 random; 5 follow `MyRoute` back and forth; 6 follow `MyRoute` in a loop. |
| `IdleWalkDistance` | 4 | Distance from you for walks 1 to 4. |
| `IdleWalkSP` | 80 | SP percentage it must exceed to idle-walk. |
| `RelativeRoute` | 1 | 1 = `MyRoute` cells are offsets from you; 0 = absolute map cells. See [Route walk](#5-advanced-settings). |
| `DoNotUseRest` | 0 | 1 = do not go non-aggressive when you sit. |
| `RestXOff`, `RestYOff` | 2, 0 | Offset from you where it waits when you sit (can be negative). |
| `ChaseSPPause` | 0 | 1 = delay chasing and following until just after an SP regeneration tick. A monster tactic of `CHASE_ALWAYS` ignores it, `CHASE_CLEVER` forces it. |
| `ChaseSPPauseSP` | -60 | Only pause below this SP. Negative = percentage. |
| `ChaseSPPauseTime` | 3000 | Longest wait, in ms, for the next SP tick. |

<a id="options-attack-skills"></a>
### 3.3 Attack skills

| Option | Default | What it does |
|---|---|---|
| `UseAttackSkill` | 1 | Automatic offensive skills. |
| `UseSkillOnly` | -1 | **-1 (Chase, recommended)** uses skills while chasing and attacking. **0** only while attacking. **1** attacks only with skills, and will not use a ranged skill before it is in melee range. |
| `UseHomunSSkillChase`, `UseHomunSSkillAttack` | 1, 1 | Allow the single-target Homunculus S skills while chasing / attacking. Turn them off when many of your skills have interruptible casts and you cannot flee. |
| `AutoMobMode` | 2 | Area skills: 0 off; 1 counts only monsters already attacking; 2 counts every monster the area would hit. |
| `AutoMobCount` | 2 | Monsters (weighted by the [Weight tactic](#weight)) needed before an area skill is used. 1 lets an area skill compete even on a single target. |
| `AoEFixedLevel` | 0 | 1 = always cast area skills at the level set in the skill's `...Level` option, ignoring the tactics' skill-level settings. |
| `AoEMaximizeTargets` | 0 | 1 = aim area skills at the spot that hits the most monsters, even if that misses the current target. |
| `AoEReserveSP` | 0 | 1 = never spend SP on another attack if that leaves too little for the area skill. |
| `AttackSkillReserveSP` | 0 | SP kept in reserve: no offensive skill is used if it would leave less than this. Overridden per monster by the [SP tactic](#sp-tactic). |
| `AutoSkillLimit` | 0 | Most times an offensive skill is used on one target. 0 = no limit. Tactics can override. |
| `AutoSkillDelay` | 200 | Extra wait, in ms, between skills, to avoid double casting. 0 removes only this wait, not cooldowns or cast time. |
| `CastTimeRatio` | .80 | Fraction of the skill's cast time the AI expects, for your cast-time reductions. |
| `AutoComboMode` | 2 | Combo skills (Eleanor): 0 never; 1 only for monsters whose skill class is a combo class; 2 always. |
| `AutoComboSpheres` | 1 | Spheres needed to **start** an automatic chain. |
| `UseVanilCaprice` | 1 | Vanilmirth: use Caprice. |
| `LintUserOptions` | 1 | **New.** At startup, turns off skill options for skills the homunculus has not learned and lowers levels to the learned level. |
| `TrustSkillProbe` | -1 | **New.** Whether to believe the client's answer about learned skill levels. -1 = automatic (not on the official server, yes on private servers). |
| `ServerType`, `AutoDetectServerType` | official, 1 | **New.** Official server or rAthena/Hercules. See the [manual](MANUAL.md#4-basic-behavior). |

**Skill switches and levels.** Each skill has `Use<Type><Skill>` (1 = on) and `<Type><Skill>Level`.
The AI will not use a skill above the learned level: with `LintUserOptions` it lowers configured levels to the
learned one at startup.

| Homunculus | Attack skill options |
|---|---|
| Eira | `UseEiraXenoSlasher`, `UseEiraEraseCutter` (Eraser Cutter), `UseEiraTwisterCutter`, `UseEiraAbsoluteZephyr`, `UseEiraSilentBreeze` |
| Bayeri | `UseBayeriStahlHorn`, `UseBayeriHailegeStar`, `UseBayeriGlanzenSpies`, `UseBayeriHeiligePferd` |
| Sera | `UseSeraParalyze` (Needle of Paralyze), `UseSeraPoisonMist`, `UseSeraCallLegion`, `UseSeraToxinOfMandara`, `UseSeraNeedleStinger` |
| Dieter | `UseDieterLavaSlide`, `UseDieterVolcanicAsh`, `UseDieterAshAsAttack`, `UseDieterBlastForge` |
| Eleanor | `UseEleanorSonicClaw`, `EleanorSilverveinLevel`, `EleanorMidnightLevel`, `UseEleanorTinderBreaker`, `UseEleanorCBC`, `UseEleanorEQC`, `UseEleanorBlazingAndFurious`, `UseEleanorTheOneFighterRises` |

Silvervein Rush and Midnight Frenzy have only a level, not a switch. Notes for choosing among
them are in [manual section 6](MANUAL.md#6-notes-for-each-homunculus-s). `LavaSlideMode` and
`PoisonMistMode` (default 0) choose how those two skills are used: 0 casts them as attacks; any
other value keeps them up around you as a buff, and the value says when to cast them (the values
in 3.4).

<a id="options-buffs"></a>
### 3.4 Buff skills

A buff option takes one of these values, which say **when** the skill may be used:

| Value | When |
|---|---|
| -2 | When idle, if there is nothing else to do |
| -1 | While chasing (so it is not wasted when there is nothing to kill) |
| 0 | Never |
| 1 | While idle |
| 2 | While chasing or attacking in Berserk mode |
| 3 | As soon as possible, interrupting other actions. The current target is dropped only for Painkiller and Provoke. |

For Homunculus S buffs there is no level option, because higher levels are better. For base
homunculi there is one, because higher levels have longer cooldowns.

| Option | Default | Controls |
|---|---|---|
| `UseOffensiveBuff` | 3 | Flitting, Mental Charge, Weapon Quicken, Bloodlust |
| `UseDefensiveBuff` | 3 | Amistr Bulwark, Accelerated Flight, Guard, Parrying, Urgent Escape |
| `UseAutoSight`, `UseAutoMag` | 1, 0 (built-in) | **Mercenary only:** Sight (level 2 archer), Magnificat (level 4 archer) |
| `UseDieterGraniticArmor`, `UseDieterMagmaFlow`, `UseDieterPyroclastic` | 0, 3, 3 | The Dieter skill of that name (`DieterPyroclasticLevel`, default 10) |
| `UseDieterTempering` | 3 | **New.** Dieter's Tempering |
| `UseEiraOveredBoost` | 0 | Eira's Overed Boost |
| `UseBayeriGoldenPherze`, `UseBayeriAngriffModus`, `UseBayeriGoldeneTone` | 0, 3, 3 | Bayeri's Golden Ferse, Angriff Modus and Goldene Tone |
| `UseProvokeOwner` | 0 | Provoke (mercenary) or Pain Killer (Sera) on you |
| `ProvokeOwnerMobbed` | 3 | Not used on you with fewer monsters than this on you, unless you are below 90% HP (the Weight tactic counts) |
| `UseSeraPainkiller` | 3 | Sera's Pain Killer |
| `UseSacrificeOwner` | 0 (built-in) | **Mercenary only:** Sacrifice on you. Works within 10 levels of the player and is unreliable because of game bugs. |
| `UseSmartBulwark` | 0 | Amistr: cast Bulwark only if at least 120 SP remain, so Bloodlust is possible later. Ignored below 160 max SP or without Bloodlust. |
| `AmiBulwarkLevel`, `FilirFlitLevel`, `FilirAccelLevel`, `LifEscapeLevel` | 5, 5, 5, 0 | Level of Amistr Bulwark, Filir Flitting, Filir Accelerated Flight, Lif Urgent Escape. |
| `UseEiraLightOfRegene` | 0 | **New.** Eira sacrifices herself to resurrect you. See the [manual](MANUAL.md#6-notes-for-each-homunculus-s). |

`LifEscapeLevel = 0` suppresses Urgent Escape: the AI skips the cast rather than sending
level 0 to the server. This is safe, contrary to the original manual's warning.
Use a learned level from 1 to 5 to enable it; `UseDefensiveBuff` decides when it is used.

**Healing** (Lif Healing Hands, Vanilmirth Chaotic Blessing; homunculus):
`UseAutoHeal` = 0 off, 1 on, 2 only when idle (including between kills), 3 only when idle and nothing else to do.
`HealOwnerHP` (50) is your HP percentage below which it heals you (Lif casts Healing Hands level 5, a Vanilmirth
Chaotic Blessing level 3). `HealSelfHP` (50) is the Vanilmirth's own HP percentage for healing itself (level 4;
level 5 when idle). Lif cannot heal itself.

<a id="options-other-skills"></a>
### 3.5 Other skills

| Option | Default | What it does |
|---|---|---|
| `UseAutoPushback`, `AutoPushbackThreshold` | 0, 2 | **Mercenary only.** Push monsters off the mercenary with a pushback skill (Arrow Repel, Skid Trap) when they are this close. Must also be allowed by the Pushback tactic. |
| `UseCastleDefend`, `CastleDefendThreshold` | 0, 4 | Amistr: use Castling when at least this many monsters (Weight counts) are on you and fewer on the Amistr. |
| `UseCastleRoute` | 0 | Amistr: use Castling to walk you along an absolute route (needs `UseIdleWalk` 5 or 6 and `RelativeRoute` 0). |
| `UseBayeriSteinWand` | 1 | Bayeri: Stone Wall when a big enough mob attacks you or him (buff values, 3.4). |
| `UseSteinWandOwnerMob`, `UseSteinWandSelfMob` | 2, 2 | Monsters on you / on the Bayeri needed (Weight counts). |
| `BayeriSteinWandLevel` | 5 | Level of Stone Wall. |
| `UseSteinWandTele`, `SteinWandTelePause` | 0, 3000 | Cast Stone Wall right after you log in or land from a teleport, then not again. Pairs well with `DoNotChase` and Bayeri's attack skill; do not combine with `UseBayeriSteinWand`. |

<a id="options-berserk"></a>
### 3.6 Berserk mode

Berserk mode makes the unit fight more aggressively after a command or against a big mob. After a
command it ends when the target dies. **The working spelling is `Berserk`** (the original manual writes
"Berzerk").

| Option | Default | What it does |
|---|---|---|
| `UseBerserkSkill` | 0 | Enter berserk mode when commanded to use an offensive skill. |
| `UseBerserkAttack` | 0 | Enter berserk mode when commanded to attack. |
| `UseBerserkMobbed` | 0 | 0 = off. Otherwise go berserk while this many monsters or more are fighting you and your friends. |
| `Berserk_SkillAlways` | 0 | Always use offensive skills in berserk mode. |
| `Berserk_Dance` | 0 | Dance attack in berserk mode (homunculus only). |
| `Berserk_IgnoreMinSP` | 0 | Ignore `AttackSkillReserveSP` in berserk mode. |
| `Berserk_ComboAlways` | 0 | Always allow combos in berserk mode. |

<a id="options-kiting"></a>
### 3.7 Kiting

Kiting means running away from monsters while attacking them, mostly for archer mercenaries. The
defaults were chosen empirically: change them only with a reason.

| Option | Default | What it does |
|---|---|---|
| `KiteMonsters` | 0 | Enable kiting. The monster's Kite tactic must allow it too. |
| `KiteParanoid` | 0 | 1 = kite before being attacked. Tactics can override. |
| `KiteStep` | 5 | Cells moved when kiting. |
| `KiteThreshold` | 3 | How close a monster must be to trigger kiting once it has attacked. |
| `KiteParanoidStep`, `KiteParanoidThreshold` | 2, 2 | The same for kiting before being attacked. |
| `KiteBounds` | 10 | Farthest it moves from you to kite. Must be less than the move bounds. |
| `ForceKite` | 0 | Allow kiting for a homunculus without a ranged attack. |
| `KiteByRange` | 0 | **New.** Kite by the monster's real attack range. See the [manual](MANUAL.md#what-the-monster-database-changes-homunculus-only). |

<a id="options-timing"></a>
### 3.8 Timing and other

| Option | Default | What it does |
|---|---|---|
| `SpawnDelay` | 1000 | Milliseconds it waits before acting after spawning. Needed so kill-steal protection works while teleporting. |
| `PVPmode` | 0 | 1 = enable PVP behavior ([PVP tactics](#5-advanced-settings)). |
| `CastInterruptLimit`, `CastInterruptPause`, `CastInterruptUnconfirmed` | 3, 5000, 0 | **New.** Pause a skill that keeps being interrupted. See the [manual](MANUAL.md#7-repeated-cast-interruptions). |
| `SphereTrackInterval`, `SphereTrackFactor`, `EleanorComboGap`, `EleanorComboDance`, `EleanorDoNotSwitchMode` | 800, 2, 500, 1, 0 | Eleanor's sphere estimate and combo pacing. See the [manual](MANUAL.md#eleanor-combos-or-sonic-claw-only). |
| `OldHomunType` | 2 | For a Homunculus S, what it was before: 1 Lif, 2 Amistr, 3 Filir, 4 Vanilmirth. It is detected automatically when possible and the option is only a fallback. |

Eleanor does not infer a style change from a refused starter: missing spheres,
range or timing can also cause refusals. Recovery may probe an enabled starter
of the other style against the same target, at most once per five seconds.
Only confirmed use changes the tracked style. Style Change also requires
confirmation; an AI-call pause alone is not sufficient. Combo dancing is unchanged.
On AI initialization and detected resurrection, enabled Sonic Claw selects
Fighting as the preferred style (including when both starters are enabled);
Tinder Breaker alone selects Grappling only when automatic combos are enabled
(`AutoComboMode` nonzero or `Berserk_ComboAlways=1`). With combos disabled,
enabled AoE skills prefer Fighting to build spheres through normal attacks.
`EleanorDoNotSwitchMode=1` prevents
automatic preference changes. Detected death clears pending casts, targets,
combo windows and estimated spheres; resurrection rebuilds those spheres.
The death marker survives AI reloads. If the client never reports the dead
state, the AI cannot identify resurrection from a pause alone.

## 4. Tactics

Tactics decide how each kind of monster is treated: ignore plants, prioritize dangerous ones, save
SP for others. They live in `H_Tactics.lua` and `M_Tactics.lua` and can be edited in the GUIs.

A homunculus also uses the monster database for monsters without a row (see the manual); a
**mercenary cannot identify monsters**, so besides the default row it needs a MobID file for the
map and server (`Mob_ID.lua`) before any specific row works. Only the default row, `MyTact[10]`
and `MyTact[11]` work without one.

<a id="tactics-format"></a>
### 4.1 File format

```lua
MyTact[id] = {TACT_BASIC, TACT_SKILL, TACT_KITE, TACT_CAST, TACT_PUSHBACK, TACT_DEBUFF,
              TACT_SKILLCLASS, TACT_RESCUE, TACT_SP, TACT_SNIPE, TACT_KS, TACT_WEIGHT, TACT_CHASE}
```

`id` is the monster's type ID (from a database site). There can be only one row per monster. Special IDs:

| id | Used for |
|---|---|
| `MyTact[0]` | The default row |
| `MyTact[10]` | Monsters detected as summoned |
| `MyTact[11]` | Monsters that look like plants, when `AutoDetectPlant` is on (mercenary only) |

The shipped default row is `{TACT_ATTACK_H, SKILL_ALWAYS, KITE_NEVER, CAST_REACT, PUSH_NEVER, DEBUFF_NEVER, CLASS_S, RESCUE_FRIEND, -1, SNIPE_DISABLE, KS_NEVER, 1, CHASE_CLEVER}`.
In the GUI: open the tactics tab, **Add** (the name is cosmetic, the ID is what counts), pick each
field from the drop-downs, **Remove** to delete a row, and **Apply** to save. Deleted rows fall back to the default row.

<a id="tactics-basic"></a>
### 4.2 Basic tactic (`TACT_BASIC`)

When and whether to attack a monster.

| Constant | Value | Meaning |
|---|---|---|
| `TACT_IGNORE` | 0 | Never attack unless commanded. |
| `TACT_TANK` | -1 | Hit once, then hold it until something else kills it. |
| `TACT_TANKMOB` | -2 | Like Tank, but if enough monsters attack the unit, attack one and use area skills on the mob. |
| `TACT_ATTACK_L` | 2 | Attack only if nothing of higher priority exists, and do not prioritize it even when it attacks you or a friend. |
| `TACT_ATTACK_M` | 3 | Seek out and attack, unless something has higher priority. |
| `TACT_ATTACK_H` | 4 | Seek out and attack, unless busy responding to a React monster. |
| `TACT_ATTACK_TOP` | 15 | Seek out and attack, top priority. |
| `TACT_ATTACK_LAST` | 14 | Attack only when there is nothing better to do (see `AttackLastFullSP`). Good for plants, event monsters and other worthless targets. |
| `TACT_REACT_L` | 5 | Attack when you, a friend or the unit are attacked; low priority. Attack Medium and High come first. |
| `TACT_REACT_M` | 7 | The same, medium priority. |
| `TACT_REACT_H` | 8 | The same, high priority. |
| `TACT_REACT_SELF` | 9 | React only when **the unit itself** is attacked, with top priority (except Attack Top). For monsters that would be a threat to it. |
| `TACT_ATK_L_REACT_M` | 13 | Attack Low while the monster is not attacking you or a friend; React Medium once it is. Good for weak assist monsters. |
| `TACT_SNIPE_L/M/H` | 10, 11, 12 | **Sniping**: treated as attack for normal attacks, but if the unit is fighting another monster it casts an attack skill on a nearby sniping monster it is **not** fighting, as long as no area skill is better. It works best with a quick, cheap skill such as Caprice. If the skill does not kill in one hit, it aggros extra monsters. |

**Priority order**, highest first (from the original manual): Attack Top, React Self, React High,
React Medium, Attack High or Snipe High, Attack Medium or Snipe Medium, React Low, Attack Low or
Snipe Low, Tank or Tank & Mob, Attack Last. This version adds `BossAggro`, `AttackMVPs`,
`AttackOnlyMVPs` and `TargetPriority` on top of it (manual, section 8).

<a id="tactics-fields"></a>
### 4.3 The other fields

<a id="skill-tactic"></a>**Skill (`TACT_SKILL`)**: how many times an offensive skill is used on this monster.

| Value | Meaning |
|---|---|
| `SKILL_NEVER` (0) | Never use skills. |
| `SKILL_ALWAYS` (100) | Always use skills. |
| a positive number N | Use a skill at most N times. |
| a negative number -N | One cast, at level N (homunculus only). The right setting for sniping with a low-level bolt. |

**Kite (`TACT_KITE`)**: `KITE_NEVER` (0); `KITE_REACT` (1) only once attacked; `KITE_ALWAYS` (2)
always, recommended for aggressive monsters. Only matters when kiting is on.

**Cast (`TACT_CAST`)**: `CAST_REACT` (1) treats the monster's casts on you, a friend or itself as hostile;
`CAST_PASSIVE` (0) does not. Mostly useful to stop the unit killing summoned Geographers.

**Pushback (`TACT_PUSHBACK`)**: mercenaries with a pushback skill only (the only homunculus skill with a
pushback is Stahl Horn, which is useless for it because of its cast time). `PUSH_NEVER` (0),
`PUSH_SELF` (1) when the mercenary is attacked, `PUSH_FRIEND` (2) when you or a friend are.

**Debuff (`TACT_DEBUFF`)**: which debuff is used (mercenary, plus Eira's Silent Breeze and Dieter's
Volcanic Ash). In the GUI choose the skill and whether to use it while attacking or chasing. In the file use
the skill's constant: a positive one (`DEBUFF_CRASH_A`, `DEBUFF_PROVOKE_A`, `DEBUFF_SANDMAN_A`, `DEBUFF_FREEZE_A`,
`DEBUFF_DECAGI_A`, `DEBUFF_LEXDIV_A`, `DEBUFF_BREEZE_A`, `DEBUFF_ASH_A`) while attacking, or the `_C` one
while chasing. `DEBUFF_ANY_A` (1) / `DEBUFF_ANY_C` (-1) use whatever the entries of `BasicDebuffs` allow.
`DEBUFF_NEVER` (0) uses none.

**Skill class (`TACT_SKILLCLASS`, called `TACT_SIZE` in old files)**: which kind of single-target skill to use.

| Constant | Value | Meaning |
|---|---|---|
| `CLASS_BOTH` | -1 | Either kind. |
| `CLASS_OLD` | 0 | Only the pre-S skills (Moonlight, Caprice). |
| `CLASS_S` | 1 | Only Homunculus S skills. On a mercenary this means no skills at all; do not use it. |
| `CLASS_MOB` | 2 | Use area skills even without enough monsters around. |
| `CLASS_COMBO_1` | 3 | Eleanor combos: use the second combo skill after the first, not the third. Her combos do less damage than spamming Sonic Claw with 10 spheres, so use this only when another effect of the combo is wanted. |
| `CLASS_COMBO_2` | 4 | Complete the combo. |
| `CLASS_MINION` | 5 | Summon minions on this target (Sera's Call Legion). Works well with a snipe tactic: the minions kill while the unit goes elsewhere. |
| `CLASS_GRAPPLE` | 6 | **New.** Eleanor's grappling chain: Tinder Breaker only. |
| `CLASS_GRAPPLE_1` | 7 | **New.** Tinder Breaker, then C.B.C. |
| `CLASS_GRAPPLE_2` | 8 | **New.** The whole chain: Tinder Breaker, C.B.C., E.Q.C. |

<a id="rescue"></a>**Rescue (`TACT_RESCUE`)**: whether the unit drops everything to save you, your friends or your other
retainer from this monster. `RESCUE_NEVER` (0), `RESCUE_FRIEND` (1) friends but not you, `RESCUE_RETAINER` (2) your other
homunculus or mercenary, `RESCUE_SELF` (3) defend itself, `RESCUE_OWNER` (4) you, `RESCUE_ALL` (5) all of those except itself.
`RescueOwnerLowHP` adds an HP condition.

<a id="sp-tactic"></a>**SP (`TACT_SP`)**: a positive number replaces `AttackSkillReserveSP` for this monster. Otherwise (-1) the
option is used. Set a higher value on weak monsters to keep SP for the strong ones without turning skills off for them.

**Snipe (`TACT_SNIPE`)**: `SNIPE_OK` (1) other monsters may be sniped while fighting this one; `SNIPE_DISABLE` (0) they may not, so this one dies faster.

**KS (`TACT_KS`)**: how polite the unit is about this monster.

| Constant | Value | Meaning |
|---|---|---|
| `KS_NEVER` | 0 | Do not attack it if it is attacking or being attacked by another player. |
| `KS_POLITE` | -1 | Do not attack it while it is moving, as it may be chasing someone (like `DoNotAttackMoving`). |
| `KS_ALWAYS` | 1 | Free-for-all: attack without regard for other players. You are responsible for what your unit does: using this on monsters that are not free-for-all can get your account sanctioned. Strongly discouraged. |

<a id="weight"></a>**Weight (`TACT_WEIGHT`)**: a number, default 1. Wherever monsters are counted (area skills, berserk,
Stein Wand, Castling, Provoke...), this monster counts as that many. Two monsters of weight 0.4, one of 0.5 and one of
1.2 count as 2 x 0.4 + 0.5 + 1.2 = 2.5.

**Chase (`TACT_CHASE`)**: like `DoNotChase`, for this monster only.

| Constant | Value | Meaning |
|---|---|---|
| `CHASE_NORMAL` | -1 | Follow `DoNotChase`. |
| `CHASE_ALWAYS` | 0 | Chase it even with `DoNotChase` = 1. A good choice for a Vanilmirth in OD2: set the orc archers to `CHASE_ALWAYS` and `DoNotChase` = 1 so it moves only when it must. |
| `CHASE_NEVER` | 1 | Never chase it, even with `DoNotChase` = 0. It can still be targeted, so use it with care. |
| `CHASE_CLEVER` | 2 | Wait for an SP tick (`ChaseSPPause`) when chasing it, even if that option is off. |

## 5. Advanced settings

These live in `H_Extra.lua` / `M_Extra.lua`, a Lua file loaded after everything else. It overrides the
GUI, so check it first when a saved change does nothing. The shipped files hold commented examples.

| Setting | What it does |
|---|---|
| `NewAutoFriend = 0` | Uncomment if you use another AI for your other unit (see [Friends](#2-friends)). |
| `FriendAttack[MOTION_...]` | Set to 1 for each motion of a friend or you that counts as an attack cue: attacking, using a skill, casting, tossing (Spear Boom, potions), Acid Bomb, Full Blast. |
| `BasicDebuffs[...]` | Which debuffs are used when debuffing is on but the tactic does not name one (default: everything except traps and Provoke). 1 turns one on. |
| `AggressiveRelogTracking = 1`, `AggressiveRelogPath` | Records relogs by constantly writing a file, so skill durations are tracked across them. It causes a lot of disk activity and can slow down some systems; do not use it without a fast disk. |
| `MyRoute={{x1,y1},{x2,y2},...}` | The route for `UseIdleWalk` 5 or 6: a list of walkable cells, each within the move bounds of its neighbours. With `RelativeRoute` 1 the cells are offsets from you. A long route may need many points. |
| `GetMyTact(e)` | A commented example of switching between two tactics lists on a condition. Not guaranteed to work. |
| `LogEnable["AAI_..."]=1` | More logging, written as `AAI_*.log` in the client folder: `AAI_SKILLFAIL`, `AAI_CLOSEST`, `AAI_DANCE`, `AAI_ACTORS`, `AAI_TARGETING` (a snapshot every 5 s), `AAI_MOBCOUNT`, `AAI_PROBE` (raw skill-probe answers). `AAI_ERROR` is on by default; `LogEnable["AAI_ERROR"]=0` hides it, but if it fills up, report that. |

**PVP.** With `PVPmode` = 1, other players are handled by `H_PVP_Tact.lua` / `M_PVP_Tact.lua`, edited like normal
tactics but without the SP, Chase, Weight and KS fields. Mark players either by adding a row with the player's account
ID, or by adding a line to `A_Friends.lua` with `KOS`, `ENEMY`, `NEUTRAL`, `FRIEND` or `ALLY` and giving those classes
a PVP tactic row.

## 6. Installation notes from the original

- **Windows with UAC (Vista and later):** if Ragnarok Online is in a protected folder, such as
  *Program Files*, the AI may not be able to run properly. Move the whole RO folder somewhere unprotected, such
  as `C:\Games`, delete the contents of `USER_AI` and install again. This is unnecessary if UAC is off.
- **Updating:** keep `A_Friends.lua`. For the other files see the [manual](MANUAL.md#updates-and-backups).
- **Check that it works:** `AAIStartH.txt` / `AAIStartM.txt` appears in the RO folder when the AI runs. If it
  does not, the AI is not installed correctly.

## 7. What changed from the original manual

| Topic | Original 1.50 manual | This version |
|---|---|---|
| Servers | Illegal private servers are not supported. | rAthena/Hercules can be selected with `ServerType` (tested offline and on a local rAthena only, not on every server). Follow your server's rules for custom AI. |
| Standby key | Alt+R in one place, Alt+T in another. | Alt+T (homunculus) and Ctrl+T (mercenary), as in `A_Friends.lua`. |
| Berserk options | Written `Berzerk`. | The working names are `UseBerserk...` and `Berserk_...`. |
| `UseBayeriHailageStar`, `UseBayeriGoldenFerse`, `UseStienWandTele`, `FilerFlitLevel` | | `UseBayeriHailegeStar`, `UseBayeriGoldenPherze`, `UseSteinWandTele`, `FilirFlitLevel`. |
| Route walk | `UseRouteWalk` and a `Route.lua` file. | `UseIdleWalk` 5 or 6 and `MyRoute` in `H_Extra.lua` / `M_Extra.lua`. |
| `OldHomunType` | Set in `H_Extra.lua`; Vanilmirth detected automatically. | Set in `H_Config.lua`, as a fallback; the base type is detected from the inherited skills when one is learned. |
| `AoEFixedLevel` | Default 1. | Default 0 in the shipped `H_Config.lua`. |
| `AggroHP` | Default 60. | 0 in the shipped `H_Config.lua`, so it engages at any HP. |
| `UseAutoSkill_MinSP` (SP tactic) | | `AttackSkillReserveSP`. |
| `AutoMobMode` | Not explained. | 0 off, 1 counts attackers only, 2 counts all. |
| PVP tactics files | `H_PVP_Tactics.lua` | `H_PVP_Tact.lua`, `M_PVP_Tact.lua` |
| Eleanor skill classes | `CLASS_COMBO_1/2` | Plus `CLASS_GRAPPLE`, `_1`, `_2`. |
| New options | | `ServerType`, `AutoDetectServerType`, `TrustSkillProbe`, `LintUserOptions`, `UseMobInfo` and the monster database options, `TargetPriority`, `KiteByRange`, `UseDamageDanger`, `CommandedFightStay`, `StayAwayDistance`, `StayAwayHold`, `CastInterrupt*`, the Homunculus S skill options, `Types/<Name>.lua` per-homunculus presets. |
| Version history | 1.00 to 1.50 changelog. | Not repeated here. Changes since the original 1.56 are in the [release notes](RELEASE_NOTES.md#changes-since-the-original-156-release-200). |

<a id="8-homunculus-scripting-primer"></a>
## 8. Homunculus scripting primer

The client-driven AI can be understood as a state machine: it idles, follows its owner, chases
a target, or attacks. This is only a learning model—the current AzzyAI has many additional
states and behaviors. Its homunculus entry point is `AI.lua`; this project separates shared
constants, utility functions, and behavior across files such as `Const_.lua`, `AzzyUtil.lua`,
and `AI_main.lua`.

Some client-provided functions used by the current code:

| Function | Purpose |
|---|---|
| `GetV(V_..., id)` | Reads a supported property of an in-game object, such as position, HP, SP, or motion. |
| `GetActors()` | Gets IDs of objects currently visible to the client. |
| `GetTick()` | Gets the client's millisecond timer, useful for scheduling and timeouts. |
| `IsMonster(id)` | Checks whether an object is classified as a monster. |
| `GetMsg(id)`, `GetResMsg(id)` | Receives structured commands from the client interface; these are not arbitrary chat messages. |
| `Move(id, x, y)`, `Attack(id, target)` | Requests movement or an attack. AzzyAI wraps these calls to apply its own checks and behavior. |
| `SkillObject(id, level, skill, target)`, `SkillGround(id, level, skill, x, y)` | Requests a targeted or ground skill. Client and server rules still determine whether it succeeds. |

This list is a starting point, not a complete or server-independent API contract. Client
implementations differ, and AzzyAI's wrappers add behavior beyond the underlying client calls.
For extensions, prefer `H_Extra.lua` and the documented configuration/tactics files over
replacing core AI files. The older guide's setup snippets target a different file layout; do
not copy them directly into this release.

**Credit:** This short, original summary is informed by the historical
[Homunculus AI Script User's Guide](https://github.com/maxmx03/YggAI/blob/main/Homunculus%20AI%20Script%20User%20Guide.md),
which credits Gravity as the original author and Max Del Canto for its 2024 update. The guide
is hosted in [YggAI](https://github.com/maxmx03/YggAI); the notes above are paraphrased and
adapted to AzzyAI 2.0, whose current source is the reference for names and behavior.

---

[Manual](MANUAL.md) · [README](../README.md) · [Português (Brasil)](REFERENCE.pt-BR.md) · [Español](REFERENCE.es.md)
