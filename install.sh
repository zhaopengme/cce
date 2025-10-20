#!/bin/bash
# CCE (Claude Config Environment) Installation Script
# Supports Linux, macOS (Intel & Apple Silicon)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
REPO="zhaopengme/cce"
BINARY_NAME="cce"
INSTALL_DIR="$HOME/.local/bin"

# Function to print colored output
print_status() {
    echo -e "${BLUE}🧙 $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Detect OS and architecture
detect_platform() {
    local os
    local arch
    local platform
    
    case "$(uname -s)" in
        Linux*)     os="linux" ;;
        Darwin*)    os="macos" ;;
        *)          
            print_error "Unsupported operating system: $(uname -s)"
            exit 1
            ;;
    esac
    
    case "$(uname -m)" in
        x86_64)     arch="x86_64" ;;
        arm64|aarch64) 
            if [[ "$os" == "macos" ]]; then
                arch="aarch64"
            else
                arch="x86_64"  # Fallback to x86_64 for Linux ARM
            fi
            ;;
        *)          
            print_warning "Unsupported architecture $(uname -m), falling back to x86_64"
            arch="x86_64"
            ;;
    esac
    
    if [[ "$os" == "linux" ]]; then
        platform="${BINARY_NAME}-linux-${arch}"
    elif [[ "$os" == "macos" ]]; then
        platform="${BINARY_NAME}-macos-${arch}"
    fi
    
    echo "$platform"
}

# Get latest release version
get_latest_version() {
    local version
    version=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')
    
    if [[ -z "$version" ]]; then
        print_error "Failed to get latest version from GitHub"
        exit 1
    fi
    
    echo "$version"
}

# Download and install binary
install_cce() {
    local platform="$1"
    local version="$2"
    local download_url="https://github.com/${REPO}/releases/download/${version}/${platform}.tar.gz"
    local temp_dir
    
    temp_dir=$(mktemp -d)
    
    print_status "Downloading CCE ${version} for ${platform}..."
    
    if ! curl -L -o "${temp_dir}/${platform}.tar.gz" "$download_url"; then
        print_error "Failed to download CCE"
        rm -rf "$temp_dir"
        exit 1
    fi
    
    print_status "Extracting binary..."
    if ! tar -xzf "${temp_dir}/${platform}.tar.gz" -C "$temp_dir"; then
        print_error "Failed to extract binary"
        rm -rf "$temp_dir"
        exit 1
    fi
    
    # Create installation directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"
    
    print_status "Installing to ${INSTALL_DIR}..."
    if ! cp "${temp_dir}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"; then
        print_error "Failed to install binary"
        rm -rf "$temp_dir"
        exit 1
    fi
    
    # Make binary executable
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
    
    # Cleanup
    rm -rf "$temp_dir"
    
    print_success "CCE installed successfully to ${INSTALL_DIR}/${BINARY_NAME}"
}

# Check if binary is in PATH
check_path() {
    if [[ ":$PATH:" == *":$INSTALL_DIR:"* ]]; then
        print_success "Installation directory is already in PATH"
        return 0
    else
        print_warning "Installation directory is not in PATH"
        return 1
    fi
}

# Add to PATH instructions
show_path_instructions() {
    echo ""
    print_status "To use CCE, you need to add it to your PATH:"
    echo ""
    echo -e "${CYAN}# For Bash users:${NC}"
    echo "echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc"
    echo "source ~/.bashrc"
    echo ""
    echo -e "${CYAN}# For Zsh users:${NC}"
    echo "echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.zshrc"
    echo "source ~/.zshrc"
    echo ""
    echo -e "${CYAN}# For Fish users:${NC}"
    echo "fish_add_path ~/.local/bin"
    echo ""
}

# Show usage instructions
show_usage() {
    local profile
    if ! profile=$(detect_shell_profile 2>/dev/null); then
        profile=""
    fi

    echo ""
    print_success "CCE installation completed!"
    echo ""
    print_status "Usage:"
    echo "  cce list                     - List all service providers"
    echo "  cce add <name> <url> <token> - Add a service provider"
    echo "  cce delete <name>            - Delete a service provider"
    echo "  cce use <name>               - Use specified service provider"
    echo "  cce check                    - Check environment variable status"
    echo "  cce --help                   - Show detailed help"
    echo ""
    print_status "Shell integration:"
    echo -e "${YELLOW}Installer has configured automatic provider loading for your shell.${NC}"
    echo "Open a new terminal to apply the changes."
    if [[ -n "$profile" ]]; then
        echo "To apply immediately, run:"
        echo "  source \"$profile\""
    fi
    echo ""
    print_status "Quick test:"
    echo "cce --version"
    echo ""
}

