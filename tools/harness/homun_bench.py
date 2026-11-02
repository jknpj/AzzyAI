#!/usr/bin/env python3
"""Compares configurations of one Homunculus S in the harness (expected damage, see damage.lua).

Each configuration is a set of H_Config.lua overrides applied on top of the shipped USER_AI/H_Config.lua
(--config picks another base).
Runs the hbench_<homunculus> scenario (180 simulated seconds against a zero-defense Neutral dummy,
or a pack of five) and prints DPS, SP spent and the casts of every configuration.

    python tools/harness/homun_bench.py eira "AutoMobCount=1" "AutoMobCount=2"
    python tools/harness/homun_bench.py eira --mobs 5 --spregen 40 "AutoMobCount=2,UseEiraTwisterCutter=0"

An empty string or the word "base" is the unchanged fixture configuration. Needs Lua 5.1 like run.py.
"""
import argparse
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))


def run(homun, overrides, mobs, spregen, seconds, config, mob_def=None, mob_mdef=None):
    env = dict(os.environ, HARNESS_DAMAGE="1", HBENCH_SET=overrides, HBENCH_MOBS=str(mobs),
               HBENCH_SECONDS=str(seconds))
    if mob_def is not None:
        env["HBENCH_DEF"] = str(mob_def)
    if mob_mdef is not None:
        env["HBENCH_MDEF"] = str(mob_mdef)
    if spregen:
        env["HBENCH_SPREGEN"] = str(spregen)
    if config:
        env["HARNESS_CONFIG"] = config
    r = subprocess.run([sys.executable, os.path.join(HERE, "run.py"), "--skip-syntax", "hbench_" + homun],
                       capture_output=True, text=True, env=env, timeout=900)
    out = r.stdout
    total = re.search(r"damage: ([\d,]+) in ([\d.]+) s = ([\d,]+) DPS", out)
    casts = re.search(r"casts: (\d+) accepted, (\d+) refused by the server, (\d+) SP spent", out)
    if not total or not casts:
        sys.exit(f"no damage report for {overrides!r}\n{out}{r.stderr}")
    skills = [(m.group(1), int(m.group(2))) for m in
              re.finditer(r"^\s+(MH_\w+|Normal attack)\s+x(\d+)\s", out, re.M) if m.group(1) != "Normal attack"]
    return {"dps": int(total.group(3).replace(",", "")), "damage": int(total.group(1).replace(",", "")),
            "sp": int(casts.group(3)), "refused": int(casts.group(2)), "skills": skills}


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("homunculus", choices=["eira", "bayeri", "sera", "dieter", "eleanor"])
    parser.add_argument("configs", nargs="+", help='overrides like "AutoMobCount=2,UseEiraTwisterCutter=0"')
    parser.add_argument("--mobs", type=int, default=1, help="monsters in the pack, 1 to 5 (default 1)")
    parser.add_argument("--spregen", type=int, default=0,
                        help="SP regeneration per second (default: 60, Eleanor 22)")
    parser.add_argument("--seconds", type=int, default=180)
    parser.add_argument("--def", dest="mob_def", type=int, help="hard DEF of the dummies (default 0)")
    parser.add_argument("--mdef", dest="mob_mdef", type=int, help="MDEF of the dummies (default 0)")
    parser.add_argument("--config", default=os.path.join(HERE, "..", "..", "USER_AI", "H_Config.lua"),
                        help="base H_Config.lua (default: the shipped USER_AI preset)")
    args = parser.parse_args()
    config = os.path.abspath(args.config)
    results = [(c, run(args.homunculus, "" if c == "base" else c, args.mobs, args.spregen, args.seconds, config,
                       args.mob_def, args.mob_mdef))
               for c in args.configs]
    base = results[0][1]["dps"]
    print(f"{args.homunculus}: {args.mobs} monster(s), {args.seconds} s, "
          f"{args.spregen or 'default'} SP/s regeneration")
    print(f"{'DPS':>10} {'vs first':>9} {'dmg/SP':>7} {'refused':>8}  configuration -> casts")
    for config, r in results:
        per_sp = f"{r['damage'] / r['sp']:.0f}" if r["sp"] else "n/a"
        casts = ", ".join(f"{k.replace('MH_', '').title().replace('_', ' ')} {n}" for k, n in
                          sorted(r["skills"], key=lambda kv: -kv[1]))
        print(f"{r['dps']:>10,} {100 * (r['dps'] / base - 1):>+8.0f}% {per_sp:>7} {r['refused']:>8}  "
              f"{config or '(fixture)'} -> {casts}")


if __name__ == "__main__":
    main()
