#!/bin/bash

# Password Generator CLI Tool Installation Script
# This script builds and installs the passgen tool globally

set -e

echo "🔐 Password Generator CLI Tool Installer"
echo "========================================"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust is not installed. Please install Rust first:"
    echo "   Visit: https://rustup.rs/"
    exit 1
fi

# Check if we're on macOS (for clipboard functionality)
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "⚠️  Warning: This tool is optimized for macOS. Clipboard functionality may not work on other systems."
fi

echo "📦 Building the project..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
else
    echo "❌ Build failed!"
    exit 1
fi

echo "🔧 Installing globally..."
cargo install --path .

if [ $? -eq 0 ]; then
    echo "✅ Installation successful!"
    echo ""
    echo "🎉 Password Generator CLI Tool is now installed!"
    echo ""
    echo "Usage examples:"
    echo "  passgen generate --length 16 --copy"
    echo "  passgen passphrase --words 4"
    echo "  passgen check \"mypassword\""
    echo "  passgen hash \"my-secret\" --algorithm sha256"
    echo ""
    echo "For more information, run: passgen --help"
else
    echo "❌ Installation failed!"
    exit 1
fi 