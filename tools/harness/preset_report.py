#!/usr/bin/env python3
"""Regenerates the preset comparison spreadsheets in docs/preset-tuning/ from the offline harness.

    python tools/harness/preset_report.py                 # old preset = USER_AI/H_Config.lua at HEAD
    python tools/harness/preset_report.py --old-ref <sha> # the preset before the tuning commit

Writes (comma separated, UTF-8 with BOM, opens in Excel/LibreOffice and renders on GitHub):
  docs/preset-tuning/preset-before-after.csv   previous shipped preset against the current file, per homunculus and pack size
  docs/preset-tuning/preset-options.csv        the current preset with one option changed at a time
  docs/preset-tuning/README.md                 the same data as Markdown tables, with the method and its limits

Every row is an expected-value simulation (damage.lua): a zero-defense Neutral dummy for 180 s, level 275
homunculi of fixtures/homun_stats.lua, rAthena data of the pinned commit. It is not a server measurement.
Needs Lua 5.1 like run.py. The runs are independent and executed in parallel.
"""
import argparse
import csv
import os
import subprocess
import sys
import tempfile
from concurrent.futures import ThreadPoolExecutor

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", ".."))
sys.path.insert(0, HERE)
import homun_bench  # noqa: E402

PACKS = [1, 2, 3, 5]
HOMUNCULI = ["eira", "bayeri", "sera", "dieter", "eleanor"]
DEFAULT_SPREGEN = {"eleanor": 22}  # the others use the scenario default of 60 SP/s

# (homunculus, monsters, SP/s or 0 for the default, label, overrides applied to the current preset)
OPTIONS = [
    ("eira", 1, 0, "current preset", ""),
    ("eira", 1, 0, "AutoMobCount 1", "AutoMobCount=1"),
    ("eira", 1, 0, "Twister Cutter on", "UseEiraTwisterCutter=1"),
    ("eira", 1, 0, "Twister Cutter on, AutoMobCount 1", "UseEiraTwisterCutter=1,AutoMobCount=1"),
    ("eira", 1, 30, "current preset", ""),
    ("eira", 1, 30, "Twister Cutter on", "UseEiraTwisterCutter=1"),
    ("eira", 3, 0, "current preset", ""),
    ("eira", 3, 0, "AutoMobCount 3", "AutoMobCount=3"),
    ("bayeri", 1, 0, "current preset", ""),
    ("bayeri", 1, 0, "AutoMobCount 1", "AutoMobCount=1"),
    ("bayeri", 1, 0, "Glanzen Spies on", "UseBayeriGlanzenSpies=1"),
    ("bayeri", 2, 80, "current preset", ""),
    ("bayeri", 2, 80, "AutoMobCount 3", "AutoMobCount=3"),
    ("bayeri", 3, 80, "current preset", ""),
    ("bayeri", 3, 80, "AutoMobCount 3", "AutoMobCount=3"),
    ("bayeri", 3, 80, "AutoMobCount 4", "AutoMobCount=4"),
    ("sera", 1, 0, "current preset", ""),
    ("sera", 1, 0, "AutoMobCount 1", "AutoMobCount=1"),
    ("sera", 1, 0, "Needle Stinger on", "UseSeraNeedleStinger=1"),
    ("sera", 3, 0, "current preset", ""),
    ("sera", 3, 0, "Needle Stinger on", "UseSeraNeedleStinger=1"),
    ("sera", 3, 0, "Toxin of Mandara off", "UseSeraToxinOfMandara=0"),
    ("dieter", 1, 0, "current preset", ""),
    ("dieter", 1, 0, "AutoMobCount 1", "AutoMobCount=1"),
    ("dieter", 1, 0, "AutoMobCount 1, Blast Forge off (previous preset)", "AutoMobCount=1,UseDieterBlastForge=0"),
    ("dieter", 2, 0, "current preset", ""),
    ("dieter", 2, 0, "Blast Forge off", "UseDieterBlastForge=0"),
    ("dieter", 2, 0, "AutoMobCount 1", "AutoMobCount=1"),
    ("dieter", 2, 0, "AutoMobCount 3", "AutoMobCount=3"),
    ("dieter", 2, 0, "Lava Slide off", "UseDieterLavaSlide=0"),
    ("dieter", 3, 0, "current preset", ""),
    ("dieter", 3, 0, "Blast Forge off", "UseDieterBlastForge=0"),
    ("dieter", 3, 0, "AutoMobCount 3", "AutoMobCount=3"),
    ("dieter", 3, 0, "Magma Flow off", "UseDieterMagmaFlow=0"),
    ("eleanor", 1, 12, "current preset", ""),
    ("eleanor", 1, 12, "Blazing And Furious on", "UseEleanorBlazingAndFurious=1"),
    ("eleanor", 1, 22, "current preset", ""),
    ("eleanor", 1, 22, "Blazing And Furious on", "UseEleanorBlazingAndFurious=1"),
    ("eleanor", 1, 22, "The One Fighter Rises off", "UseEleanorTheOneFighterRises=0"),
    ("eleanor", 1, 22, "AutoComboSpheres 3", "AutoComboSpheres=3"),
    ("eleanor", 1, 22, "AutoComboSpheres 6", "AutoComboSpheres=6"),
    ("eleanor", 3, 22, "current preset", ""),
    ("eleanor", 3, 22, "Blazing And Furious on", "UseEleanorBlazingAndFurious=1"),
    ("eleanor", 3, 22, "The One Fighter Rises off", "UseEleanorTheOneFighterRises=0"),
    ("eleanor", 3, 22, "AutoComboSpheres 3", "AutoComboSpheres=3"),
    ("eleanor", 3, 40, "current preset", ""),
    ("eleanor", 3, 40, "Blazing And Furious on", "UseEleanorBlazingAndFurious=1"),
    ("eleanor", 5, 22, "current preset", ""),
    ("eleanor", 5, 22, "Blazing And Furious on", "UseEleanorBlazingAndFurious=1"),
]


