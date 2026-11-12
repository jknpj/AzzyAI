#!/usr/bin/env python3
"""Runs the AzzyAI checks: Lua syntax for every file, then the offline AI scenarios.

Usage:
    python tools/harness/run.py                 # syntax, version, option and scenario checks
    python tools/harness/run.py bayeri dieter   # only these scenarios
    python tools/harness/run.py --verbose sera  # also print every cast
    python tools/harness/run.py --globals       # globals assigned inside functions (missing 'local')

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


def check_version():
    """CVersion (USER_AI/Const_.lua) is the release number: CHANGELOG.md and the Windows app's
    AssemblyInfo.cs must say the same, and the three file-check numbers (AI.lua and AI_M.lua Version,
    AI_main.lua MainVersion, AzzyUtil.lua AUVersion) must be its MAJOR.MINOR, e.g. 2.0.0 -> "2.0"."""
    def read(name, pattern):
        with open(os.path.join(USER_AI, name), encoding="utf-8", errors="replace") as f:
            return re.search(pattern, f.read(), re.M)

    code = read("Const_.lua", r'^CVersion="(\d+)\.(\d+)\.(\d+)"')
    if not code:
        print('VERSION ERROR: CVersion="MAJOR.MINOR.PATCH" not found in USER_AI/Const_.lua')
        return False
    major, minor, patch = (int(g) for g in code.groups())
    release = f"{major}.{minor}.{patch}"
    ok = True
    with open(os.path.join(REPO, "GUI", "AzzyAIConfig", "Properties", "AssemblyInfo.cs"),
              encoding="utf-8-sig", errors="replace") as f:
        asm = f.read()
    for attr in ("AssemblyVersion", "AssemblyFileVersion"):
        m = re.search(r'^\[assembly: %s\("([^"]*)"\)\]' % attr, asm, re.M)
        if not m or m.group(1) != release:
            print(f'VERSION ERROR: GUI AssemblyInfo.cs {attr} is {m.group(1) if m else "missing"}, release is {release}')
            ok = False
    with open(os.path.join(REPO, "CHANGELOG.md"), encoding="utf-8") as f:
        notes = re.search(r"Current release:\s*\**(\d+\.\d+\.\d+)", f.read())
    if not notes or notes.group(1) != release:
        print(f"VERSION ERROR: Const_.lua says {release}, CHANGELOG.md says "
              f"{notes.group(1) if notes else 'no \"Current release\" line'}")
        ok = False
    expected = f"{major}.{minor}"
    for name, var in (("AI.lua", "Version"), ("AI_M.lua", "Version"), ("AI_main.lua", "MainVersion"),
                      ("AzzyUtil.lua", "AUVersion")):
        m = read(name, rf'^{var}="([^"]*)"')
        if not m or m.group(1) != expected:
            print(f'VERSION ERROR: {name} {var} is {m.group(1) if m else "missing"}, release {release} needs "{expected}"')
            ok = False
    if ok:
        print(f"version: {release} (file check {expected})")
    return ok


# Options that exist without a reader on purpose. Keep this empty unless an option really must
# exist for a plugin or for the config files only.
UNUSED_OK = set()

# the shipped configuration files: every option they list has to be read by the AI
CONFIG_FILES = ("Defaults.lua", "H_Config.lua", "M_Config.lua")


def check_options():
    """Every option in the shipped configs must be read by some USER_AI file. A name nothing reads
    is a switch the player can change with no effect: KiteParanoid, MoveStickyFight and the
    Berzerk_* mis-spellings of the mercenary options were all like that. The config files and
    Const_.lua are definitions, not readers, so they are left out of the search."""
    def read(name):
        with open(os.path.join(USER_AI, name), encoding="utf-8", errors="replace") as f:
            return f.read()
    defined = {}
    for cfg in CONFIG_FILES:
        for name in re.findall(r"^\s*([A-Za-z_]\w*)\s*=", read(cfg), re.M):
            defined.setdefault(name, []).append(cfg)
    blob = "\n".join(read(n) for n in sorted(os.listdir(USER_AI))
                     if n.endswith(".lua") and n not in CONFIG_FILES and n != "Const_.lua")
    unused = [n for n in sorted(defined)
              if n not in UNUSED_OK and not re.search(r"\b%s\b" % re.escape(n), blob)]
    for name in unused:
        print(f"OPTION ERROR: {name} is in {'/'.join(defined[name])} but nothing in USER_AI reads it")
    print(f"options: {len(defined)} in {'/'.join(CONFIG_FILES)}, {len(unused)} unread")
    return not unused


def all_scenarios():
    with open(os.path.join(HERE, "scenarios.lua"), encoding="utf-8") as f:
        text = f.read()
    body = text.split("SCENARIOS = {", 1)[1]
    return re.findall(r"^\t(\w+)\s*=\s*(?:\{|\w+\()", body, re.M)


def run_scenario(lua, name, ticks, env):
    with tempfile.TemporaryDirectory(prefix="azzyai-") as tmp:
        dst = os.path.join(tmp, "AI", "USER_AI")
        ignore = ["*.exe", "*.pdb", "*.pdf", "data"]
        if env.get("HARNESS_NO_TYPES"):
            ignore.append("Types")  # the benchmarks compare the global preset alone against the per-type files
        shutil.copytree(USER_AI, dst, ignore=shutil.ignore_patterns(*ignore))
        os.makedirs(os.path.join(dst, "data"))
        # fixed config so the tests don't depend on the user's own H_Config.lua; HARNESS_CONFIG
        # points at another one (the benchmarks compare against the shipped USER_AI/H_Config.lua)
        shutil.copy(env.get("HARNESS_CONFIG") or os.path.join(HERE, "fixtures", "H_Config.lua"),
                    os.path.join(dst, "H_Config.lua"))
        r = subprocess.run([lua, os.path.join(HERE, "harness.lua"), name, str(ticks)],
                           cwd=tmp, capture_output=True, text=True, env=env, timeout=300)
        sys.stdout.write(r.stdout)
        if r.stderr:
            sys.stdout.write(r.stderr)
        return r.returncode == 0


def check_globals(luac):
    """Lists globals assigned inside functions (SETGLOBAL outside the main chunk): usually a missing 'local'."""
    found = {}
    for name in sorted(os.listdir(USER_AI)):
        if not name.endswith(".lua"):
            continue
        path = os.path.join(USER_AI, name)
        r = subprocess.run([luac, "-l", "-p", path], capture_output=True, text=True)
        scope = None
        for line in r.stdout.splitlines():
            m = re.match(r"(main|function) <[^:]*:(\d+),", line)
            if m:
                scope = None if m.group(1) == "main" else int(m.group(2))
                continue
            m = re.search(r"\bSETGLOBAL\b.*;\s*(\S+)", line)
            if m and scope is not None:
                found.setdefault(m.group(1), set()).add(f"{name}:{scope}")
    for g in sorted(found):
        print(f"  {g:28} {', '.join(sorted(found[g], key=lambda s: (s.split(':')[0], int(s.split(':')[1]))))}")
    print(f"globals assigned inside functions: {len(found)}")


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("scenarios", nargs="*", help="scenario names (default: all)")
    parser.add_argument("--ticks", type=int, default=300, help="AI calls per scenario, 100 ms each")
    parser.add_argument("--verbose", action="store_true", help="print every cast")
    parser.add_argument("--trace", action="store_true", help="print TraceAI output")
    parser.add_argument("--profile", action="store_true", help="print time per AI() call and the hottest functions")
    parser.add_argument("--globals", action="store_true",
                        help="only list globals assigned inside functions of USER_AI (missing 'local' candidates)")
    parser.add_argument("--skip-syntax", action="store_true", help="skip syntax, version and option checks")
    args = parser.parse_args()

    lua = find_tool("LUA", ["lua5.1", "lua51", "lua"])
    luac = find_tool("LUAC", ["luac5.1", "luac51", "luac"])
    if args.globals:
        check_globals(luac)
        return
    ok = True
    if not args.skip_syntax:
        ok = check_syntax(luac)
        ok = check_version() and ok
        ok = check_options() and ok

    env = dict(os.environ, HARNESS_VERBOSE="1" if args.verbose else "0", HARNESS_TRACE="1" if args.trace else "0",
               HARNESS_PROFILE="1" if args.profile else "0")
    names = args.scenarios or all_scenarios()
    passed = sum(run_scenario(lua, n, args.ticks, env) for n in names)
    print(f"scenarios: {passed}/{len(names)} passed")
    ok = ok and passed == len(names)
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
