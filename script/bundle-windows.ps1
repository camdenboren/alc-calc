# https://stackoverflow.com/questions/57949031/powershell-script-stops-if-program-fails-like-bash-set-o-errexit
$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if ($env:CARGO)
{
    $Cargo = $env:CARGO
} elseif (Get-Command "cargo" -ErrorAction SilentlyContinue)
{
    $Cargo = "cargo"
} else
{
    Write-Error "Could not find cargo in path." -ErrorAction Stop
}

function Green
{
    process { Write-Host $_ -ForegroundColor Green }
}
function Red
{
    process { Write-Host $_ -ForegroundColor Red }
}
function Yellow
{
    process { Write-Host $_ -ForegroundColor Yellow }
}

Write-Output "`ncargo packager..." | Yellow
& $Cargo packager --release
Resolve-Path -Path .\target\release\alc-calc_*.exe | boxes -d ansi

if ($?) {
    Write-Output "`nBuild successful`n" | Green
    exit 0
}
else {
    Write-Output "`nBuild failed`n" | Red
    exit 1
}