def casts_text(skills):
    return "; ".join(f"{k.replace('MH_', '').replace('CRAW', 'CLAW').title().replace('_', ' ')} {n}"
                     for k, n in sorted(skills, key=lambda kv: -kv[1]))


def run(job):
    homun, mobs, spregen, overrides, config = job
    return homun_bench.run(homun, overrides, mobs, spregen, 180, config)


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--old-ref", default="HEAD", help="git ref holding the previous USER_AI/H_Config.lua")
    parser.add_argument("--out", default=os.path.join(REPO, "docs", "preset-tuning"))
    args = parser.parse_args()
    os.makedirs(args.out, exist_ok=True)
    new_config = os.path.join(REPO, "USER_AI", "H_Config.lua")
    with tempfile.TemporaryDirectory(prefix="azzyai-preset-") as tmp:
        old_config = os.path.join(tmp, "H_Config.lua")
        data = subprocess.run(["git", "show", f"{args.old_ref}:USER_AI/H_Config.lua"], cwd=REPO,
                              capture_output=True, check=True).stdout
        with open(old_config, "wb") as f:
            f.write(data)
        sha = subprocess.run(["git", "rev-parse", "--short", args.old_ref], cwd=REPO, capture_output=True,
                             text=True, check=True).stdout.strip()

        ab_jobs, ab_keys = [], []
        for homun in HOMUNCULI:
            for mobs in PACKS:
                spregen = DEFAULT_SPREGEN.get(homun, 60)
                ab_keys.append((homun, mobs, spregen))
                ab_jobs.append((homun, mobs, spregen, "", old_config))
                ab_jobs.append((homun, mobs, spregen, "", new_config))
        opt_jobs = [(h, m, s, o, new_config) for h, m, s, _l, o in OPTIONS]
        with ThreadPoolExecutor(max_workers=4) as pool:
            results = list(pool.map(run, ab_jobs + opt_jobs))
    ab_results, opt_results = results[:len(ab_jobs)], results[len(ab_jobs):]

    ab_header = ["homunculus", "monsters", "sp_regen_per_s", f"dps_before ({sha})", "dps_after", "dps_change_pct",
                 "damage_per_sp_before", "damage_per_sp_after", "casts_before", "casts_after"]
    ab_rows = []
    for i, (homun, mobs, spregen) in enumerate(ab_keys):
        old, new = ab_results[2 * i], ab_results[2 * i + 1]
        ab_rows.append([homun, mobs, spregen, old["dps"], new["dps"], f"{100 * (new['dps'] / old['dps'] - 1):+.0f}",
                        round(old["damage"] / old["sp"]) if old["sp"] else "",
                        round(new["damage"] / new["sp"]) if new["sp"] else "",
                        casts_text(old["skills"]), casts_text(new["skills"])])

    opt_header = ["homunculus", "monsters", "sp_regen_per_s", "variant", "overrides", "dps", "vs_current_preset_pct",
                  "damage_per_sp", "refused_casts", "casts"]
    opt_rows, reference = [], {}
    for (homun, mobs, spregen, label, over), r in zip(OPTIONS, opt_results):
        if label == "current preset":
            reference[(homun, mobs, spregen)] = r["dps"]
        ref = reference[(homun, mobs, spregen)]
        opt_rows.append([homun, mobs, spregen or DEFAULT_SPREGEN.get(homun, 60), label, over or "(none)", r["dps"],
                         f"{100 * (r['dps'] / ref - 1):+.0f}" if label != "current preset" else "0",
                         round(r["damage"] / r["sp"]) if r["sp"] else "", r["refused"], casts_text(r["skills"])])

    for name, header, rows in (("preset-before-after.csv", ab_header, ab_rows), ("preset-options.csv", opt_header, opt_rows)):
        with open(os.path.join(args.out, name), "w", newline="", encoding="utf-8-sig") as f:
            w = csv.writer(f)
            w.writerow(header)
            w.writerows(rows)
    with open(os.path.join(args.out, "README.md"), "w", newline="\n", encoding="utf-8") as f:
        f.write(markdown(sha, ab_rows, opt_rows))
    print(f"wrote {len(ab_rows)} before/after rows and {len(opt_rows)} option rows to {args.out} (previous preset {sha})")


