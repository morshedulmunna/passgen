#!/bin/bash

# Quick Install Script for Password Generator CLI
# This script provides a simple way to install the tool

set -e

echo "🔐 Password Generator CLI - Quick Install"
echo "========================================="

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

# Check if we're on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    print_warning "This tool is optimized for macOS. Some features may not work on other systems."
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_error "Rust is not installed. Please install Rust first:"
    echo "   Visit: https://rustup.rs/"
    echo "   Or run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

print_status "Rust is installed. Building the project..."

# Build the project
if cargo build --release; then
    print_success "Build completed successfully!"
else
    print_error "Build failed!"
    exit 1
fi

print_status "Installing globally..."

# Install globally
if cargo install --path .; then
    print_success "Installation completed!"
    echo ""
    echo "🎉 Password Generator CLI is now installed!"
    echo ""
    echo "Try these commands:"
    echo "  passgen --help"
    echo "  passgen generate --length 16 --copy"
    echo "  passgen passphrase --words 4"
    echo ""
    print_warning "Note: For clipboard functionality, ensure your terminal has accessibility permissions:"
    echo "   System Preferences > Security & Privacy > Privacy > Accessibility > Add Terminal"
else
    print_error "Installation failed!"
    exit 1
fi 