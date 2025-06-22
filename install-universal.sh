#!/bin/bash

# Universal Installer for Password Generator CLI
# This script automatically detects the platform and installs the appropriate binary

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Detect platform
detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)
    
    case "$os" in
        "darwin")
            if [[ "$arch" == "x86_64" ]]; then
                echo "x86_64-apple-darwin"
            elif [[ "$arch" == "arm64" ]]; then
                echo "aarch64-apple-darwin"
            else
                print_error "Unsupported architecture: $arch"
                exit 1
            fi
            ;;
        "linux")
            if [[ "$arch" == "x86_64" ]]; then
                echo "x86_64-unknown-linux-gnu"
            elif [[ "$arch" == "aarch64" ]]; then
                echo "aarch64-unknown-linux-gnu"
            else
                print_error "Unsupported architecture: $arch"
                exit 1
            fi
            ;;
        "msys"*|"cygwin"*|"mingw"*)
            echo "x86_64-pc-windows-msvc"
            ;;
        *)
            print_error "Unsupported operating system: $os"
            exit 1
            ;;
    esac
}

# Get latest version from GitHub
get_latest_version() {
    local version=$(curl -s https://api.github.com/repos/morshedulmunna/passgen/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    echo "$version"
}

# Download and install
main() {
    echo "🔐 Password Generator CLI - Universal Installer"
    echo "=============================================="
    
    local platform=$(detect_platform)
    local version=$(get_latest_version)
    local download_url="https://github.com/morshedulmunna/passgen/releases/download/${version}/passgen-${platform}.tar.gz"
    
    print_status "Detected platform: $platform"
    print_status "Latest version: $version"
    print_status "Download URL: $download_url"
    
    # Create temporary directory
    local temp_dir=$(mktemp -d)
    cd "$temp_dir"
    
    print_status "Downloading binary..."
    if curl -L -o "passgen.tar.gz" "$download_url"; then
        print_success "Download completed!"
    else
        print_error "Download failed!"
        exit 1
    fi
    
    print_status "Extracting binary..."
    tar -xzf passgen.tar.gz
    
    # Determine install location
    local install_dir="$HOME/.local/bin"
    if [[ "$platform" == *"windows"* ]]; then
        install_dir="$HOME/AppData/Local/Microsoft/WinGet/Packages"
    fi
    
    # Create install directory if it doesn't exist
    mkdir -p "$install_dir"
    
    print_status "Installing to $install_dir..."
    if [[ "$platform" == *"windows"* ]]; then
        cp passgen.exe "$install_dir/"
        print_success "Installed as passgen.exe"
    else
        cp passgen "$install_dir/"
        chmod +x "$install_dir/passgen"
        print_success "Installed as passgen"
    fi
    
    # Add to PATH if not already there
    if [[ ":$PATH:" != *":$install_dir:"* ]]; then
        print_warning "Adding $install_dir to PATH..."
        if [[ "$platform" == *"darwin"* ]]; then
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
            print_status "Added to ~/.zshrc. Please restart your terminal or run: source ~/.zshrc"
        elif [[ "$platform" == *"linux"* ]]; then
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
            print_status "Added to ~/.bashrc. Please restart your terminal or run: source ~/.bashrc"
        fi
    fi
    
    # Cleanup
    cd - > /dev/null
    rm -rf "$temp_dir"
    
    print_success "Installation completed!"
    echo ""
    echo "🎉 Password Generator CLI is now installed!"
    echo ""
    echo "Try these commands:"
    echo "  passgen --help"
    echo "  passgen generate --length 16 --copy"
    echo "  passgen passphrase --words 4"
    echo ""
    
    if [[ "$platform" == *"darwin"* ]]; then
        print_warning "Note: For clipboard functionality, ensure your terminal has accessibility permissions:"
        echo "   System Preferences > Security & Privacy > Privacy > Accessibility > Add Terminal"
    fi
}

main "$@" 