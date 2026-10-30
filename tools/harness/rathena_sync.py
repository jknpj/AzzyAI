#!/usr/bin/env python3
"""Regenerates tools/harness/fixtures/rathena.lua from the rAthena Renewal databases.

The harness uses the generated file as the "server truth" for SERVER_PRIVATE scenarios
(SP, cooldowns, ranges, spirit spheres), to check the homunculus skill trees and to give
scenario monsters real rAthena data. The file is committed, so the harness and CI never
need network access; run this script only to move to a newer rAthena commit.

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
REPO = "rathena/rathena"

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
