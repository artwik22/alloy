#!/bin/bash

# Update script for Index
# This script rebuilds and reinstalls index from the current source code

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=== Index Update ==="
echo ""

# Check if Rust toolchain is configured
if ! rustc --version &> /dev/null; then
    echo "Rust toolchain not configured. Setting up default stable toolchain..."
    rustup default stable
fi

# Clean previous build for fresh compilation
echo "Cleaning previous build..."
cargo clean

echo "Building Index..."
cargo build --release

if [ ! -f "target/release/index" ]; then
    echo "Error: Build failed or binary not found"
    exit 1
fi

echo ""
echo "Build successful!"
echo ""

# Kill running index processes if any
if pgrep -x "index" > /dev/null; then
    echo "Found running index processes. Stopping them..."
    pkill -x index || killall index 2>/dev/null || true
    sleep 1
    # Force kill if still running
    if pgrep -x "index" > /dev/null; then
        echo "Force stopping remaining processes..."
        pkill -9 -x index || killall -9 index 2>/dev/null || true
        sleep 0.5
    fi
fi

# Detect where index is currently installed
INSTALLED_PATH=""
if [ -f "/usr/local/bin/index" ]; then
    INSTALLED_PATH="/usr/local/bin/index"
    INSTALL_DIR="/usr/local/bin"
    USE_SUDO=true
elif [ -f "$HOME/.local/bin/index" ]; then
    INSTALLED_PATH="$HOME/.local/bin/index"
    INSTALL_DIR="$HOME/.local/bin"
    USE_SUDO=false
elif [ -f "$HOME/.cargo/bin/index" ]; then
    INSTALLED_PATH="$HOME/.cargo/bin/index"
    INSTALL_DIR="$HOME/.cargo/bin"
    USE_SUDO=false
else
    echo "Warning: Could not find existing index installation."
    echo "Falling back to standard installation method..."
    USE_SUDO=true
    INSTALL_DIR="/usr/local/bin"
fi

# Install the updated binary
if [ "$USE_SUDO" = true ] && command -v sudo &> /dev/null; then
    echo "Installing updated index to $INSTALL_DIR (requires sudo)..."
    sudo cp target/release/index "$INSTALL_DIR/index"
    sudo chmod +x "$INSTALL_DIR/index"
    echo "Index updated successfully in $INSTALL_DIR"
elif [ "$USE_SUDO" = false ]; then
    echo "Installing updated index to $INSTALL_DIR..."
    mkdir -p "$INSTALL_DIR"
    cp target/release/index "$INSTALL_DIR/index"
    chmod +x "$INSTALL_DIR/index"
    echo "Index updated successfully in $INSTALL_DIR"
else
    # Fallback: try to install to user's local bin
    LOCAL_BIN="$HOME/.local/bin"
    mkdir -p "$LOCAL_BIN"
    echo "Installing updated index to $LOCAL_BIN..."
    cp target/release/index "$LOCAL_BIN/index"
    chmod +x "$LOCAL_BIN/index"
    echo "Index updated successfully in $LOCAL_BIN"
    
    # Check if ~/.local/bin is in PATH
    if [[ ":$PATH:" != *":$LOCAL_BIN:"* ]]; then
        echo ""
        echo "Note: $LOCAL_BIN is not in your PATH"
        echo "Add this line to your ~/.bashrc or ~/.zshrc:"
        echo "export PATH=\"\$HOME/.local/bin:\$PATH\""
    fi
fi

echo ""
echo "=== Update complete! ==="
echo "You can now run 'index' with the updated version."
