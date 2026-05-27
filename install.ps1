# Radhe AI — Windows Installer
$version = "v0.6.0"
$url = "https://github.com/DevWizard-Vandan/radhe-ai/releases/download/$version/radhe.exe"
$dest = "$env:USERPROFILE\.radhe\bin\radhe.exe"
Write-Host "Installing Radhe AI $version for Windows..." -ForegroundColor Cyan
# Create dirs
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.radhe\bin" | Out-Null
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.radhe\packs" | Out-Null
# Download binary
Invoke-WebRequest -Uri $url -OutFile $dest
# Add to PATH if not already present
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
$binDir = "$env:USERPROFILE\.radhe\bin"
if ($currentPath -notlike "*$binDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$currentPath;$binDir", "User")
    Write-Host "Added $binDir to PATH." -ForegroundColor Green
}
# Copy default packs
$packsDir = "$env:USERPROFILE\.radhe\packs"
Write-Host "Downloading starter packs..." -ForegroundColor Cyan
@("math", "cs", "science") | ForEach-Object {
    $packUrl = "https://raw.githubusercontent.com/DevWizard-Vandan/radhe-ai/main/packs/$_.md"
    Invoke-WebRequest -Uri $packUrl -OutFile "$packsDir\$_.md"
    Write-Host "  Downloaded $_.md" -ForegroundColor Green
}
Write-Host ""
Write-Host "Radhe AI installed! Restart your terminal, then run: radhe --version" -ForegroundColor Green
