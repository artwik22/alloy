#!/bin/bash
set -e

echo "Building Vitals system monitor..."
cargo build --release

echo "Installing to /usr/local/bin..."
sudo cp target/release/vitals /usr/local/bin/

echo "Installation complete!"
echo "Run 'vitals' to start the application."
