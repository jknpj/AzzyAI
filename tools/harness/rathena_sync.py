#!/usr/bin/env python3
"""Regenerates tools/harness/fixtures/rathena.lua and USER_AI/MobInfo.lua from the rAthena Renewal databases.

The harness uses the generated file as the "server truth" for SERVER_PRIVATE scenarios
(SP, cooldowns, ranges, spirit spheres), to check the homunculus skill trees and to give
scenario monsters real rAthena data. MobInfo.lua is loaded by the AI itself: every monster's
race, element, size and boss/MVP/plant flags, the element damage table and the element of each
homunculus attack skill. The web GUI's webgui/mobdata.js (monsters and the maps they spawn on,
for its tactics suggestions) is written when the webgui submodule is checked out. All files are
committed, so the AI, the harness and CI never need network access; run this script only to move
to a newer rAthena commit.

Usage:
    python tools/harness/rathena_sync.py              # latest rathena/rathena master
    python tools/harness/rathena_sync.py --ref <sha>  # a specific commit/branch/tag

Only needs the Python standard library. Behind a proxy, set HTTPS_PROXY (urllib also reads the
Windows proxy settings). On Windows the system certificate store is trusted, so proxies that
inspect TLS with a corporate CA work too.
"""
import argparse
import datetime
import json
import os
import re
import sys
import urllib.request

try:
    # optional: on Linux/macOS this makes Python trust the OS certificate store as well
    import truststore
    truststore.inject_into_ssl()
except ImportError:
    pass

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "fixtures", "rathena.lua")
MOBINFO_OUT = os.path.join(HERE, "..", "..", "USER_AI", "MobInfo.lua")
REPO = "rathena/rathena"

# rAthena names -> the numbers MobInfo.lua uses (also the client's element/race/size numbering)
ELEMENTS = ["Neutral", "Water", "Earth", "Fire", "Wind", "Poison", "Holy", "Dark", "Ghost", "Undead"]
RACES = ["Formless", "Undead", "Brute", "Plant", "Insect", "Fish", "Demon", "Demihuman", "Angel", "Dragon",
         "Player_Human", "Player_Doram"]
SIZES = ["Small", "Medium", "Large"]
# flags in MobInfo.lua; must match MOB_FLAG_* in USER_AI/AzzyUtil.lua
FLAG_BOSS, FLAG_MVP, FLAG_PLANT, FLAG_IMMOBILE = 1, 2, 4, 8
PLANT_MODES = ("IgnoreMelee", "IgnoreMagic", "IgnoreRanged", "IgnoreMisc")
# webgui/mobdata.js
WEBGUI = os.path.join(HERE, "..", "..", "webgui")
WG_BOSS, WG_MVP, WG_PLANT, WG_AGGRESSIVE, WG_IMMOBILE = 1, 2, 4, 8, 16
# rAthena mob_db Ai codes (doc/mob_db_mode_list.txt), same sets as harness.lua
AGGRESSIVE_AI = {"04", "05", "08", "09", "10", "11", "12", "13", "19", "20", "21", "26", "27"}
IMMOBILE_AI = {"06", "10", "11"}

# homunculus skill IDs (8001-8099); mercenary skills start at 8201
SKILL_IDS = range(8001, 8100)

# monsters available to scenarios through { x, y, mobId }
MOB_IDS = [
    1002,  # Poring: passive
    1007,  # Fabre: passive
    1015,  # Zombie: aggressive, undead
    1031,  # Poporing
    1039,  # Baphomet: MVP
    1068,  # Hydra: immobile, aggressive
    1076,  # Skeleton: aggressive, undead
    1078, 1079, 1080, 1081, 1082, 1083,  # Red/Blue/Green/Yellow/White/Shining Plant
    1084, 1085,  # Black/Red Mushroom
    1101,  # Baphomet Jr.: aggressive
    1147,  # Maya: MVP
    1368,  # Geographer: immobile, aggressive
    1672, 1673, 1678,  # Dimik/Venatu: Earth 2, Fire 2, Earth 2 (mixed-element packs)
]

# homunculus classes in homunculus_db.yml, keyed by the AzzyAI homunculus type constant
HOMUN_CLASSES = {
    "LIF": "Lif", "AMISTR": "Amistr", "FILIR": "Filir", "VANILMIRTH": "Vanilmirth",
    "EIRA": "Eira", "BAYERI": "Bayeri", "SERA": "Sera", "DIETER": "Dieter", "ELEANOR": "Eleanor",
}


