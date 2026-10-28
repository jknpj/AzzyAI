#!/usr/bin/env python3
"""Runs the AzzyAI checks: Lua syntax for every file, then the offline AI scenarios.

Usage:
    python tools/harness/run.py                 # syntax check + all scenarios
    python tools/harness/run.py bayeri dieter   # only these scenarios
    python tools/harness/run.py --verbose sera  # also print every cast

Needs a Lua 5.1 interpreter (the version embedded in the RO client). It is looked up in the
LUA / LUAC environment variables, then on PATH as lua5.1 / lua51 / lua.
"""
import argparse
import os
import re
import shutil
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", ".."))
USER_AI = os.path.join(REPO, "USER_AI")


def find_tool(env_name, names):
    if os.environ.get(env_name):
        return os.environ[env_name]
    for name in names:
        path = shutil.which(name)
        if path:
            return path
    local = os.path.join(os.environ.get("LOCALAPPDATA", ""), "lua51")
    for name in names:
        path = os.path.join(local, name + ".exe")
        if os.path.exists(path):
            return path
    sys.exit(f"Lua 5.1 not found: set {env_name} or put one of {names} on PATH")


def check_syntax(luac):
    failed = 0
    files = []
    for root, dirs, names in os.walk(REPO):
        dirs[:] = [d for d in dirs if d not in (".git", "node_modules")]
        files += [os.path.join(root, n) for n in names if n.endswith(".lua")]
    for path in sorted(files):
        r = subprocess.run([luac, "-p", path], capture_output=True, text=True)
        if r.returncode != 0:
            failed += 1
            print("SYNTAX ERROR", (r.stderr or r.stdout).strip())
    print(f"syntax: {len(files)} Lua files, {failed} with errors")
    return failed == 0


def all_scenarios():
    with open(os.path.join(HERE, "scenarios.lua"), encoding="utf-8") as f:
        text = f.read()
    body = text.split("SCENARIOS = {", 1)[1]
    return re.findall(r"^\t(\w+)\s*=\s*\{", body, re.M)


def run_scenario(lua, name, ticks, env):
    with tempfile.TemporaryDirectory(prefix="azzyai-") as tmp:
        dst = os.path.join(tmp, "AI", "USER_AI")
        shutil.copytree(USER_AI, dst, ignore=shutil.ignore_patterns("*.exe", "*.pdb", "*.pdf", "data"))
        os.makedirs(os.path.join(dst, "data"))
        # fixed config so the tests don't depend on the user's own H_Config.lua
        shutil.copy(os.path.join(HERE, "fixtures", "H_Config.lua"), os.path.join(dst, "H_Config.lua"))
        r = subprocess.run([lua, os.path.join(HERE, "harness.lua"), name, str(ticks)],
                           cwd=tmp, capture_output=True, text=True, env=env, timeout=300)
        sys.stdout.write(r.stdout)
        if r.stderr:
            sys.stdout.write(r.stderr)
        return r.returncode == 0


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("scenarios", nargs="*", help="scenario names (default: all)")
    parser.add_argument("--ticks", type=int, default=300, help="AI calls per scenario, 100 ms each")
    parser.add_argument("--verbose", action="store_true", help="print every cast")
    parser.add_argument("--trace", action="store_true", help="print TraceAI output")
    parser.add_argument("--skip-syntax", action="store_true")
    args = parser.parse_args()

    lua = find_tool("LUA", ["lua5.1", "lua51", "lua"])
    luac = find_tool("LUAC", ["luac5.1", "luac51", "luac"])
    ok = True
    if not args.skip_syntax:
        ok = check_syntax(luac)

    env = dict(os.environ, HARNESS_VERBOSE="1" if args.verbose else "0", HARNESS_TRACE="1" if args.trace else "0")
    names = args.scenarios or all_scenarios()
    passed = sum(run_scenario(lua, n, args.ticks, env) for n in names)
    print(f"scenarios: {passed}/{len(names)} passed")
    ok = ok and passed == len(names)
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