detect_shell_profile() {
    local shell_name profile
    shell_name=$(basename "${SHELL:-}")

    if [[ -z "$shell_name" ]]; then
        if [[ "$(uname -s)" == "Darwin" ]]; then
            shell_name="zsh"
        else
            shell_name="bash"
        fi
    fi

    case "$shell_name" in
        zsh) profile="$HOME/.zshrc" ;;
        bash)
            if [[ "$(uname -s)" == "Darwin" ]]; then
                profile="$HOME/.bash_profile"
            else
                profile="$HOME/.bashrc"
            fi
            ;;
        *)
            return 1
    esac

    echo "$profile"
    return 0
}

remove_existing_integration() {
    local file="$1"
    [[ -f "$file" ]] || return 0

    if grep -Fq ">>> CCE Shell Integration >>>" "$file"; then
        if sed --version >/dev/null 2>&1; then
            sed -i '/# >>> CCE Shell Integration >>>/,/# <<< CCE Shell Integration <<</d' "$file"
        else
            sed -i '' '/# >>> CCE Shell Integration >>>/,/# <<< CCE Shell Integration <<</d' "$file"
        fi
    fi
}

append_shell_integration() {
    local profile="$1"
    mkdir -p "$(dirname "$profile")"
    touch "$profile"

    cat <<'EOF' >>"$profile"
# >>> CCE Shell Integration >>>
if command -v cce >/dev/null 2>&1; then
  _cce_binary="$(command -v cce)"

  cce() {
    if [[ "$1" == "use" && -n "$2" ]]; then
      local _output
      _output=$(CCE_SHELL_INTEGRATION=1 "$_cce_binary" use "$2" 2>/dev/null)
      if [[ $? -eq 0 && -n "$_output" ]]; then
        eval "$_output"
        echo "⚡ Switched to service provider '$2'"
        echo "✅ Environment variables are now active in current terminal"
        return 0
      fi
    elif [[ "$1" == "clear" ]]; then
      local _output
      _output=$(CCE_SHELL_INTEGRATION=1 "$_cce_binary" clear 2>/dev/null)
      if [[ $? -eq 0 && -n "$_output" ]]; then
        eval "$_output"
        echo "🧹 Cleared service provider configuration"
        echo "✅ Environment variables are now unset in current terminal"
        return 0
      fi
    fi

    "$_cce_binary" "$@"
  }

  __cce_apply_current_provider() {
    local _cfg="$HOME/.cce/config.toml"
    if [[ -f "$_cfg" ]]; then
      local _provider
      _provider=$(awk -F'"' '/^current_provider/ {print $2; exit}' "$_cfg")
      if [[ -n "$_provider" ]]; then
        local _boot_output
        _boot_output=$(CCE_SHELL_INTEGRATION=1 "$_cce_binary" use "$_provider" 2>/dev/null)
        if [[ $? -eq 0 && -n "$_boot_output" ]]; then
          eval "$_boot_output"
        fi
      fi
    fi
  }

  __cce_apply_current_provider
  unset -f __cce_apply_current_provider
fi
# <<< CCE Shell Integration <<<
EOF
}

configure_shell_integration() {
    local profile status
    profile=$(detect_shell_profile 2>/dev/null)
    status=$?
    if [[ $status -ne 0 || -z "$profile" ]]; then
        print_warning "Automatic shell integration is supported for bash and zsh shells. Please configure other shells manually."
        return
    fi

    print_status "Configuring shell integration at ${profile}..."

    remove_existing_integration "$profile"
    append_shell_integration "$profile"

    print_success "Shell integration script added to ${profile}"
}

# Main installation process
main() {
    print_status "Installing CCE (Claude Config Environment)..."
    echo ""
    
    # Detect platform
    local platform
    platform=$(detect_platform)
    print_status "Detected platform: $platform"
    
    # Get latest version
    local version
    version=$(get_latest_version)
    print_status "Latest version: $version"
    
    # Install
    install_cce "$platform" "$version"
    
    # Configure shell integration
    configure_shell_integration
    
    # Check PATH and show instructions
    if ! check_path; then
        show_path_instructions
    fi
    
    # Show usage
    show_usage
}

# Run main function
main "$@"
