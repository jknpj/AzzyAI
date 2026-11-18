#!/usr/bin/env python3
"""Checks that the web GUI's embedded H_Config.lua and M_Config.lua are the shipped preset.

The web page starts a new configuration from `webgui/templates.js`. When that copy drifts from
`USER_AI/H_Config.lua`/`M_Config.lua`, a player who configures through the web page gets different
defaults from the ones the AI ships with (LavaSlideMode was 1 in the copy and 0 in the preset, so
his Dieter laid the lava at the owner's feet instead of on the monsters).

Usage:
    python tools/webgui_templates_check.py
"""
import os
import re
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))


def normalize(text):
    return text.replace("\r\n", "\n").strip("\n")


def embedded(templates_js, name):
    pattern = re.compile(r"'%s': String\.raw`(.*?)`" % re.escape(name), re.S)
    match = pattern.search(templates_js)
    return match.group(1) if match else None


def main():
    templates_path = os.path.join(REPO, "webgui", "templates.js")
    if not os.path.exists(templates_path):
        print("webgui/templates.js not found (submodule not checked out), skipping")
        return 0
    with open(templates_path, encoding="utf-8") as f:
        templates_js = f.read()
    failed = False
    for name in ("H_Config.lua", "M_Config.lua"):
        shipped_path = os.path.join(REPO, "USER_AI", name)
        with open(shipped_path, encoding="utf-8", errors="replace") as f:
            shipped = normalize(f.read())
        body = embedded(templates_js, name)
        if body is None:
            print(f"ERROR: webgui/templates.js has no '{name}' template")
            failed = True
            continue
        body = normalize(body)
        if body == shipped:
            print(f"{name}: matches USER_AI/{name}")
            continue
        failed = True
        shipped_lines, body_lines = shipped.split("\n"), body.split("\n")
        print(f"ERROR: the web GUI copy of {name} differs from USER_AI/{name}")
        for i in range(max(len(shipped_lines), len(body_lines))):
            a = shipped_lines[i] if i < len(shipped_lines) else "<missing>"
            b = body_lines[i] if i < len(body_lines) else "<missing>"
            if a != b:
                print(f"  line {i + 1}: USER_AI: {a}")
                print(f"  line {i + 1}: webgui : {b}")
                break
        print(f"  copy USER_AI/{name} into webgui/templates.js (String.raw body) to fix it")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
