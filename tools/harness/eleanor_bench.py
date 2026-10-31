#!/usr/bin/env python3
"""Runs the harness' Eleanor benchmark scenarios and prints a comparison table.

The scenarios (eleanor_bench_* in scenarios.lua) mirror the test server's passive benchmark:
a Neutral zero-defense dummy, 180 simulated seconds, the level 275 Eleanor of
fixtures/homun_stats.lua and the damage model of damage.lua. The simulation is deterministic
(expected values, no random rolls), so one run per configuration is all there is.

    python tools/harness/eleanor_bench.py                    # all configurations
    python tools/harness/eleanor_bench.py --attack-delay 200 # normal-attack cadence in ms
    python tools/harness/eleanor_bench.py sonic_only fighting_none

Needs Lua 5.1 like run.py (LUA environment variable or PATH).
"""
import argparse
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CONFIGS = ["attack_only", "sonic_only", "fighting_none", "fighting_blazing", "fighting_fighter", "fighting_both",
           "grappling_none", "grappling_blazing", "grappling_fighter", "grappling_both"]
LABELS = {
    "attack_only": "no skills: normal attacks only",
    "sonic_only": "fighting: Sonic Claw only",
    "fighting_none": "fighting: combo (Sonic>Silvervein>Midnight)",
    "fighting_blazing": "fighting: combo + Blazing And Furious",
    "fighting_fighter": "fighting: combo + The One Fighter Rises",
    "fighting_both": "fighting: combo + both",
    "grappling_none": "grappling: combo (Tinder>CBC>EQC)",
    "grappling_blazing": "grappling: combo + Blazing And Furious",
    "grappling_fighter": "grappling: combo + The One Fighter Rises",
    "grappling_both": "grappling: combo + both",
}


def run(name, attack_delay):
    env = dict(os.environ, HARNESS_DAMAGE="1")
    if attack_delay:
        env["ELEANOR_ATTACK_DELAY"] = str(attack_delay)
    r = subprocess.run([sys.executable, os.path.join(HERE, "run.py"), "--skip-syntax", "eleanor_bench_" + name],
                       capture_output=True, text=True, env=env, timeout=600)
    out = r.stdout
    head = re.search(r"eleanor_bench_\S+\s+(PASS|FAIL)", out)
    if not head:
        sys.exit(f"{name}: no result\n{out}{r.stderr}")
    total = re.search(r"damage: ([\d,]+) in ([\d.]+) s = ([\d,]+) DPS", out)
    casts = re.search(r"casts: (\d+) accepted, (\d+) refused by the server, (\d+) SP spent", out)
    skills = {m.group(1): (int(m.group(2)), int(m.group(3).replace(",", "")))
              for m in re.finditer(r"^\s+(MH_\w+|Normal attack)\s+x(\d+)\s+([\d,]+)\s", out, re.M)}
    if not total or not casts:
        sys.exit(f"{name}: no damage report\n{out}")
    return {"status": head.group(1), "damage": int(total.group(1).replace(",", "")),
            "seconds": float(total.group(2)), "dps": int(total.group(3).replace(",", "")),
            "accepted": int(casts.group(1)), "refused": int(casts.group(2)), "sp": int(casts.group(3)),
            "skills": skills}


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("configs", nargs="*", help="configurations to run (default: all)")
    parser.add_argument("--attack-delay", type=int, help="ms between normal attacks (scenario default: 300)")
    args = parser.parse_args()
    names = args.configs or CONFIGS
    results = {n: run(n, args.attack_delay) for n in names}
    base = results.get("sonic_only")
    print(f"{'configuration':44} {'DPS':>9} {'vs Sonic':>9} {'dmg/SP':>7} {'refused':>8}  skills (casts)")
    for n in names:
        r = results[n]
        per_sp = f"{r['damage'] / r['sp']:.0f}" if r["sp"] else "n/a"
        skills = ", ".join(f"{k.replace('MH_', '').replace('CRAW', 'CLAW').title().replace('_', ' ')} {v[0]}"
                           for k, v in sorted(r["skills"].items(), key=lambda kv: -kv[1][1]) if k != "Normal attack")
        vs = f"{100 * (r['dps'] / base['dps'] - 1):+.0f}%" if base and n != "sonic_only" else ""
        print(f"{LABELS[n]:44} {r['dps']:>9,} {vs:>9} {per_sp:>7} {r['refused']:>8}  {skills}")
    print("expected values at the pinned rAthena commit; the model ignores status effects and buffs")


if __name__ == "__main__":
    main()
