# CCE - Claude Config Environment

🧙 A Claude environment variable switching tool written in Rust, allowing you to easily manage multiple Claude API service providers.

## ✨ Features

- 🔄 **Easy Switching** - Quickly switch between different Claude API service providers
- 📝 **Configuration Management** - Securely store and manage multiple service provider configurations
- 🎨 **User-Friendly Interface** - Colorful output and intuitive command-line interface
- ⚡ **High Performance** - Built with Rust, fast startup and efficient execution
- 🔒 **Secure & Reliable** - Local configuration storage to protect your API keys

## 🚀 Quick Start

### Installation

#### Option 1: One-Click Install (Recommended)
```bash
# Install with curl (supports Linux, macOS Intel & Apple Silicon)
curl -sSL https://raw.githubusercontent.com/zhaopengme/cce/master/install.sh | bash
```

#### Option 2: Download from Releases
```bash
# Visit https://github.com/zhaopengme/cce/releases
# Download the appropriate binary for your platform:
# - cce-linux-x86_64.tar.gz (Linux)
# - cce-macos-x86_64.tar.gz (macOS Intel)
# - cce-macos-aarch64.tar.gz (macOS Apple Silicon)  
# - cce-windows-x86_64.exe.zip (Windows)

# Extract and install
tar -xzf cce-*.tar.gz
chmod +x cce
mv cce ~/.local/bin/  # Make sure ~/.local/bin is in your PATH
```

#### Option 3: Build from Source
```bash
# Clone the project
git clone https://github.com/zhaopengme/cce.git
cd cce

# Build the project
cargo build --release

# Install (optional)
cargo install --path .
```

#### Option 4: Windows PowerShell
```powershell
# Download and run the PowerShell installer
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/zhaopengme/cce/master/install.ps1" -OutFile "install.ps1"
.\install.ps1
```

### Setup Shell Integration

The key feature of CCE is the ability to make `cce use` commands take effect immediately in your current terminal. This requires a one-time setup:

```bash
# Add this line to your shell configuration file (~/.zshrc, ~/.bashrc, etc.)
eval "$(cce shellenv)"

# Then reload your shell
source ~/.zshrc  # or source ~/.bashrc
```

**Note**: If you used the one-click install script, it will automatically guide you through this setup process with detailed instructions for your specific shell.

### Basic Usage

#### 1. List all service providers
```bash
cce list
```

#### 2. Add a service provider
```bash
cce add <name> <API_URL> <API_TOKEN>

# Examples
cce add anthropic https://api.anthropic.com sk-ant-api03-xxxx
cce add custom https://custom-claude-api.com custom-token-123
```

#### 3. Delete a service provider
```bash
cce delete <name>

# Examples
cce delete anthropic
```

#### 4. Switch service provider ⭐

**With shell integration (recommended)**:
```bash
cce use anthropic
# ⚡ Switched to service provider 'anthropic'
# ✅ Environment variables are now active in current terminal
```

**Without shell integration**:
```bash
eval "$(cce use anthropic --eval)"
```

#### 5. Check environment variable status
```bash
cce check
```

## 📋 Command Reference

### `cce shellenv`
Outputs shell integration function. Add `eval "$(cce shellenv)"` to your shell configuration file for the best experience.

### `cce list`
Display all configured service providers with their status:
- Provider name
- API URL
- Masked token preview
- Current active status

### `cce add <name> <api_url> <token>`
Add a new service provider:
- `name`: Custom provider name
- `api_url`: Claude API endpoint URL
- `token`: API access token

If the provider already exists, it will be overwritten.

### `cce delete <name>`
Remove the specified service provider. No confirmation required.

### `cce use <name> [--eval]`
Switch to the specified service provider:

**Normal mode** (`cce use <name>`):
- 📋 Display complete switching information
- 💡 Provide environment variable commands for copying

**Eval mode** (`cce use <name> --eval`):
- ⚡ Output only environment variable commands
- 🔧 Perfect for use with `eval` command
- 💻 Ideal for scripts and automation

### `cce check`
Verify current environment variable status:
- Display current environment variables
- Compare CCE configuration with actual environment variables
- Provide suggestions when there are mismatches

## 🔧 Configuration

Configuration file is stored at `~/.cce/config.toml`:

```toml
current_provider = "anthropic"

[providers.anthropic]
name = "anthropic"
api_url = "https://api.anthropic.com"
token = "sk-ant-api03-your-token-here"

[providers.custom]
name = "custom"
api_url = "https://custom-claude-api.com"
token = "custom-token-123"
```

## 🌍 Environment Variables

After using `cce use` command, the following environment variables are automatically set:
- `ANTHROPIC_AUTH_TOKEN`: API authentication token
- `ANTHROPIC_BASE_URL`: API base URL

## 💡 Usage Tips

### 1. Quick Switching
```bash
# Add common providers
cce add prod https://api.anthropic.com sk-ant-prod-xxx
cce add dev https://dev-api.example.com dev-token-xxx

# Quick switch (with shell integration)
cce use prod
cce use dev
```

### 2. Script Usage
```bash
#!/bin/bash
eval "$(cce use anthropic --eval)"
# Environment variables are now set and ready to use
curl -H "Authorization: Bearer $ANTHROPIC_AUTH_TOKEN" "$ANTHROPIC_BASE_URL/v1/messages"
```

### 3. Verify Configuration
```bash
cce check                    # Check current status
echo $ANTHROPIC_AUTH_TOKEN   # Verify token
echo $ANTHROPIC_BASE_URL     # Verify URL
```

### 4. Backup Configuration
```bash
cp ~/.cce/config.toml ~/.cce/config.toml.backup
```

## 📥 Platform Support

| Platform | Architecture | Binary | Status |
|----------|-------------|---------|---------|
| Linux | x86_64 | `cce-linux-x86_64.tar.gz` | ✅ |
| macOS | Intel (x86_64) | `cce-macos-x86_64.tar.gz` | ✅ |
| macOS | Apple Silicon (ARM64) | `cce-macos-aarch64.tar.gz` | ✅ |
| Windows | x86_64 | `cce-windows-x86_64.exe.zip` | ✅ |

All releases include automated CI/CD testing across multiple platforms to ensure reliability.

## 🐛 Troubleshooting

### Installation Issues
If the one-click install fails:
```bash
# Check if curl is available
curl --version

# Check if your platform is supported
uname -s && uname -m

# Try manual installation from releases page
# https://github.com/zhaopengme/cce/releases
```

### Shell Integration Not Working
Make sure you've added `eval "$(cce shellenv)"` to your shell configuration file and reloaded it:
```bash
echo 'eval "$(cce shellenv)"' >> ~/.zshrc
source ~/.zshrc
```

### Environment Variables Not Set
Run `cce check` to diagnose the issue and follow the suggestions.

### PATH Issues
If `cce` command is not found after installation:
```bash
# Add ~/.local/bin to your PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

# Verify installation
which cce
cce --version
```

### Configuration File Corrupted
If the config file is corrupted, you can delete and recreate it:
```bash
rm -rf ~/.cce
```

## 🤝 Contributing

Issues and Pull Requests are welcome!

## 📞 Contact

- Author: [@zhaopengme](https://x.com/zhaopengme)
- Twitter: https://x.com/zhaopengme

## 📄 License

MIT License
