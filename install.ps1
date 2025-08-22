# CCE Installation Script for Windows PowerShell

param(
    [string]$InstallPath = "$env:USERPROFILE\.cargo\bin"
)

$ErrorActionPreference = "Stop"

Write-Host "🧙 Installing CCE (Claude Config Environment)..." -ForegroundColor Blue

# Check if Rust/Cargo is installed
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Error: Rust/Cargo not found" -ForegroundColor Red
    Write-Host "Please install Rust first: https://rustup.rs/" -ForegroundColor Yellow
    exit 1
}

Write-Host "✅ Rust environment detected" -ForegroundColor Green

# Build the project
Write-Host "🔨 Building project..." -ForegroundColor Yellow
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Build completed" -ForegroundColor Green

# Install the binary
Write-Host "📦 Installing to system..." -ForegroundColor Yellow
cargo install --path .

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Installation failed" -ForegroundColor Red
    exit 1
}

Write-Host "✅ CCE binary installed successfully" -ForegroundColor Green

# Check if CCE is available
Write-Host "🧪 Verifying installation..." -ForegroundColor Yellow

if (Get-Command cce -ErrorAction SilentlyContinue) {
    Write-Host "✅ CCE installed successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "📖 Usage:" -ForegroundColor Blue
    Write-Host "  cce list                     - List all service providers"
    Write-Host "  cce add <name> <url> <token> - Add a service provider"
    Write-Host "  cce delete <name>            - Delete a service provider"  
    Write-Host "  cce use <name>               - Use specified service provider"
    Write-Host "  cce check                    - Check environment variable status"
    Write-Host "  cce --help                   - Show detailed help"
    Write-Host ""
    Write-Host "🔧 Shell Integration Setup:" -ForegroundColor Cyan
    Write-Host "For PowerShell, add this to your `$PROFILE:" -ForegroundColor Yellow
    Write-Host '  function cce { Invoke-Expression "$(cce.exe use $args --eval 2>$null)" }' -ForegroundColor White
    Write-Host ""
    Write-Host "💡 Start using: 'cce list' to manage your Claude configurations!" -ForegroundColor Green
} else {
    Write-Host "⚠️  Installation may not be complete" -ForegroundColor Yellow
    Write-Host "Please ensure $InstallPath is in your PATH" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "🎉 Installation completed!" -ForegroundColor Green