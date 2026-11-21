# AzzyAI for Pre-Renewal servers

This branch is the **Pre-Renewal build** of AzzyAI. It targets servers of the
episode 13.2 era (the last Pre-Renewal episode), not the Renewal game.

It is not "Renewal with some options off": the Renewal-only data, skills and
options are not shipped, so the AI cannot load them by accident.

## What is in it

- The four original homunculi: **Lif, Amistr, Filir and Vanilmirth**.
- Mercenaries, with the same mercenary AI as the main build (`M_Config.lua`,
  `M_SkillList.lua`, `M_Tactics.lua`). Servers that removed mercenaries simply
  never see one.
- The whole AI engine: targeting, kiting, stay-away, kiting by attack range,
  tactics per monster class, party/friend handling, cast interruption
  protection, relog recovery, and every option that does not belong to a
  Homunculus S skill.

## What is not in it

| Renewal feature | Why it is gone |
|---|---|
| Homunculus S (the five Renewal-only types) | Introduced with Renewal. rAthena's `db/pre-re/skill_db.yml` does not even define nine of their skills (Xeno Slasher, Blast Forge, Twister Cutter, Toxin of Mandara, Heilige Pferd and others). |
| ~80 Homunculus S options and 13 enums | They only existed to drive the S skills and buffs. The GUIs do not show them. |
| `SkillDamage` and `SkillDamageBaseLevel` | The Renewal damage formula adds `skill level * caster base level / divisor`, which does not exist in Pre-Renewal. |
| Renewal monster database | Replaced by the Pre-Renewal one (see below). |
| Renewal element table | Replaced by the classic `attr_fix` (for example fire on water is 50%, not 90%). |
| Homunculus S skill timings | No S skills to time. |

Because `SkillDamage` is absent, the AI chooses attack skills the classic way:
by skill-type priority and the options (`UseAttackSkill`, tactics, SP limits,
element checks against the monster database). There is no damage-per-second
scoring, and no base-level scaling anywhere.

## Data

`USER_AI/MobInfo.lua` is generated from rAthena's Pre-Renewal database:

```
cargo run --release -p rathena-sync --manifest-path tools/Cargo.toml -- --era pre
```

That reads `db/pre-re/mob_db.yml`, `db/pre-re/attr_fix.yml` and
`db/pre-re/skill_db.yml` from the rAthena commit named in the file header, and
writes:

- `USER_AI/MobInfo.lua`: 1004 monsters (race, element and element level, size,
  boss/MVP/plant/immobile flags, attack range, DEF, MDEF), the classic element
  table and the element of each homunculus skill that deals damage;
- `webgui/mobdata.js`: the Pre-Renewal spawns (monsters and the maps they are
  on), from `npc/pre-re/mobs`.

Pre-Renewal content is frozen: gravity stopped adding to it in 2011, so this
data does not need to follow game updates. Regenerate it only when the
generator itself changes.

## GUI

The `webgui` submodule has its own `prerenewal` branch: 105 homunculus options
instead of 186, 13 skills instead of 50, and the Pre-Renewal monster list. The
Windows application embeds that same `webgui/data/options.json`, so both GUIs
always match.

To refresh the GUI data after changing the schema:

```
cd webgui
node tools/dump-data.js
```

## Tests

```
cargo run --release -p azzy-harness --manifest-path tools/Cargo.toml
```

The harness runs the real Lua files against a simulated client. This build
keeps the scenarios for the base homunculi, the mercenaries, movement, kiting,
tactics and the element checks, and drops the Homunculus S ones.

The Renewal damage model (`tools/harness/damage.lua`) is not used here: the
scenarios check which skills are cast, not their expected damage. A Pre-Renewal
damage model would need its own implementation (no base-level term, classic
skill ratios) and is not part of this build yet.

## Known limitations

- **Pre-Renewal skill timings** are still the values in `H_SkillList.lua`
  (official client tables). They should be audited against
  `db/pre-re/skill_db.yml`; Pre-Renewal has no fixed cast time, and several
  reuse delays differ.
- **Mercenary data** is the same as the main build; servers that customized it
  need their own `M_SkillList.lua`.
- The harness fixture (`tools/harness/fixtures/rathena.lua`) is not generated
  for Pre-Renewal yet, so private-server scenarios use the AI's own data.

## Keeping up with the main build

This branch only receives what we write for it: AI fixes that are not about
Renewal (targeting, movement, tactics, cast handling, logging). Renewal-only
work stays on `main` and is not merged here. The `main` branch has no Pre-Renewal
code: it is the Renewal product.

## Current state of this branch

- The four original homunculi, the pre-re monster database and the classic
  element table: done.
- Homunculus S: no data, no options, no constants, no GUI entries. The branches
  that named its skill ids are unreachable (if false then) and are being
  deleted in small steps; none of them runs.
- The harness has no S scenario, no sphere/style simulation and no S benchmark;
  57 scenarios pass.
- The GUI is its own prerenewal branch of the webgui submodule: 105 options
  and 13 skills.
- Still open: the last unreachable branches, the Renewal damage fixture used by
  the private-server scenarios, and a pre-re audit of the skill timings.

## Version

The version number follows the AzzyAI it was forked from (2.1.1), so this is
**AzzyAI Pre-Renewal 2.2.0**: the 2.x line of *this* product, not an upgrade of
the Renewal AI. Releases are tagged `prere-vX.Y.Z` to keep the two apart; the
`main` branch keeps the plain `vX.Y.Z` tags.
