#!/usr/bin/env python3
"""Downloads the LATAM data of the homunculus skills AzzyAI uses from the Divine Pride API.

The result goes to tools/harness/fixtures/divinepride_<server>.json (raw API answers keyed by
skill ID), so the official values in USER_AI/H_SkillList.lua can be compared with what the LATAM
client actually uses. The file holds third-party data and is git-ignored; keep it local.

Usage (needs a free API key from your divine-pride.net profile page):
    set DIVINEPRIDE_API_KEY=<your key>
    python tools/harness/divinepride_sync.py               # LATAM
    python tools/harness/divinepride_sync.py --server kROM # another server
    python tools/harness/divinepride_sync.py --refresh     # fetch again even if cached

Following https://www.divine-pride.net/tools/api-doc: only the known homunculus skill IDs are
requested (no ID enumeration), answers already in the output file are reused, and 429 answers
are retried after Retry-After. The key is only read from the environment and sent to the API;
it is never printed or saved. Standard library only; behind a proxy, set HTTPS_PROXY first.
"""
import argparse
import datetime
import json
import os
import sys
import time
import urllib.error
import urllib.parse
import urllib.request

try:
    # optional: on Linux/macOS this makes Python trust the OS certificate store as well
    import truststore
    truststore.inject_into_ssl()
except ImportError:
    pass

HERE = os.path.dirname(os.path.abspath(__file__))
API = "https://www.divine-pride.net/api/database/Skill/{id}"

# homunculus skills defined in USER_AI/H_SkillList.lua (passives included, for the descriptions)
SKILL_IDS = [
    8001, 8002, 8003, 8004,             # Lif: Healing Hands, Urgent Escape, Brain Surgery, Mental Change
    8005, 8006, 8007, 8008,             # Amistr: Castling, Bulwark, Adamantium Skin, Bloodlust
    8009, 8010, 8011, 8012,             # Filir: Moonlight, Flitting, Accelerated Flight, S.B.R.44
    8013, 8014, 8015, 8016,             # Vanilmirth: Caprice, Chaotic Blessings, Change Instruction, Bio Explosion
    8018, 8019, 8020, 8021, 8053, 8054,  # Sera
    8022, 8023, 8024, 8025, 8026, 8047, 8048,  # Eira
    8027, 8028, 8029, 8030, 8036, 8037, 8038, 8050, 8051,  # Eleanor
    8031, 8032, 8033, 8034, 8035, 8056, 8057, 8058,  # Bayeri
    8039, 8040, 8041, 8042, 8043, 8044, 8045,  # Dieter
]


def fetch(skill_id, key, server, language, retries=5):
    url = API.format(id=skill_id) + "?" + urllib.parse.urlencode({"apiKey": key})
    req = urllib.request.Request(url, headers={
        "User-Agent": "AzzyAI-harness", "Accept": "application/json",
        "Accept-Language": language, "x-server": server})
    for _ in range(retries):
        try:
            with urllib.request.urlopen(req, timeout=60) as r:
                return json.loads(r.read().decode("utf-8"))
        except urllib.error.HTTPError as e:
            if e.code != 429:
                raise
            wait = int(e.headers.get("Retry-After") or 30)
            print(f"  rate limited, waiting {wait} s")
            time.sleep(wait)
    raise RuntimeError("still rate limited, try again later")


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--server", default="LATAM", help="Divine Pride server code (default: LATAM)")
    parser.add_argument("--language", default="en", help="Accept-Language for names/descriptions")
    parser.add_argument("--delay", type=float, default=1.0, help="seconds between requests")
    parser.add_argument("--refresh", action="store_true", help="ignore answers already saved")
    args = parser.parse_args()

    key = os.environ.get("DIVINEPRIDE_API_KEY", "").strip()
    if not key:
        sys.exit("DIVINEPRIDE_API_KEY is not set (get it from your divine-pride.net profile page)")

    out = os.path.join(HERE, "fixtures", f"divinepride_{args.server.lower()}.json")
    skills = {}
    if os.path.exists(out) and not args.refresh:
        with open(out, encoding="utf-8") as f:
            skills = json.load(f).get("skills", {})

    missing = []
    for sid in SKILL_IDS:
        if str(sid) in skills:
            continue
        try:
            data = fetch(sid, key, args.server, args.language)
        except urllib.error.HTTPError as e:
            # report only the status: the request URL contains the key
            if e.code == 401:
                sys.exit("HTTP 401: the API key was refused")
            if e.code == 404:
                missing.append(sid)
                continue
            sys.exit(f"HTTP {e.code} for skill {sid}")
        except urllib.error.URLError as e:
            sys.exit(f"network error: {e.reason}")
        skills[str(sid)] = data
        region = data.get("region", "?") if isinstance(data, dict) else "?"
        name = data.get("name", "?") if isinstance(data, dict) else "?"
        print(f"{sid} {name} ({region})")
        if region not in ("?", args.server):
            print(f"  warning: asked for {args.server}, got {region}")
        time.sleep(args.delay)

    with open(out, "w", encoding="utf-8", newline="\n") as f:
        json.dump({"server": args.server, "date": str(datetime.date.today()), "skills": skills},
                  f, ensure_ascii=False, indent=1, sort_keys=True)
    print(f"wrote {out}: {len(skills)} skills, not found: {missing or 'none'}")


if __name__ == "__main__":
    main()
