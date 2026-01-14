#!/bin/bash

# Update script for Core System Monitor
# This script rebuilds and reinstalls core from the current source code

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=== Core System Monitor Update ==="
echo ""

# Check if Rust toolchain is configured
if ! rustc --version &> /dev/null; then
    echo "Rust toolchain not configured. Setting up default stable toolchain..."
    rustup default stable
fi

# Clean previous build for fresh compilation
echo "Cleaning previous build..."
cargo clean

echo "Building Core System Monitor..."
cargo build --release

if [ ! -f "target/release/core" ]; then
    echo "Error: Build failed or binary not found"
    exit 1
fi

echo ""
echo "Build successful!"
echo ""

# Kill running core processes if any
if pgrep -x "core" > /dev/null; then
    echo "Found running core processes. Stopping them..."
    pkill -x core || killall core 2>/dev/null || true
    sleep 1
    # Force kill if still running
    if pgrep -x "core" > /dev/null; then
        echo "Force stopping remaining processes..."
        pkill -9 -x core || killall -9 core 2>/dev/null || true
        sleep 0.5
    fi
fi

# Detect where core is currently installed
INSTALLED_PATH=""
if [ -f "/usr/local/bin/core" ]; then
    INSTALLED_PATH="/usr/local/bin/core"
    INSTALL_DIR="/usr/local/bin"
    USE_SUDO=true
elif [ -f "$HOME/.local/bin/core" ]; then
    INSTALLED_PATH="$HOME/.local/bin/core"
    INSTALL_DIR="$HOME/.local/bin"
    USE_SUDO=false
elif [ -f "$HOME/.cargo/bin/core" ]; then
    INSTALLED_PATH="$HOME/.cargo/bin/core"
    INSTALL_DIR="$HOME/.cargo/bin"
    USE_SUDO=false
else
    echo "Warning: Could not find existing core installation."
    echo "Falling back to standard installation method..."
    USE_SUDO=true
    INSTALL_DIR="/usr/local/bin"
fi

# Install the updated binary
if [ "$USE_SUDO" = true ] && command -v sudo &> /dev/null; then
    echo "Installing updated core to $INSTALL_DIR (requires sudo)..."
    sudo cp target/release/core "$INSTALL_DIR/core"
    sudo chmod +x "$INSTALL_DIR/core"
    echo "Core updated successfully in $INSTALL_DIR"
elif [ "$USE_SUDO" = false ]; then
    echo "Installing updated core to $INSTALL_DIR..."
    mkdir -p "$INSTALL_DIR"
    cp target/release/core "$INSTALL_DIR/core"
    chmod +x "$INSTALL_DIR/core"
    echo "Core updated successfully in $INSTALL_DIR"
else
    # Fallback: try to install to user's local bin
    LOCAL_BIN="$HOME/.local/bin"
    mkdir -p "$LOCAL_BIN"
    echo "Installing updated core to $LOCAL_BIN..."
    cp target/release/core "$LOCAL_BIN/core"
    chmod +x "$LOCAL_BIN/core"
    echo "Core updated successfully in $LOCAL_BIN"
    
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
echo "You can now run 'core' with the updated version."
