@echo off
rem Regenerates the homunculus benchmarks: docs\preset-tuning\* and the table in docs\RELEASE_NOTES.md.
rem Previous preset = USER_AI\H_Config.lua at commit 0cc1f35, no Types files; current = H_Config.lua plus USER_AI\Types\<Name>.lua.
rem Needs python and Lua 5.1 like tools\harness\run.py. About a minute, 4 runs in parallel.
setlocal
cd /d "%~dp0.." || exit /b 1
python tools\harness\preset_report.py --old-ref 0cc1f35
if errorlevel 1 (
  echo.
  echo *** The benchmark failed, see the messages above. Nothing was committed. ***
  exit /b 1
)
echo.
echo Done. Review the changes with: git diff --stat docs
exit /b 0
