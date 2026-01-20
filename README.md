# Alloy Desktop Environment

Modern desktop applications for Linux built with Rust and GTK4/libadwaita.

## 📦 Projects

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
System resource monitor with comprehensive tracking.

### ⚡ **Spark** - Shell/Launcher
Quickshell-based launcher and shell system for Wayland.

## 🚀 Quick Start

### Requirements

- Rust 1.70+
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
cd fuse && ./install.sh
cd index && ./install.sh
cd Vitals && ./install.sh
cd spark && ./install.sh
```

### Running

```bash
# Fuse
cd fuse && ./run_fuse.sh
# or if installed: fuse

# Index
cd index && ./index

# Vitals
cd Vitals && ./run.sh

# Spark
cd spark && ./run.sh
```

## ⚙️ Configuration

**Main configuration:**
- `~/.config/alloy/colors.json` - Colors, wallpapers, theme settings
- Fallback: `~/.config/sharpshell/colors.json`

Settings are synchronized automatically between applications.

## 🔧 Troubleshooting

**Build issues:**
```bash
sudo chown -R $USER:$USER ~/.config/alloy/*/target/
cd <project> && cargo clean && ./install.sh
```

**Spark not working:**
- Check Quickshell installation
- Verify keyboard shortcuts in compositor
- Run `cd spark && ./run.sh` to test

## 📄 License

MIT License