def fetch(url):
    req = urllib.request.Request(url, headers={"User-Agent": "AzzyAI-harness"})
    with urllib.request.urlopen(req, timeout=120) as r:
        return r.read().decode("utf-8")


def resolve_ref(ref):
    data = json.loads(fetch(f"https://api.github.com/repos/{REPO}/commits/{ref}"))
    return data["sha"]


_INT = re.compile(r"-?(0|[1-9][0-9]*)$")
_KEY = re.compile(r"([^\s:#\"'][^:]*?):(?:\s+(.*))?$")


def _scalar(v):
    if v[:1] == '"':
        end = v.rfind('"')
        return v[1:end].replace('\\"', '"').replace("\\\\", "\\")
    if v[:1] == "'":
        end = v.rfind("'")
        return v[1:end].replace("''", "'")
    v = re.sub(r"\s+#.*$", "", v)  # trailing comment
    if v in ("true", "false"):
        return v == "true"
    if v in ("null", "~", ""):
        return None
    if _INT.match(v):
        return int(v)
    return v  # also keeps codes with leading zeros, such as Ai: 04, as strings


def parse_yaml(text):
    """Block-style YAML as used by rAthena's databases: nested mappings and sequences,
    plain/quoted scalars and comments (no anchors, flow collections or multi-line strings)."""
    lines = []
    for raw in text.splitlines():
        body = raw.rstrip()
        stripped = body.lstrip(" ")
        if stripped and not stripped.startswith("#") and stripped != "---":
            lines.append([len(body) - len(stripped), stripped])
    pos = 0

    def block(indent):
        # a key's value may also be a sequence at the key's own indentation
        if lines[pos][1][:1] == "-":
            return seq(lines[pos][0])
        return mapping(lines[pos][0])

    def mapping(indent):
        nonlocal pos
        out = {}
        while pos < len(lines):
            ind, txt = lines[pos]
            if ind < indent or (ind == indent and txt[:1] == "-"):
                break
            m = _KEY.match(txt)
            if ind > indent:
                pos += 1  # continuation of a multi-line scalar; not needed here
                continue
            if not m:
                raise ValueError(f"unexpected YAML line: {txt!r}")
            pos += 1
            key, value = m.group(1).strip(), m.group(2)
            if value is not None and value.strip():
                out[key] = _scalar(value.strip())
            elif pos < len(lines) and (lines[pos][0] > ind or (lines[pos][0] == ind and lines[pos][1][:1] == "-")):
                out[key] = block(ind)
            else:
                out[key] = None
        return out

    def seq(indent):
        nonlocal pos
        out = []
        while pos < len(lines):
            ind, txt = lines[pos]
            if ind != indent or txt[:1] != "-":
                break
            rest = txt[1:].lstrip(" ")
            if not rest:
                pos += 1
                out.append(block(ind + 1) if pos < len(lines) and lines[pos][0] > ind else None)
            elif _KEY.match(rest) and rest[:1] not in "\"'":
                # "- Key: value" starts a mapping whose keys line up after the dash
                lines[pos] = [ind + len(txt) - len(rest), rest]
                out.append(mapping(lines[pos][0]))
            else:
                pos += 1
                out.append(_scalar(rest))
        return out

    return block(0)


def load_db(sha, path):
    text = fetch(f"https://raw.githubusercontent.com/{REPO}/{sha}/{path}")
    return parse_yaml(text)["Body"]


def per_level(value, maxlv, key):
    """rAthena fields are either one value for every level or a list of {Level, <key>}."""
    if value is None:
        return [0] * maxlv
    if not isinstance(value, list):
        return [int(value)] * maxlv
    by_level = {int(e["Level"]): int(e[key]) for e in value}
    out, last = [], 0
    for lv in range(1, maxlv + 1):
        last = by_level.get(lv, last)
        out.append(last)
    return out


def ai_code(v):
    # leading-zero codes (Ai: 04) stay strings; others (Ai: 10) are parsed as ints
    if v is None:
        return "06"
    return f"{v:02d}" if isinstance(v, int) else str(v).zfill(2)


def lua(v, indent=""):
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, (int, float)):
        return str(v)
    if isinstance(v, str):
        return '"' + v.replace("\\", "\\\\").replace('"', '\\"') + '"'
    if isinstance(v, list):
        return "{" + ",".join(lua(x) for x in v) + "}"
    if isinstance(v, dict):
        parts = []
        for k, x in v.items():
            key = f"[{k}]" if isinstance(k, int) else k
            parts.append(f"{key}={lua(x)}")
        return "{" + ", ".join(parts) + "}"
    raise TypeError(type(v))


