# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-10-18

### Added
- `cce add` now accepts an optional `--model` flag and exports both `ANTHROPIC_MODEL` and `ANTHROPIC_DEFAULT_HAIKU_MODEL` when selected.
- Introduced `cce clear` to unset the active provider and remove Claude-specific environment variables.
- Added `cce install` for one-command shell integration setup across bash, zsh, and fish, plus an enriched `cce shellenv` helper.

### Improved
- `cce use` highlights the currently active provider, skips redundant switches, and guides users to apply environment changes immediately.
- `cce check` now validates whether the live environment matches the stored configuration and suggests corrective actions.

## [0.1.0] - 2024-01-XX

### Added
- Initial release of CCE (Claude Config Environment)
- Service provider management (`add`, `delete`, `list`)
- Environment variable switching with `use` command
- Shell integration with `shellenv` command for immediate effect
- Configuration checking with `check` command
- Support for `ANTHROPIC_AUTH_TOKEN` and `ANTHROPIC_BASE_URL` environment variables
- Local TOML configuration storage in `~/.cce/config.toml`
- Colorful CLI output with user-friendly messages
- Built with Rust for high performance and reliability

### Features
- **Easy Switching**: Quickly switch between different Claude API service providers
- **Shell Integration**: `eval "$(cce shellenv)"` for immediate environment variable effects
- **Configuration Management**: Secure local storage of multiple service provider configurations
- **User-Friendly Interface**: Intuitive command-line interface with colored output
- **Cross-Platform**: Works on macOS, Linux, and Windows
- **No Confirmation Prompts**: Streamlined workflow without unnecessary interruptions

### Commands
- `cce list` - List all configured service providers
- `cce add <name> <api_url> <token>` - Add a new service provider
- `cce delete <name>` - Remove a service provider
- `cce use <name>` - Switch to a service provider (with shell integration)
- `CCE_SHELL_INTEGRATION=1 cce use <name>` - Emit environment variable exports for integration
- `CCE_SHELL_INTEGRATION=1 cce clear` - Emit unset commands for integration
- `cce check` - Verify current environment variable status
- `cce shellenv` - Output shell integration function
