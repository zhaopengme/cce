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
    Write-Host "🔧 Configuring shell integration..." -ForegroundColor Cyan

    function Get-CceIntegrationBlock {
@'
# >>> CCE Shell Integration >>>
if (Get-Command cce -ErrorAction SilentlyContinue) {
    $script:CceBinary = (Get-Command cce).Source

    function Apply-CceEnvironment {
        param(
            [string[]]$Lines
        )

        $expanded = @()
        foreach ($entry in $Lines) {
            if ($null -eq $entry) { continue }
            $expanded += ($entry -split "`r?`n")
        }

        foreach ($line in $expanded) {
            if ([string]::IsNullOrWhiteSpace($line)) { continue }
            $trimmed = $line.Trim()
            if ($trimmed -match '^export\s+([A-Z0-9_]+)="([^"]*)"$') {
                $name = $matches[1]
                $value = $matches[2]
                [Environment]::SetEnvironmentVariable($name, $value, 'Process')
            } elseif ($trimmed -match '^unset\s+([A-Z0-9_]+)$') {
                $name = $matches[1]
                [Environment]::SetEnvironmentVariable($name, $null, 'Process')
            }
        }
    }

    function Invoke-CceBinary {
        param(
            [string[]]$Arguments
        )

        $env:CCE_SHELL_INTEGRATION = '1'
        $output = & $script:CceBinary @Arguments 2>$null
        $status = $LASTEXITCODE
        Remove-Item Env:CCE_SHELL_INTEGRATION -ErrorAction SilentlyContinue
        return [PSCustomObject]@{
            Status = $status
            Output = $output
        }
    }

    function cce {
        param(
            [Parameter(ValueFromRemainingArguments = $true)]
            [string[]]$Args
        )

        if ($Args.Length -ge 2 -and $Args[0] -eq 'use') {
            $result = Invoke-CceBinary -Arguments $Args
            if ($result.Status -eq 0 -and $result.Output) {
                Apply-CceEnvironment -Lines $result.Output
                Write-Host "⚡ Switched to service provider '$($Args[1])'"
                Write-Host '✅ Environment variables are now active in current terminal'
                return
            }
        } elseif ($Args.Length -ge 1 -and $Args[0] -eq 'clear') {
            $result = Invoke-CceBinary -Arguments $Args
            if ($result.Status -eq 0 -and $result.Output) {
                Apply-CceEnvironment -Lines $result.Output
                Write-Host '🧹 Cleared service provider configuration'
                Write-Host '✅ Environment variables are now unset in current terminal'
                return
            }
        }

        & $script:CceBinary @Args
    }

    function Initialize-CceEnvironment {
        $configPath = Join-Path $env:USERPROFILE '.cce\config.toml'
        if (Test-Path $configPath) {
            $match = Select-String -Path $configPath -Pattern '^current_provider\s*=\s*"([^"]+)"' | Select-Object -First 1
            if ($match) {
                $provider = $match.Matches[0].Groups[1].Value
                if ($provider) {
                    $result = Invoke-CceBinary -Arguments @('use', $provider)
                    if ($result.Status -eq 0 -and $result.Output) {
                        Apply-CceEnvironment -Lines $result.Output
                    }
                }
            }
        }
    }

    Initialize-CceEnvironment
}
# <<< CCE Shell Integration <<<
'@
    }

    function Set-CceShellIntegration {
        param(
            [string]$ProfilePath
        )

        if (-not (Test-Path $ProfilePath)) {
            New-Item -ItemType File -Path $ProfilePath -Force | Out-Null
        }

        $existing = Get-Content -Path $ProfilePath -Raw -ErrorAction SilentlyContinue
        if ($existing) {
            $existing = [regex]::Replace(
                $existing,
                '# >>> CCE Shell Integration >>>.*?# <<< CCE Shell Integration <<<\s*',
                '',
                [System.Text.RegularExpressions.RegexOptions]::Singleline
            )
        } else {
            $existing = ''
        }

        $block = Get-CceIntegrationBlock
        if ($existing.Length -gt 0 -and -not $existing.EndsWith("`n")) {
            $existing += "`n"
        }
        $updated = $existing + $block + "`n"
        Set-Content -Path $ProfilePath -Value $updated -Encoding UTF8
        Write-Host "✅ Shell integration written to $ProfilePath" -ForegroundColor Green
    }

    Set-CceShellIntegration -ProfilePath $PROFILE
    Write-Host ""
    Write-Host "Restart PowerShell or run '. `$PROFILE' to apply the environment automatically." -ForegroundColor Yellow
    Write-Host ""
    Write-Host "💡 Start using: 'cce list' to manage your Claude configurations!" -ForegroundColor Green
} else {
    Write-Host "⚠️  Installation may not be complete" -ForegroundColor Yellow
    Write-Host "Please ensure $InstallPath is in your PATH" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "🎉 Installation completed!" -ForegroundColor Green