def build_skills(body):
    skills = {}
    for s in body:
        sid = int(s["Id"])
        if sid not in SKILL_IDS:
            continue
        n = int(s.get("MaxLevel", 1))
        req = s.get("Requires") or {}
        unit = s.get("Unit") or {}
        skills[sid] = {
            "name": s["Name"],
            "maxlv": n,
            "target": s.get("TargetType", "Passive"),
            "range": per_level(s.get("Range"), n, "Size"),
            "splash": per_level(s.get("SplashArea"), n, "Area"),
            "layout": per_level(unit.get("Layout"), n, "Size"),
            "sp": per_level(req.get("SpCost"), n, "Amount"),
            "hp": per_level(req.get("HpCost"), n, "Amount"),
            "spheres": per_level(req.get("SpiritSphereCost"), n, "Amount"),
            "cast": per_level(s.get("CastTime"), n, "Time"),
            "fixed": per_level(s.get("FixedCastTime"), n, "Time"),
            "delay": per_level(s.get("AfterCastActDelay"), n, "Time"),
            "cooldown": per_level(s.get("Cooldown"), n, "Time"),
            "duration": per_level(s.get("Duration1"), n, "Time"),
        }
    return skills


def build_homun(body, name_to_id):
    trees = {}
    by_class = {h["Class"]: h for h in body}
    for const, cls in HOMUN_CLASSES.items():
        h = by_class.get(cls)
        if h is None:
            sys.exit(f"homunculus class {cls} not found in homunculus_db.yml")
        tree = {}
        for e in h.get("SkillTree") or []:
            sid = name_to_id.get(e["Skill"])
            if sid is not None:
                tree[sid] = {"maxlv": int(e.get("MaxLevel", 1)), "level": int(e.get("RequiredLevel", 0)),
                             "intimacy": int(e.get("RequiredIntimacy", 0))}
        trees[const] = tree
    return trees


def build_mobs(body):
    mobs = {}
    for m in body:
        mid = int(m["Id"])
        if mid not in MOB_IDS:
            continue
        mobs[mid] = {
            "name": m.get("Name", m["AegisName"]),
            "level": int(m.get("Level", 1)),
            "hp": int(m.get("Hp", 1)),
            "race": m.get("Race", "Formless"),
            "element": m.get("Element", "Neutral"),
            "elv": int(m.get("ElementLevel", 1)),
            "size": m.get("Size", "Small"),
            "ai": ai_code(m.get("Ai")),
            "class": m.get("Class", "Normal"),
            "mvp": bool(m.get("MvpExp")),
        }
    missing = sorted(set(MOB_IDS) - set(mobs))
    if missing:
        sys.exit(f"monsters not found in mob_db.yml: {missing}")
    return mobs


def index_of(table, name, what, default=0):
    if name is None:
        return default
    if name not in table:
        print(f"  warning: unknown {what} {name!r}, using {table[default]}")
        return default
    return table.index(name)


