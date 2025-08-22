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
    print_status "Shell Integration Setup:"
    echo -e "${YELLOW}Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):${NC}"
    echo 'eval "$(cce shellenv)"'
    echo ""
    print_status "Quick test:"
    echo "cce --version"
    echo ""
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
    
    # Check PATH and show instructions
    if ! check_path; then
        show_path_instructions
    fi
    
    # Show usage
    show_usage
}

# Run main function
main "$@"