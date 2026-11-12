@echo off
rem Builds the Windows configuration app (Release, x86) and copies AzzyAIConfig.exe into USER_AI.
rem Same build as tools\gui.ps1 -Action Build -Configuration Release, without PowerShell.
rem Needs Visual Studio or Build Tools with the ".NET desktop build tools" workload, and
rem internet access to nuget.org the first time (behind a proxy, set HTTPS_PROXY first).
setlocal
set "ROOT=%~dp0.."
set "VSWHERE=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe"
if not exist "%VSWHERE%" goto :novs

set "MSBUILD="
for /f "usebackq delims=" %%i in (`"%VSWHERE%" -latest -products * -requires Microsoft.Component.MSBuild -find MSBuild\**\Bin\MSBuild.exe`) do if not defined MSBUILD set "MSBUILD=%%i"
if not defined MSBUILD goto :novs

echo MSBuild: %MSBUILD%
"%MSBUILD%" "%ROOT%\GUI\AzzyAIConfig\AzzyAIConfig.csproj" /restore /nologo /verbosity:minimal /p:Configuration=Release /p:Platform=x86 /p:Deterministic=true
if errorlevel 1 goto :fail

set "BUILT=%ROOT%\.artifacts\gui\Release\AzzyAIConfig.exe"
if not exist "%BUILT%" (
  echo The build finished but %BUILT% was not found.
  goto :fail
)
rem a running copy locks the file: close the app first
copy /y "%BUILT%" "%ROOT%\USER_AI\AzzyAIConfig.exe" >nul
if errorlevel 1 (
  echo Could not copy the app into USER_AI. Close AzzyAIConfig.exe if it is running, then try again.
  goto :fail
)
echo.
echo Built and copied: %ROOT%\USER_AI\AzzyAIConfig.exe
exit /b 0

:novs
echo Visual Studio Build Tools with the ".NET desktop build tools" workload were not found.
echo See docs\RELEASE_NOTES.md, "Windows desktop GUI development".
exit /b 1

:fail
echo.
echo *** The build failed, see the messages above. USER_AI\AzzyAIConfig.exe was not changed. ***
exit /b 1
