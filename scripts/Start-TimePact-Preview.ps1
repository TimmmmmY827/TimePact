$ErrorActionPreference = "Stop"

$projectRoot = Split-Path -Parent $PSScriptRoot
$vsDevCmd = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat"
$rustBin = "C:\Users\Tim\AppData\Local\Programs\Rust\bin"
$nodeModules = Join-Path $projectRoot "node_modules"

if (-not (Test-Path -LiteralPath $vsDevCmd)) {
    throw "Visual Studio Build Tools environment was not found: $vsDevCmd"
}

if (-not (Test-Path -LiteralPath (Join-Path $rustBin "cargo.exe"))) {
    throw "The TimePact Rust toolchain was not found: $rustBin"
}

if (-not (Test-Path -LiteralPath $nodeModules)) {
    throw "Dependencies are missing. Run npm install in $projectRoot first."
}

$environmentLines = cmd.exe /d /s /c "`"$vsDevCmd`" -arch=x64 -host_arch=x64 >nul && set"
foreach ($line in $environmentLines) {
    if ($line -match "^([^=][^=]*)=(.*)$") {
        Set-Item -Path "Env:$($matches[1])" -Value $matches[2]
    }
}

$env:Path = "$rustBin;$env:Path"
$env:CARGO_NET_OFFLINE = "true"

Set-Location -LiteralPath $projectRoot

Write-Host ""
Write-Host "TimePact Preview" -ForegroundColor Cyan
Write-Host "Frontend changes refresh automatically." -ForegroundColor DarkGray
Write-Host "Press Ctrl+C in this window to stop the preview." -ForegroundColor DarkGray
Write-Host ""

& npm.cmd run preview:dev
if ($LASTEXITCODE -ne 0) {
    throw "TimePact Preview exited with code $LASTEXITCODE."
}