def write_mobinfo(sha, mob_body, skill_body, attr_body):
    """USER_AI/MobInfo.lua: the monster facts the AI uses to pick skills and treat bosses/plants."""
    lines = [
        "-- MobInfo.lua: generated by tools/harness/rathena_sync.py from rAthena Renewal; do not edit by hand.",
        f"-- Source: https://github.com/{REPO}/tree/{sha} (db/re/mob_db.yml, attr_fix.yml, skill_db.yml),",
        f"-- generated {datetime.date.today()}. Used by GetMobInfo() and friends in AzzyUtil.lua.",
        "-- MobInfo[class] = {race, element, element level, size, flags, attack range}",
        "--   element: " + ", ".join(f"{i} {n}" for i, n in enumerate(ELEMENTS)),
        "--   race: " + ", ".join(f"{i} {n}" for i, n in enumerate(RACES)),
        "--   size: " + ", ".join(f"{i} {n}" for i, n in enumerate(SIZES)),
        f"--   flags (sum): {FLAG_BOSS} boss protocol, {FLAG_MVP} MVP, {FLAG_PLANT} plant (1 damage from everything),",
        f"--                {FLAG_IMMOBILE} immobile (never chases: Ai 06, 10 or 11)",
        "--   attack range: cells (mob_db AttackRange, at least 1)",
        "MobInfo = {}",
    ]
    count = 0
    for m in sorted(mob_body, key=lambda m: int(m["Id"])):
        modes = m.get("Modes") or {}
        flags = 0
        if m.get("Class") == "Boss":
            flags += FLAG_BOSS
        if m.get("MvpExp"):
            flags += FLAG_MVP
        if all(modes.get(k) for k in PLANT_MODES):
            flags += FLAG_PLANT
        if ai_code(m.get("Ai")) in IMMOBILE_AI:
            flags += FLAG_IMMOBILE
        info = [index_of(RACES, m.get("Race"), "race"), index_of(ELEMENTS, m.get("Element"), "element"),
                int(m.get("ElementLevel", 1)), index_of(SIZES, m.get("Size"), "size"), flags,
                max(1, int(m.get("AttackRange", 0)))]
        lines.append(f"MobInfo[{int(m['Id'])}]={{{','.join(str(x) for x in info)}}}")
        count += 1

    lines += ["", "-- ElementFix[element level][attack element][defense element] = damage %", "ElementFix = {}"]
    for entry in attr_body:
        lv = int(entry["Level"])
        lines.append(f"ElementFix[{lv}] = {{}}")
        for atk in ELEMENTS:
            row = entry.get(atk) or {}
            values = [int(row.get(d, 100)) for d in ELEMENTS]
            lines.append(f"ElementFix[{lv}][{ELEMENTS.index(atk)}] = {{[0]={values[0]},{','.join(str(v) for v in values[1:])}}}")

    lines += ["", "-- SkillElement[skill] = element of the homunculus skills that deal damage",
              "-- (weapon-element skills use the homunculus' neutral attack)", "SkillElement = {}"]
    for s in skill_body:
        sid = int(s["Id"])
        flags = s.get("DamageFlags") or {}
        # NoDamage skills don't hurt anything, except splash ones like Blazing and Furious, whose
        # damage comes from the splash part
        if sid not in SKILL_IDS or s.get("Type") not in ("Weapon", "Magic", "Misc") or \
                (flags.get("NoDamage") and not flags.get("Splash")):
            continue
        element = s.get("Element")
        if isinstance(element, list):  # per-level element: use the first one
            element = element[0].get("Element")
        idx = ELEMENTS.index(element) if element in ELEMENTS else 0
        lines.append(f"SkillElement[{sid}] = {idx} -- {s['Name']} ({element or 'Weapon'})")

    with open(MOBINFO_OUT, "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(lines) + "\n")
    print(f"wrote {os.path.normpath(MOBINFO_OUT)}: {count} monsters")


def write_webgui_mobdata(sha, mob_body):
    """webgui/mobdata.js: monsters and the maps they spawn on, for the web GUI's tactics suggestions."""
    if not os.path.isdir(WEBGUI):
        print(f"  {os.path.normpath(WEBGUI)} not found, mobdata.js skipped")
        return
    aegis = {m["AegisName"]: int(m["Id"]) for m in mob_body if m.get("AegisName")}
    mobs = {}
    for m in mob_body:
        modes = m.get("Modes") or {}
        ai = ai_code(m.get("Ai"))
        flags = 0
        if m.get("Class") == "Boss":
            flags |= WG_BOSS
        if m.get("MvpExp"):
            flags |= WG_MVP
        if all(modes.get(k) for k in PLANT_MODES):
            flags |= WG_PLANT
        if ai in AGGRESSIVE_AI:
            flags |= WG_AGGRESSIVE
        if ai in IMMOBILE_AI:
            flags |= WG_IMMOBILE
        mobs[int(m["Id"])] = [m.get("Name", m["AegisName"]), int(m.get("Level", 1)), flags,
                              index_of(ELEMENTS, m.get("Element"), "element"), int(m.get("ElementLevel", 1)),
                              index_of(RACES, m.get("Race"), "race")]

    # npc/re/mobs/**.txt: "map,x,y[,xs,ys]<TAB>monster|boss_monster|miniboss_monster<TAB>Name<TAB>id,amount,..."
    spawn = re.compile(r"^([^,\s/]+),[^\t]*\t(?:boss_|miniboss_)?monster\t[^\t]*\t([A-Za-z0-9_]+)\s*,", re.M)
    maps = {}
    files = list_spawn_files(sha, "npc/re/mobs")
    for path in files:
        for map_name, mob in spawn.findall(fetch(f"https://raw.githubusercontent.com/{REPO}/{sha}/{path}")):
            mid = int(mob) if mob.isdigit() else aegis.get(mob)
            if mid in mobs:
                ids = maps.setdefault(map_name, [])
                if mid not in ids:
                    ids.append(mid)

    out = os.path.join(WEBGUI, "mobdata.js")
    lines = [
        "// mobdata.js: generated by tools/harness/rathena_sync.py (AzzyAI repository); do not edit by hand.",
        f"// Source: https://github.com/{REPO}/tree/{sha} (db/re/mob_db.yml, npc/re/mobs), generated {datetime.date.today()}.",
        "// mobs[id] = [name, level, flags, element, element level, race]",
        f"//   flags (sum): {WG_BOSS} boss protocol, {WG_MVP} MVP, {WG_PLANT} plant (1 damage from everything),",
        f"//   {WG_AGGRESSIVE} aggressive, {WG_IMMOBILE} can't move. Element/race numbers as in USER_AI/MobInfo.lua.",
        "// maps[map] = monster IDs that spawn there (rAthena Renewal spawn scripts)",
        "window.AzzyMobData = {",
        "  elements: " + json.dumps(ELEMENTS) + ",",
        "  races: " + json.dumps(RACES) + ",",
        "  mobs: {",
    ]
    lines += [f"    {mid}: {json.dumps(mobs[mid], ensure_ascii=False)}," for mid in sorted(mobs)]
    lines += ["  },", "  maps: {"]
    lines += [f"    {json.dumps(name)}: {json.dumps(maps[name])}," for name in sorted(maps)]
    lines += ["  }", "};", ""]
    with open(out, "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(lines))
    print(f"wrote {os.path.normpath(out)}: {len(mobs)} monsters, {len(maps)} maps from {len(files)} spawn files")


def list_spawn_files(sha, path):
    """All .txt files under a repository folder (GitHub contents API, a few requests)."""
    items = json.loads(fetch(f"https://api.github.com/repos/{REPO}/contents/{path}?ref={sha}"))
    files = []
    for item in items:
        if item["type"] == "dir":
            files += list_spawn_files(sha, item["path"])
        elif item["name"].endswith(".txt"):
            files.append(item["path"])
    return files


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--ref", default="master", help="rAthena commit, branch or tag (default: master)")
    args = parser.parse_args()

    sha = resolve_ref(args.ref)
    print(f"rAthena {sha}")
    skill_body = load_db(sha, "db/re/skill_db.yml")
    name_to_id = {s["Name"]: int(s["Id"]) for s in skill_body}
    skills = build_skills(skill_body)
    homun = build_homun(load_db(sha, "db/re/homunculus_db.yml"), name_to_id)
    mobs = []
    for path in ("db/re/mob_db.yml", "db/mob_db2.yml"):
        try:
            mobs += load_db(sha, path) or []
        except Exception:
            pass
    write_mobinfo(sha, mobs, skill_body, load_db(sha, "db/re/attr_fix.yml"))
    write_webgui_mobdata(sha, mobs)
    mobs = build_mobs(mobs)

    lines = [
        "-- Generated by tools/harness/rathena_sync.py; do not edit by hand.",
        f"-- Source: https://github.com/{REPO}/tree/{sha} (db/re), generated {datetime.date.today()}.",
        "-- Per-level lists are indexed by skill level; times in ms.",
        f'RATHENA_COMMIT = "{sha}"',
        "",
        "-- RA_SKILL[id]: homunculus skills from db/re/skill_db.yml",
        "RA_SKILL = {",
    ]
    lines += [f"\t[{sid}] = {lua(skills[sid])}," for sid in sorted(skills)]
    lines += ["}", "", "-- RA_HOMUN[homuntype constant][skill id]: skill tree from db/re/homunculus_db.yml", "RA_HOMUN = {"]
    for const in HOMUN_CLASSES:
        tree = homun[const]
        lines.append(f"\t{const} = {{")
        lines += [f"\t\t[{sid}] = {lua(tree[sid])}," for sid in sorted(tree)]
        lines.append("\t},")
    lines += ["}", "", "-- RA_MOB[id]: monsters from db/re/mob_db.yml", "RA_MOB = {"]
    lines += [f"\t[{mid}] = {lua(mobs[mid])}," for mid in sorted(mobs)]
    lines += ["}", ""]

    with open(OUT, "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(lines))
    print(f"wrote {OUT}: {len(skills)} skills, {len(homun)} homunculi, {len(mobs)} monsters")


if __name__ == "__main__":
    main()
