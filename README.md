# Alloy

A collection of modern desktop applications for Linux built with Rust and GTK4/libadwaita.

## 📦 Components

### 🎛️ **Fuse** - Settings Manager
System settings application with tabs for:
- Network & Bluetooth
- Appearance (colors, wallpapers, theme, rounding)
- Audio
- Index (file manager settings)
- Notifications
- System
- About

### 📁 **Index** - File Explorer
Lightweight file manager with minimalist design.

### 📊 **Vitals** - System Monitor
System resource monitor with comprehensive performance tracking.

### ⚡ **Spark** - Shell/Launcher
Quickshell-based launcher and shell system for Wayland.

## 🚀 Quick Start

### Requirements

**Basic:**
- Rust (edition 2021)
- GTK4 and libadwaita development libraries

**For Spark:**
- Quickshell
- Wayland compositor (tested with Hyprland)

### Installing Dependencies

**Arch Linux / CachyOS:**
```bash
sudo pacman -S rust gtk4 libadwaita
```

**Ubuntu / Debian:**
```bash
sudo apt install rustc libgtk-4-dev libadwaita-1-dev
```

**Fedora:**
```bash
sudo dnf install rust gtk4-devel libadwaita-devel
```

### Building & Installing

Each project has its own `install.sh` script:

```bash
# Fuse
cd fuse && ./install.sh

# Index
cd index && ./install.sh

# Vitals
cd Vitals && ./install.sh

# Spark
cd spark && ./install.sh
```

Installation scripts automatically:
- Check and configure Rust toolchain
- Build the project in release mode
- Install binaries to `/usr/local/bin` (requires sudo) or `~/.local/bin`

### Running

After installation, you can run applications directly from the terminal:

```bash
# Fuse
fuse

# Index
index

# Vitals
vitals

# Spark
cd spark && ./run.sh
```

Or directly from project directories:

```bash
# Fuse
cd fuse && ./target/release/fuse

# Index
cd index && ./target/release/index

# Vitals
cd Vitals && ./target/release/vitals

# Spark
cd spark && ./run.sh
```

## ⚙️ Configuration

**Main configuration:**
- `~/.config/alloy/colors.json` - Colors, wallpapers, theme settings
- Fallback: `~/.config/sharpshell/colors.json`

Settings are automatically synchronized between applications.

## 🏗️ Architecture

The project consists of several independent applications:

- **Fuse** - System settings application
- **Index** - File manager
- **Vitals** - System monitor (uses the `core` library from the main directory)
- **Spark** - Quickshell-based launcher (QML)

Each component can be built and installed independently.

## 🔧 Troubleshooting

**Build issues:**
```bash
# Check target directory permissions
sudo chown -R $USER:$USER ~/.config/alloy/*/target/

# Clean and rebuild
cd <project> && cargo clean && ./install.sh
```

**Spark not working:**
- Check Quickshell installation
- Verify keyboard shortcuts in compositor
- Run `cd spark && ./run.sh` to test
- Spark requires a Wayland compositor

**Dependency issues:**
- Make sure you have all required development libraries installed
- On Arch Linux, Quickshell may require installation from AUR (use `yay` or `paru`)

## 📄 License

MIT License

## 👥 Authors

Alloy Team
