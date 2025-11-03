# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

CCE (Claude Config Environment) is a Rust CLI tool for managing multiple Claude API service providers. It allows users to switch between different API endpoints and tokens via environment variables.

## Build and Development Commands

### Building the Project
```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# The binary will be in target/debug/cce or target/release/cce
```

### Running the Project
```bash
# Run directly with cargo
cargo run -- <command>

# Examples:
cargo run -- list
cargo run -- add test-provider https://api.example.com test-token
cargo run -- use test-provider
cargo run -- tui  # Launch interactive TUI
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test <test_name>
```

### Code Quality
```bash
# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# Lint with all warnings
cargo clippy -- -D warnings
```

### Installing Locally
```bash
# Install from source to ~/.cargo/bin
cargo install --path .
```

## Architecture

### Module Structure

The codebase is organized into 5 main modules:

1. **main.rs** - Entry point that orchestrates command parsing and execution
2. **cli.rs** - Command-line interface definitions using Clap
3. **config.rs** - Configuration management and persistence
4. **provider.rs** - Core business logic for provider operations
5. **tui.rs** - Interactive Text User Interface using ratatui and crossterm

### Key Design Patterns

**Configuration Storage**: Uses TOML format stored at `~/.cce/config.toml`. The Config struct manages:
- A HashMap of provider configurations
- Current provider selection
- Serialization/deserialization with serde

**Shell Integration Model**: CCE supports two execution modes:
1. **Normal mode**: Direct execution with user-friendly output
2. **Shell integration mode** (`CCE_SHELL_INTEGRATION=1`): Outputs shell commands (export/unset) to stdout for `eval` consumption

The shell integration works by wrapping the `cce` command in a shell function (see `output_shellenv()` in provider.rs:417-456) that:
- Intercepts `use` and `clear` commands
- Executes the binary with `CCE_SHELL_INTEGRATION=1`
- Evaluates the output to modify the parent shell's environment

**Command Aliases**: The CLI supports aliases (e.g., `ls` for `list`, `del` for `delete`) defined via Clap's `#[command(alias = "...")]` attribute.

**TUI Architecture**: The interactive TUI provides a full-screen terminal interface using:
- **ratatui** - Terminal UI framework for rendering widgets and layouts
- **crossterm** - Cross-platform terminal manipulation (keyboard input, raw mode, alternate screen)

The TUI supports three input modes:
1. **Normal mode** - Navigate providers, use/clear/delete operations
2. **AddProvider mode** - Multi-field form for adding new providers (name, URL, token, model)
3. **DeleteConfirm mode** - Confirmation dialog overlay before deleting

Key TUI features:
- Arrow keys / j/k for navigation
- Enter or 'u' to activate a provider
- 'a' to add a new provider
- 'd' to delete with confirmation
- 'c' to clear the current provider
- Tab/Shift-Tab to navigate between form fields
- Real-time status messages and error handling

### Environment Variables

CCE manages these environment variables:
- `ANTHROPIC_AUTH_TOKEN` - API authentication token
- `ANTHROPIC_BASE_URL` - API base URL
- `ANTHROPIC_MODEL` - Model name (optional, set if provider has model configured)
- `ANTHROPIC_DEFAULT_OPUS_MODEL` - Default Opus model (optional)
- `ANTHROPIC_DEFAULT_SONNET_MODEL` - Default Sonnet model (optional)
- `ANTHROPIC_DEFAULT_HAIKU_MODEL` - Default Haiku model (optional)

Control variable:
- `CCE_SHELL_INTEGRATION` - Set to "1" to enable shell integration mode

### Data Flow

**Adding a provider**:
1. CLI parses `add` command with name, api_url, token, optional model (cli.rs:23-33)
2. ProviderManager::add_provider creates Provider struct (provider.rs:50-74)
3. Config::add_provider inserts into HashMap (config.rs:55-69)
4. Config::save serializes to ~/.cce/config.toml (config.rs:38-53)

**Using a provider**:
1. CLI parses `use` command (cli.rs:48-51)
2. ProviderManager::use_provider checks shell integration mode (provider.rs:97-140)
3. Config::set_current_provider updates current selection (config.rs:80-87)
4. Config is saved to disk
5. If shell mode: emit export commands to stdout (provider.rs:282-292)
6. If normal mode: set env vars in process and print confirmation

**Shell integration installation**:
1. Detects user's shell from $SHELL environment variable (provider.rs:331)
2. Determines appropriate config file (~/.zshrc, ~/.bashrc, etc.)
3. Checks if integration already exists to avoid duplicates (provider.rs:354-367)
4. Appends `eval "$(cce shellenv)"` to shell config (provider.rs:389-401)
5. The shellenv command outputs a wrapper function that intercepts cce use/clear

## Cross-Platform Considerations

The project supports Linux, macOS (Intel and Apple Silicon), and Windows:
- Shell integration varies by shell type (bash, zsh, fish)
- macOS uses ~/.bash_profile instead of ~/.bashrc for bash
- Windows uses PowerShell with a separate install.ps1 script
- Path expansion handles tilde (~) differently across platforms

## Release Process

Releases are automated via GitHub Actions (see .github/workflows/release.yml):
- Triggered by pushing a tag matching `v*` pattern
- Builds for multiple targets: linux-x86_64, macos-x86_64, macos-aarch64, windows-x86_64
- Creates compressed archives (.tar.gz for Unix, .zip for Windows)
- Uploads to GitHub releases with auto-generated release notes

To create a release:
```bash
git tag v0.x.x
git push origin v0.x.x
```