def n(value):
    return f"{int(value):,}" if str(value).lstrip("-").isdigit() else str(value)


def table(header, rows):
    lines = ["| " + " | ".join(header) + " |", "|" + "|".join("---" for _ in header) + "|"]
    lines += ["| " + " | ".join(str(c) for c in row) + " |" for row in rows]
    return "\n".join(lines)


def markdown(sha, ab_rows, opt_rows):
    out = [
        "# Default preset tuning: comparison tables",
        "",
        "[Release notes](../RELEASE_NOTES.md#default-preset-tuning) | spreadsheets: "
        "[preset-before-after.csv](preset-before-after.csv), [preset-options.csv](preset-options.csv)",
        "",
        "Generated by `python tools/harness/preset_report.py`; do not edit by hand. Every number is the expected value of an "
        "offline simulation (`tools/harness/damage.lua`): a zero-defense Neutral dummy for 180 s, the level 275 homunculi "
        "of `tools/harness/fixtures/homun_stats.lua`, rAthena data of the commit pinned in `fixtures/rathena.lua`, no "
        "SP refill. It is **not** a measurement on a server: only Sonic Claw and Xeno Slasher were checked against "
        "measured damage. DPS is expected damage per second; damage/SP is expected damage divided by the SP the casts "
        "spent. SP regeneration is 60 SP/s unless stated (Eleanor 22 SP/s, measured on the test server). Monsters are "
        "the same dummy, standing together in a pack.",
        "",
        f"## 1. Previous preset (`{sha}`) against the current `USER_AI/H_Config.lua`",
        "",
        "No overrides on either side.",
        "",
    ]
    for homun in HOMUNCULI:
        out += [f"### {homun.title()} (before and after)", ""]
        rows = [[r[1], n(r[3]), n(r[4]), f"{r[5]}%", n(r[6]), n(r[7]), r[8] or "none", r[9] or "none"]
                for r in ab_rows if r[0] == homun]
        out += [table(["monsters", "DPS before", "DPS after", "change", "damage/SP before", "damage/SP after",
                       "casts before (180 s)", "casts after (180 s)"], rows), ""]
    out += [
        "## 2. One option changed at a time, against the current preset",
        "",
        "`vs preset` compares each row with the `current preset` row of the same group (same monsters and SP regeneration). "
        "The overrides are applied on top of the current preset.",
        "",
    ]
    for homun in HOMUNCULI:
        out += [f"### {homun.title()} (one option at a time)", ""]
        rows = [[r[1], r[2], r[3], f"`{r[4]}`", n(r[5]), f"{r[6]}%", n(r[7]), r[8], r[9] or "none"]
                for r in opt_rows if r[0] == homun]
        out += [table(["monsters", "SP/s", "variant", "overrides", "DPS", "vs preset", "damage/SP", "refused", "casts (180 s)"],
                      rows), ""]
    out += [
        "## Limits",
        "",
        "- Validated against measured damage only for Sonic Claw (Eleanor) and Xeno Slasher (Eira). The other skills "
        "use the same formulas but were never compared with the server.",
        "- Magma Flow (Dieter) is priced as one splash hit per cast; the in-game buff that triggers on attacks is not "
        "modelled, so its rows understate or overstate it. Ground units (Lava Slide, Blast Forge) are priced as hitting "
        "every monster in the area on every tick for the whole duration.",
        "- Status effects (Bleeding, Stun, Tinder Breaker's DEF reduction), buffs, the owner's skills and monster attacks "
        "are not modelled. The AI follows its skill-class priority, so these tables do not show what the best skill "
        "would be, only what the AI does with each configuration.",
        "- No official-server data: skill values differ there, and the preset keeps `ServerType = SERVER_OFFICIAL`.",
        "",
    ]
    return "\n".join(out)


if __name__ == "__main__":
    main()
