Write-Host "Installing Radhe AI — Offline Terminal Assistant for Students" -ForegroundColor Cyan

$RadheDir = Join-Path $HOME ".radhe"
$BinDir = Join-Path $RadheDir "bin"
$ModelsDir = Join-Path $RadheDir "models"

try {
    # 1. Create directories
    Write-Host "Creating directories..." -ForegroundColor Yellow
    if (-not (Test-Path $BinDir)) {
        New-Item -ItemType Directory -Path $BinDir -Force | Out-Null
    }
    if (-not (Test-Path $ModelsDir)) {
        New-Item -ItemType Directory -Path $ModelsDir -Force | Out-Null
    }

    # 2. Download radhe.exe
    $RadheUrl = "https://github.com/DevWizard-Vandan/radhe-ai/releases/latest/download/radhe.exe"
    $RadheDest = Join-Path $BinDir "radhe.exe"
    Write-Host "Downloading radhe.exe from $RadheUrl..." -ForegroundColor Yellow
    Invoke-WebRequest -Uri $RadheUrl -OutFile $RadheDest -ShowProgress

    # 3. Get latest llama.cpp release asset dynamically
    $LlamaReleaseUrl = "https://api.github.com/repos/ggml-org/llama.cpp/releases/latest"
    Write-Host "Fetching latest llama.cpp release info..." -ForegroundColor Yellow
    $LlamaRelease = Invoke-RestMethod -Uri $LlamaReleaseUrl
    
    $LlamaAsset = $LlamaRelease.assets | Where-Object { $_.name -like "*bin-win-cpu-x64.zip" }
    if ($null -eq $LlamaAsset) {
        throw "Could not find latest llama.cpp CPU build matching *bin-win-cpu-x64.zip in release assets."
    }

    $LlamaUrl = $LlamaAsset.browser_download_url
    $LlamaZipName = $LlamaAsset.name
    $LlamaZip = Join-Path $env:TEMP "llama.zip"
    
    Write-Host "Downloading latest llama.cpp zip ($LlamaZipName) from $LlamaUrl..." -ForegroundColor Yellow
    Invoke-WebRequest -Uri $LlamaUrl -OutFile $LlamaZip -ShowProgress

    # 4. Extract llama-completion.exe and dll files
    Write-Host "Extracting llama.cpp dependencies..." -ForegroundColor Yellow
    $TempExtract = Join-Path $env:TEMP "llama_extract"
    if (Test-Path $TempExtract) {
        Remove-Item -Path $TempExtract -Recurse -Force | Out-Null
    }
    New-Item -ItemType Directory -Path $TempExtract -Force | Out-Null
    Expand-Archive -Path $LlamaZip -DestinationPath $TempExtract -Force

    Get-ChildItem -Path $TempExtract -Filter "llama-completion.exe" -Recurse | Copy-Item -Destination $BinDir -Force
    Get-ChildItem -Path $TempExtract -Filter "*.dll" -Recurse | Copy-Item -Destination $BinDir -Force

    # Clean up temp zip and extract directory
    Remove-Item -Path $LlamaZip -Force | Out-Null
    Remove-Item -Path $TempExtract -Recurse -Force | Out-Null

    # 5. Download model Qwen2.5-Coder-1.5B-Instruct-Q4_K_M.gguf
    $ModelUrl = "https://huggingface.co/Qwen/Qwen2.5-Coder-1.5B-Instruct-GGUF/resolve/main/qwen2.5-coder-1.5b-instruct-q4_k_m.gguf"
    $ModelDest = Join-Path $ModelsDir "Qwen2.5-Coder-1.5B-Instruct-Q4_K_M.gguf"
    Write-Host "Downloading Qwen2.5-Coder 1.5B model (~1GB)..." -ForegroundColor Yellow
    Invoke-WebRequest -Uri $ModelUrl -OutFile $ModelDest -ShowProgress

    # 6. Add $BinDir to User PATH permanently
    Write-Host "Updating system Environment PATH..." -ForegroundColor Yellow
    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($UserPath -notlike "*$BinDir*") {
        $NewPath = "$UserPath;$BinDir"
        $NewPath = $NewPath -replace ';;', ';'
        [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
        Write-Host "Added $BinDir to User PATH permanently." -ForegroundColor Green
    } else {
        Write-Host "$BinDir is already in User PATH." -ForegroundColor Green
    }

    # 7. Print Success Message
    Write-Host ""
    Write-Host "Radhe AI v0.4.0 installed successfully!" -ForegroundColor Green
    Write-Host "Restart your terminal, then try:" -ForegroundColor Green
    Write-Host "radhe --code `"hello world in c`"" -ForegroundColor Green
    Write-Host "radhe --explain `"binary search`"" -ForegroundColor Green
    Write-Host "radhe doctor" -ForegroundColor Green
}
catch {
    Write-Error "Installation failed: $_"
    exit 1
}
