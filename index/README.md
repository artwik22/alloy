# Index

A lightweight file explorer written in Rust with GTK4.

![Index Screenshot](screenshot.png)

## Features

- Fast directory navigation
- Copy, Cut, Paste files and folders
- Delete files to trash
- Rename files and folders
- Create new files and folders
- Search/filter files in current directory
- Sidebar with quick access to common locations
- Minimalist design with no rounded corners

## Requirements

- Rust 1.70+
- GTK4 development libraries

### Installing GTK4 on Linux

**Arch Linux:**
```bash
sudo pacman -S gtk4
```

**Ubuntu/Debian:**
```bash
sudo apt install libgtk-4-dev
```

**Fedora:**
```bash
sudo dnf install gtk4-devel
```

## Installation

### Method 1: Install Script (Recommended)

```bash
chmod +x install.sh
./install.sh
```

This will build the project and install `index` to `/usr/local/bin` (or `~/.local/bin` if sudo is not available).

After installation, you can run Index from anywhere:

```bash
index
```

### Updating

To update Index to the latest version:

```bash
chmod +x update.sh
./update.sh
```

This script will rebuild and reinstall Index, automatically detecting where it was previously installed.

### Method 2: Cargo Install

```bash
cargo install --path .
```

This installs Index to `~/.cargo/bin` (make sure it's in your PATH).

### Method 3: Manual Installation

```bash
cargo build --release
sudo cp target/release/index /usr/local/bin/
```

## Building (without installing)

```bash
cargo build --release
```

## Running (from source)

```bash
cargo run --release
```

## Keyboard Shortcuts

- **Backspace** - Go to parent folder
- **Enter** - Open selected file/folder
- **Delete** - Move selected items to trash
- **Ctrl+H** - Toggle hidden files
- **Ctrl+A** - Select all files
- **F5** - Refresh current directory
- **m** - Open selected file with micro editor
- **h** - Open terminal in current directory

## Mouse Shortcuts

- **Mouse Back Button (Button 8)** - Navigate back in history
- **Mouse Forward Button (Button 9)** - Navigate forward in history

## Changelog

### Latest Changes

**Added:**
- Drag and drop support between windows (multi-file support)
- Visual feedback during drag operations (shows number of files being dragged)
- Mouse side buttons navigation (back/forward buttons)
- Keyboard shortcut "h" to open terminal in current directory
- Multi-selection support (can select multiple files at once)

**Fixed:**
- Drag and drop between windows in Hyprland window manager
- Cross-window file operations compatibility

**Improved:**
- File selection model changed from SingleSelection to MultiSelection
- Better drag and drop compatibility with other GTK4 applications
- Window-level drop target for improved cross-window support

## License

MIT
