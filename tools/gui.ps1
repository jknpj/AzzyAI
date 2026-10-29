[CmdletBinding()]
param(
    [ValidateSet('Build', 'Prepare', 'Run', 'Test')]
    [string]$Action = 'Build',
    [ValidateSet('Debug', 'Release')]
    [string]$Configuration = 'Debug'
)

$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
$output = Join-Path $root ".artifacts\gui\$Configuration"
$sandbox = Join-Path $root ".artifacts\gui\run\$Configuration"
$project = Join-Path $root 'GUI\AzzyAIConfig\AzzyAIConfig.csproj'

$vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
if (!(Test-Path $vswhere)) {
    throw 'Install Visual Studio Build Tools with the .NET desktop build tools workload. See README.md.'
}
$msbuild = & $vswhere -latest -products * -requires Microsoft.Component.MSBuild -find 'MSBuild\**\Bin\MSBuild.exe'
if (!$msbuild) {
    throw 'MSBuild was not found. Install the .NET desktop build tools workload.'
}
$msbuild = @($msbuild)[0]

& $msbuild $project /restore /nologo /verbosity:minimal `
    "/p:Configuration=$Configuration" /p:Platform=x86
if ($LASTEXITCODE -ne 0) {
    throw "GUI build failed (exit code $LASTEXITCODE)."
}
if ($Action -eq 'Build') {
    Write-Host "Built: $output\AzzyAIConfig.exe"
    return
}

$executable = Join-Path $sandbox 'AzzyAIConfig.exe'
$running = Get-Process -Name AzzyAIConfig -ErrorAction SilentlyContinue |
    Where-Object { $_.Path -eq $executable }
if ($running) {
    throw "Close the GUI running from $sandbox before refreshing its files."
}
New-Item -ItemType Directory -Force $sandbox | Out-Null
Copy-Item "$output\*" $sandbox -Force
foreach ($file in @('Defaults.lua', 'H_Config.lua', 'M_Config.lua', 'H_Extra.lua',
    'M_Extra.lua', 'H_Tactics.lua', 'M_Tactics.lua', 'H_PVP_Tact.lua', 'M_PVP_Tact.lua')) {
    Copy-Item (Join-Path $root "USER_AI\$file") $sandbox -Force
}

if ($Action -eq 'Run') {
    $process = Start-Process $executable -WorkingDirectory $sandbox -PassThru
    Start-Sleep -Seconds 2
    $process.Refresh()
    if ($process.HasExited -or !$process.Responding -or $process.MainWindowHandle -eq 0) {
        throw 'The GUI did not open a responsive window. Run the GUI smoke test for startup diagnostics.'
    }
    Write-Host "GUI running (PID $($process.Id)) using disposable configs in $sandbox"
} elseif ($Action -eq 'Test') {
    $powershell = Join-Path $env:WINDIR 'SysWOW64\WindowsPowerShell\v1.0\powershell.exe'
    & $powershell -NoProfile -STA -ExecutionPolicy Bypass -File `
        (Join-Path $PSScriptRoot 'gui-smoke.ps1') -Sandbox $sandbox
    if ($LASTEXITCODE -ne 0) {
        throw "GUI smoke test failed (exit code $LASTEXITCODE)."
    }
} else {
    Write-Host "Prepared disposable configs in $sandbox"
}
