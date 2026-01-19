# Alloy Desktop Environment

A complete desktop application ecosystem for Linux - modern, high-performance applications written in Rust with GTK4/libadwaita.

## 📦 Projects

### 🎛️ **Fuse** - Settings Manager
Settings application for Quickshell with full control over system appearance and behavior.

### 📁 **Index** - File Explorer
Lightweight and fast file manager with minimalist design.

### 📊 **Vitals** - System Monitor
Modern system monitor with comprehensive resource tracking.

### ⚡ **Spark** - Shell/Launcher System
Modern launcher/shell system for Quickshell with Wayland support.

### 🔧 **Core** - Shared Library
Shared library for system monitoring used by all applications.

## 🚀 Quick Start

### Requirements

- Rust 1.70+
- GTK4 development libraries
- libadwaita development libraries

**For Spark:**
- Quickshell
- Wayland compositor (tested with Hyprland)
- Optional: `cava`, `playerctl`, `pactl`, `grim`, `slurp`

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

### Building

**All applications:**
```bash
cd ~/.config/alloy
./rebuild_all.sh
```

**Individual applications:**
Each project has its own `install.sh` script:
```bash
cd fuse && ./install.sh
cd index && ./install.sh
cd Vitals && ./install.sh
cd spark && ./install.sh
```

### Running

```bash
./run_fuse.sh      # Fuse
./run_index.sh     # Index
cd Vitals && ./run.sh
cd spark && ./run.sh
```

## ⚙️ Configuration

**Main configuration:**
- `spark/colors.json` - Colors, wallpapers, sidebar settings
- Fallback: `~/.config/sharpshell/colors.json`

**Index:**
- `~/.config/index/.index_pinned` - Pinned folders

Settings are synchronized automatically between applications.

## ⌨️ Keyboard Shortcuts

**Index:**
- `Backspace` - Go to parent folder
- `Enter` - Open selected file/folder
- `Delete` - Move to trash
- `Ctrl+H` - Toggle hidden files

**Spark:**
- `Super+R` - Open launcher
- `Super+M` - Toggle dashboard
- `Super+V` - Open clipboard manager
- `!` - Web search prefix

## 🔧 Troubleshooting

**Build issues:**
```bash
sudo chown -R $USER:$USER ~/.config/alloy/*/target/
./rebuild_all.sh
```

**Spark not working:**
- Check Quickshell installation
- Verify keyboard shortcut in compositor
- Run `./spark/run.sh` to test

**Sidebar visibility:**
Requires Quickshell restart after changes in Fuse.

## 📄 License

MIT License
