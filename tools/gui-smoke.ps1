[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$Sandbox
)

$ErrorActionPreference = 'Stop'
if ([IntPtr]::Size -ne 4 -or [Threading.Thread]::CurrentThread.ApartmentState -ne 'STA') {
    throw 'Run this test through tools\gui.ps1 -Action Test (32-bit STA Windows PowerShell required).'
}

Add-Type -AssemblyName System.Windows.Forms
[System.Windows.Forms.Application]::SetUnhandledExceptionMode(
    [System.Windows.Forms.UnhandledExceptionMode]::ThrowException)
[System.Windows.Forms.Application]::EnableVisualStyles()
[System.Windows.Forms.Application]::SetCompatibleTextRenderingDefault($false)
$previousDirectory = [Environment]::CurrentDirectory
$form = $null
try {
    [Environment]::CurrentDirectory = (Resolve-Path $Sandbox).Path
    $before = @{}
    Get-ChildItem $Sandbox -Filter '*.lua' | ForEach-Object {
        $before[$_.Name] = (Get-FileHash $_.FullName).Hash
    }
    $assembly = [Reflection.Assembly]::LoadFrom((Join-Path $Sandbox 'AzzyAIConfig.exe'))
    $form = [Activator]::CreateInstance($assembly.GetType('AzzyAIConfig.MainForm', $true))
    $form.ShowInTaskbar = $false
    $form.Show()
    [System.Windows.Forms.Application]::DoEvents()
    $tabs = $form.Controls.Find('tabControl1', $true)[0]
    $expectedTabs = @('Homunculus', 'Mercenary', 'Homunculus Tactics',
        'Mercenary Tactics', 'PVP Tactics', 'Merc PVP Tactics', 'Extra Options')
    if ($tabs.TabPages.Count -ne $expectedTabs.Count) {
        throw "Expected $($expectedTabs.Count) tabs, found $($tabs.TabPages.Count)."
    }
    foreach ($name in $expectedTabs) {
        $page = @($tabs.TabPages | Where-Object { $_.Text -eq $name })
        if ($page.Count -ne 1) {
            throw "Expected exactly one '$name' tab."
        }
        $tabs.SelectedTab = $page[0]
        [System.Windows.Forms.Application]::DoEvents()
        if (!$page[0].Visible) {
            throw "Tab '$name' did not become visible."
        }
    }
    foreach ($name in @('homunculusConfigControl', 'propertyGridMercenary')) {
        $control = $form.Controls.Find($name, $true)[0]
        $property = $control.GetType().GetProperty('SelectedObject',
            [Reflection.BindingFlags]'Instance, Public, NonPublic')
        if ($null -eq $property -or $null -eq $property.GetValue($control, $null)) {
            throw "Configuration binding is missing for '$name'."
        }
    }
    $form.Close()
    Get-ChildItem $Sandbox -Filter '*.lua' | ForEach-Object {
        if ($before[$_.Name] -ne (Get-FileHash $_.FullName).Hash) {
            throw "Opening the GUI unexpectedly changed $($_.Name)."
        }
    }
    Write-Host 'PASS: desktop GUI opens, all seven tabs render, configs bind, and Lua files remain unchanged.'
} finally {
    if ($null -ne $form) {
        $form.Dispose()
    }
    [Environment]::CurrentDirectory = $previousDirectory
}
