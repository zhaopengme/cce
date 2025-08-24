# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

CCE (Claude Config Environment) is a Rust-based CLI tool for managing and switching between different Claude API service providers. It allows users to store multiple API configurations and quickly switch between them by setting appropriate environment variables.

## Development Commands

### Building and Testing
```bash
# Build the project
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Install locally
cargo install --path .

# Run the application
cargo run
```

### Common Development Tasks
```bash
# Run with specific arguments
cargo run -- list
cargo run -- add my-provider https://api.example.com token123
cargo run -- use my-provider
cargo run -- clear
cargo run -- install
```

## Architecture

### Core Components

1. **CLI Module (`src/cli.rs`)**: Command-line interface parsing using `clap`
   - Defines all available commands and their arguments
   - Uses clap's derive macros for automatic parsing

2. **Config Module (`src/config.rs`)**: Configuration management
   - Handles TOML config file operations
   - Stores provider configurations in `~/.cce/config.toml`
   - Provides provider management methods (add, remove, set current)

3. **Provider Module (`src/provider.rs`)**: Business logic implementation
   - Manages provider operations (list, add, delete, use, check, clear, install)
   - Handles environment variable management
   - Implements shell integration functionality

### Key Design Patterns

- **Command Pattern**: Each CLI command is handled by a specific method in `ProviderManager`
- **Configuration Persistence**: Uses TOML format for human-readable config storage
- **Shell Integration**: Provides `shellenv` command for immediate environment variable effects
- **Environment Variable Management**: Sets `ANTHROPIC_AUTH_TOKEN` and `ANTHROPIC_BASE_URL`

### Data Flow

1. `main.rs` parses CLI arguments using `cli.rs`
2. Loads configuration using `config.rs`
3. Routes commands to appropriate `ProviderManager` methods
4. `ProviderManager` executes business logic and updates configuration
5. Changes are persisted to TOML file

### Configuration Structure

```toml
current_provider = "anthropic"

[providers.anthropic]
name = "anthropic"
api_url = "https://api.anthropic.com"
token = "sk-ant-api03-your-token-here"
```

## Important Implementation Details

### Shell Integration
- The `shellenv` command outputs a shell function that wraps `cce use` and `cce clear` commands
- This allows immediate environment variable changes in the current terminal
- Users should add `eval "$(cce shellenv)"` to their shell profile

### Environment Variables
- `ANTHROPIC_AUTH_TOKEN`: API authentication token
- `ANTHROPIC_BASE_URL`: API base URL
- These are set by `cce use` and checked by `cce check`
- These can be unset by `cce clear` to switch back to official Claude client

### Error Handling
- Uses `anyhow` for comprehensive error handling
- Provides user-friendly error messages with colored output
- Uses `context!` for error context preservation

### CLI Features
- Colored output for better user experience
- Both normal and eval modes for `use` and `clear` commands
- Automatic shell integration installation with `install` command
- Automatic config directory creation
- No confirmation prompts for streamlined workflow
- `clear` command allows users to easily switch back to official Claude client

## Dependencies

- **clap**: Command-line argument parsing
- **serde/toml**: Configuration serialization/deserialization
- **dirs**: Cross-platform directory paths
- **anyhow**: Error handling
- **colored**: Colored terminal output

## Testing Strategy

The project currently lacks comprehensive unit tests. When adding new features, ensure to:
- Test provider management operations
- Verify configuration persistence
- Test shell integration functionality
- Validate environment variable handling