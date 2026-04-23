<#
.SYNOPSIS
    Mailify installer for Windows.
.EXAMPLE
    irm https://mailify.donilite.me/install.ps1 | iex
.EXAMPLE
    & ([ScriptBlock]::Create((irm https://mailify.donilite.me/install.ps1))) -Version v0.2.0 -InstallDir C:\tools\mailify
#>
[CmdletBinding()]
param(
    [string] $Version      = $(if ($env:MAILIFY_VERSION)      { $env:MAILIFY_VERSION }      else { 'latest' }),
    [string] $InstallDir   = $(if ($env:MAILIFY_INSTALL_DIR)  { $env:MAILIFY_INSTALL_DIR }  else { "$env:LOCALAPPDATA\Programs\mailify" }),
    [string] $Repo         = $(if ($env:MAILIFY_REPO)         { $env:MAILIFY_REPO }         else { 'donilite/mailify' }),
    [switch] $NoVerify
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

function Write-Log { param([string]$Msg) Write-Host "→ $Msg" -ForegroundColor Cyan }
function Fail      { param([string]$Msg) Write-Host "✗ $Msg" -ForegroundColor Red; exit 1 }

# --- detect arch ---
$arch = $env:PROCESSOR_ARCHITECTURE
switch ($arch) {
    'AMD64' { $target = 'x86_64-pc-windows-msvc' }
    default { Fail "unsupported arch: $arch" }
}

# --- resolve version ---
if ($Version -eq 'latest') {
    Write-Log "Resolving latest release tag from github.com/$Repo"
    $api = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest" -UseBasicParsing
    $Version = $api.tag_name
    if (-not $Version) { Fail "could not resolve latest version" }
}
$versionNoV = $Version.TrimStart('v')
$archive = "mailify-$versionNoV-$target.zip"
$url     = "https://github.com/$Repo/releases/download/$Version/$archive"
$shaUrl  = "$url.sha256"

# --- download ---
$tmp = Join-Path ([IO.Path]::GetTempPath()) ("mailify-" + [Guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $tmp | Out-Null
try {
    Write-Log "Downloading $archive"
    $archivePath = Join-Path $tmp $archive
    Invoke-WebRequest -Uri $url -OutFile $archivePath -UseBasicParsing

    if (-not $NoVerify) {
        Write-Log "Verifying SHA256"
        $shaPath = "$archivePath.sha256"
        Invoke-WebRequest -Uri $shaUrl -OutFile $shaPath -UseBasicParsing
        $expected = (Get-Content $shaPath -First 1).Split(' ')[0].ToLower()
        $actual   = (Get-FileHash $archivePath -Algorithm SHA256).Hash.ToLower()
        if ($expected -ne $actual) { Fail "checksum mismatch (expected $expected, got $actual)" }
    }

    Write-Log "Extracting"
    Expand-Archive -Path $archivePath -DestinationPath $tmp -Force
    $stage = Join-Path $tmp "mailify-$versionNoV-$target"

    # --- install binary ---
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    Copy-Item -Force (Join-Path $stage 'mailify.exe') (Join-Path $InstallDir 'mailify.exe')

    # --- install templates to %APPDATA%\mailify\templates ---
    $dataDir = Join-Path $env:APPDATA 'mailify'
    New-Item -ItemType Directory -Force -Path $dataDir | Out-Null
    $tplDest = Join-Path $dataDir 'templates'
    if (Test-Path $tplDest) { Remove-Item -Recurse -Force $tplDest }
    Copy-Item -Recurse (Join-Path $stage 'templates') $tplDest

    Write-Log "Installed mailify $Version → $InstallDir\mailify.exe"
    Write-Log "Templates → $tplDest"
    Write-Log "Set MAILIFY_TEMPLATES__PATH=$tplDest or put path in Mailify.toml"

    $currentPath = [Environment]::GetEnvironmentVariable('Path', 'User')
    if (($currentPath -split ';') -notcontains $InstallDir) {
        Write-Log "Adding $InstallDir to user PATH"
        [Environment]::SetEnvironmentVariable('Path', "$currentPath;$InstallDir", 'User')
        Write-Log "Open a new terminal for PATH to take effect."
    }
}
finally {
    Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
}
