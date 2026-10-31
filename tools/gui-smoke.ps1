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
    $tabs.SelectedIndex = 0
    $hom = $form.Controls.Find('homunculusConfigControl', $true)[0]
    $binding = $hom.GetType().GetProperty('SelectedObject',
        [Reflection.BindingFlags]'Instance, Public, NonPublic')
    $config = $binding.GetValue($hom, $null)
    $properties = [ComponentModel.TypeDescriptor]::GetProperties($config)
    $search = $hom.Controls.Find('settingsSearch', $true)[0]
    $sFilter = $hom.Controls.Find('homunculusSFilter', $true)[0]
    $baseFilter = $hom.Controls.Find('homunculusBaseFilter', $true)[0]
    $clear = $hom.Controls.Find('clearFilters', $true)[0]
    $categories = $hom.Controls.Find('homunculusCategories', $true)[0]
    $clear.PerformClick()
    $search.Text = 'Twister'
    $grid = $categories.TabPages[1].Controls[0]
    $visibleProperties = [ComponentModel.TypeDescriptor]::GetProperties($grid.SelectedObject)
    if ($visibleProperties.Count -ne 2 -or $null -eq $visibleProperties['UseEiraTwisterCutter']) {
        throw 'Search did not expose both Twister Cutter controls.'
    }
    $search.Text = 'variable cast time'
    $visibleProperties = [ComponentModel.TypeDescriptor]::GetProperties($grid.SelectedObject)
    if ($null -eq $visibleProperties['CastTimeRatio']) {
        throw 'Description search did not find CastTimeRatio.'
    }
    $search.Clear()
    $sFilter.SelectedItem = 'Eira'
    $baseFilter.SelectedItem = 'Amistr'
    $visibleProperties = [ComponentModel.TypeDescriptor]::GetProperties($grid.SelectedObject)
    $buffProperties = [ComponentModel.TypeDescriptor]::GetProperties($categories.TabPages[3].Controls[0].SelectedObject)
    if ($null -eq $visibleProperties['UseEiraTwisterCutter'] -or
        $null -eq $buffProperties['HealOwnerBreeze'] -or
        $null -ne $visibleProperties['UseEleanorCBC'] -or
        $null -ne $visibleProperties['SphereTrackInterval']) {
        throw 'Combined Eira/Amistr filters hid Eira options or included Eleanor options.'
    }
    $sFilter.SelectedItem = 'Eleanor'
    $visibleProperties = [ComponentModel.TypeDescriptor]::GetProperties($grid.SelectedObject)
    if ($null -eq $visibleProperties['SphereTrackInterval']) {
        throw 'Eleanor filter did not expose SphereTrackInterval.'
    }
    $search.Text = 'no-such-setting'
    $summary = $hom.Controls.Find('settingsSummary', $true)[0]
    if (!$summary.Text.StartsWith('No matching settings.')) {
        throw 'Empty search results need an explicit message.'
    }
    $clear.PerformClick()
    if ($search.Text -ne '' -or $sFilter.SelectedIndex -ne 0 -or $baseFilter.SelectedIndex -ne 0) {
        throw 'Clear filters did not reset the search and both type filters.'
    }
    foreach ($size in @(@(800, 580), @(1100, 760))) {
        $form.Size = New-Object Drawing.Size($size[0], $size[1])
        [System.Windows.Forms.Application]::DoEvents()
        $description = $hom.Controls.Find('settingsDescription', $true)[0]
        if ($categories.Height -lt 120 -or $description.Bottom -gt $description.Parent.ClientSize.Height -or
            $categories.Bottom -gt $description.Top -or $search.Width -lt 150) {
            throw "Homunculus layout is clipped at $($size[0]) x $($size[1])."
        }
    }
    $roundtrip = Join-Path $Sandbox 'H_Config.roundtrip.lua'
    $minimal = Join-Path $Sandbox 'H_Config.minimal.lua'
    try {
        Copy-Item (Join-Path $Sandbox 'H_Config.lua') $roundtrip -Force
        Add-Content $roundtrip "`r`n-- untouched test comment`r`nCustomUnknownOption = 123"
        $originalValues = @{}
        foreach ($match in [regex]::Matches([IO.File]::ReadAllText($roundtrip),
            '(?m)^([A-Za-z][A-Za-z0-9_]*)[ \t]*=[ \t]*([A-Za-z_]+|-?(?:\d+(?:\.\d*)?|\.\d+))')) {
            $originalValues[$match.Groups[1].Value] = $match.Groups[2].Value
        }
        $config.Save($roundtrip)
        $savedValues = @{}
        foreach ($match in [regex]::Matches([IO.File]::ReadAllText($roundtrip),
            '(?m)^([A-Za-z][A-Za-z0-9_]*)[ \t]*=[ \t]*([A-Za-z_]+|-?(?:\d+(?:\.\d*)?|\.\d+))')) {
            $savedValues[$match.Groups[1].Value] = $match.Groups[2].Value
        }
        foreach ($name in $originalValues.Keys) {
            $oldValue = $originalValues[$name]
            $newValue = $savedValues[$name]
            if ($oldValue -match '^-?[\d.]' -and $newValue -match '^-?[\d.]') {
                if ([double]::Parse($oldValue, [Globalization.CultureInfo]::InvariantCulture) -ne
                    [double]::Parse($newValue, [Globalization.CultureInfo]::InvariantCulture)) {
                    throw "Saving unedited settings changed $name from $oldValue to $newValue."
                }
            } elseif ($oldValue -ne $newValue) {
                throw "Saving unedited settings changed $name from $oldValue to $newValue."
            }
        }
        foreach ($name in @('EiraXenoSlasherLevel', 'EiraEraseCutterLevel', 'BayeriStahlHornLevel',
            'BayeriHailegeStarLevel', 'SeraParalyzeLevel', 'EleanorSilverveinLevel', 'EleanorMidnightLevel')) {
            $properties[$name].SetValue($config, 7)
            if ($properties[$name].GetValue($config) -ne 7) {
                throw "$name still caps an intermediate level at 5 or rounds it to 10."
            }
        }
        $properties['ServerType'].SetValue($config, [Enum]::Parse($properties['ServerType'].PropertyType, 'Private'))
        $properties['CastTimeRatio'].SetValue($config, 0.75)
        $properties['SphereTrackInterval'].SetValue($config, 200)
        $properties['AutoSkillDelay'].SetValue($config, 0)
        $properties['CastInterruptLimit'].SetValue($config, 2)
        $properties['CastInterruptPause'].SetValue($config, 4000)
        $properties['CastInterruptUnconfirmed'].SetValue($config, $true)
        foreach ($invalid in @(@('CastInterruptLimit', -1), @('CastInterruptPause', 0))) {
            $rejected = $false
            try { $properties[$invalid[0]].SetValue($config, $invalid[1]) } catch { $rejected = $true }
            if (!$rejected) { throw "$($invalid[0]) accepted an invalid value." }
        }
        $rejectedInterval = $false
        try { $properties['SphereTrackInterval'].SetValue($config, 0) } catch { $rejectedInterval = $true }
        if (!$rejectedInterval -or $config.SphereTrackInterval -ne 200) {
            throw 'SphereTrackInterval must reject zero without changing its value.'
        }
        $properties['UseEiraTwisterCutter'].SetValue($config, $true)
        $properties['EiraTwisterCutterLevel'].SetValue($config, 3)
        $properties['UseEleanorCBC'].SetValue($config, $false)
        $properties['UseBayeriGoldeneTone'].SetValue($config,
            [Enum]::Parse($properties['UseBayeriGoldeneTone'].PropertyType, 'ASAP'))
        $config.Save($roundtrip)
        $text = [IO.File]::ReadAllText($roundtrip)
        foreach ($pattern in @('(?m)^ServerType\s*=\s*SERVER_PRIVATE', '(?m)^CastTimeRatio\s*=\s*0\.75',
            '(?m)^EiraTwisterCutterLevel\s*=\s*3', '(?m)^UseEleanorCBC\s*=\s*0',
            '(?m)^UseBayeriGoldeneTone\s*=\s*3',
            '(?m)^SphereTrackInterval\s*=\s*200', '(?m)^AutoSkillDelay\s*=\s*0',
            '(?m)^CastInterruptLimit\s*=\s*2', '(?m)^CastInterruptPause\s*=\s*4000',
            '(?m)^CastInterruptUnconfirmed\s*=\s*1',
            '(?m)^CustomUnknownOption = 123', '(?m)^-- untouched test comment')) {
            if ($text -notmatch $pattern) { throw "Round-trip did not preserve or save: $pattern" }
        }
        $config.Open($roundtrip)
        if ($config.EiraTwisterCutterLevel -ne 3 -or $config.UseEleanorCBC -or
            $config.ServerType.ToString() -ne 'Private' -or $config.CastTimeRatio -ne 0.75 -or
            $config.SphereTrackInterval -ne 200 -or $config.AutoSkillDelay -ne 0 -or
            $config.CastInterruptLimit -ne 2 -or $config.CastInterruptPause -ne 4000 -or
            !$config.CastInterruptUnconfirmed) {
            throw 'Imported new settings did not match the saved values.'
        }
        foreach ($name in @('EiraXenoSlasherLevel', 'EiraEraseCutterLevel', 'BayeriStahlHornLevel',
            'BayeriHailegeStarLevel', 'SeraParalyzeLevel', 'EleanorSilverveinLevel', 'EleanorMidnightLevel')) {
            if ($properties[$name].GetValue($config) -ne 7) { throw "$name did not survive saving and loading." }
        }
        $config.EiraTwisterCutterLevel = 11
        $savedHash = (Get-FileHash $roundtrip).Hash
        $rejected = $false
        try { $config.Save($roundtrip) } catch { $rejected = $true }
        if (!$rejected -or (Get-FileHash $roundtrip).Hash -ne $savedHash) {
            throw 'Invalid skill levels must fail without modifying the configuration.'
        }
        $config.SetDefaults()
        if ($config.EiraTwisterCutterLevel -ne 10 -or $config.UseEiraTwisterCutter -or !$config.UseEleanorCBC -or
            $config.SphereTrackInterval -ne 800 -or $config.AutoSkillDelay -ne 400 -or
            $config.CastInterruptLimit -ne 3 -or $config.CastInterruptPause -ne 5000 -or
            $config.CastInterruptUnconfirmed) {
            throw 'Reset to defaults did not reset the new settings.'
        }
        [IO.File]::WriteAllText($minimal, "-- ServerType = SERVER_PRIVATE`r`nServerType = SERVER_OFFICIAL -- test`r`n")
        $config.Open($minimal)
        if ($config.ServerType.ToString() -ne 'Official') {
            throw 'A commented server setting overrode the actual assignment.'
        }
        $config.Revert()
        $hom.Refresh()
    } finally {
        Remove-Item $roundtrip, $minimal -ErrorAction SilentlyContinue
    }
    $form.Controls.Find('buttonApply', $true)[0].Enabled = $false
    $form.Close()
    Get-ChildItem $Sandbox -Filter '*.lua' | ForEach-Object {
        if ($before[$_.Name] -ne (Get-FileHash $_.FullName).Hash) {
            throw "Opening the GUI unexpectedly changed $($_.Name)."
        }
    }
    Write-Host 'PASS: seven tabs, resizing, search, type filters, skill limits, settings round-trip/defaults/import, and unchanged source Lua.'
} finally {
    if ($null -ne $form) {
        $form.Dispose()
    }
    [Environment]::CurrentDirectory = $previousDirectory
}
