# Alloy Desktop Environment

<div align="center">

**A complete desktop application ecosystem for Linux**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![GTK4](https://img.shields.io/badge/GTK4-4.12+-blue?style=for-the-badge)](https://www.gtk.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)](LICENSE)

*Modern, high-performance applications written in Rust with GTK4/libadwaita*

</div>

---

## 📦 Projects

### 🎛️ **Fuse** - Settings Manager
Settings application for Quickshell with full control over system appearance and behavior.

**Features:**
- 6 settings tabs (General, Colors, Wallpapers, Bar, System, Audio)
- 24 predefined color themes
- HEX color editing with live preview
- Wallpaper management with 16:9 previews
- Full audio mixer with PulseAudio integration
- Sidebar visibility and bar position control
- Toggles for hidden files and notifications

### 📁 **Index** - File Explorer
Lightweight and fast file manager with minimalist design.

**Features:**
- Fast directory navigation
- Copy, cut, paste files and folders
- Move to trash
- Rename files and folders
- Create new files and folders
- Search/filter files in current directory
- Sidebar with quick access to common locations
- Hidden files support (synchronized with Fuse)
- Pin folders to sidebar
- Drag & drop between windows
- Mouse side buttons navigation

### 📊 **Vitals** - System Monitor
Modern system monitor with comprehensive resource tracking.

**Features:**
- Dashboard with overview of all resources
- Detailed CPU monitoring with per-core breakdown
- Real-time memory monitoring
- Disk usage for all mounted partitions
- Network activity (upload/download)
- Process management with search
- Sparkline and detailed Cairo charts
- Modern, intuitive interface

### ⚡ **Spark** (SharpShell) - Shell/Launcher System
Modern launcher/shell system for Quickshell with Wayland support.

**Features:**
- Application launcher with fast search
- Web search (DuckDuckGo, Wikipedia, Reddit, YouTube)
- Built-in calculator
- Package management (pacman, AUR)
- Notes system
- Dashboard with system monitoring
- Media player with audio visualizer
- Clipboard manager with history
- Screenshot tool
- D-Bus notification system
- Side Panel / Top Bar with audio visualizer
- Native wallpaper management through Quickshell

### 🔧 **Core** - Shared Library
Shared library for system monitoring used by all applications.

**Features:**
- System data collection (`sysinfo`)
- Base chart widgets
- Common data structures
- GTK4/libadwaita integration

---

## 🚀 Quick Start

### Requirements

**Basic:**
- Rust 1.70+
- GTK4 development libraries
- libadwaita development libraries

**Optional (for Spark):**
- Quickshell
- Wayland compositor (tested with Hyprland)
- `cava` - audio visualizer
- `playerctl` - media player control
- `pactl` - PulseAudio control
- `grim` and `slurp` - screenshots (Wayland)

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

**Fuse:**
```bash
cd fuse
cargo build --release
# Or use the install script:
./install.sh
```

**Index:**
```bash
cd index
cargo build --release
# Or use the install script:
./install.sh
```

**Vitals:**
```bash
cd Vitals
cargo build --release
# Or use the install script:
./install.sh
```

**Spark:**
```bash
cd spark
./install.sh
```

**Core:**
```bash
cargo build --release
# Or use the update script:
./update.sh
```

### Running

**Fuse:**
```bash
./run_fuse.sh
# Or directly:
./fuse/target/release/fuse
```

**Index:**
```bash
./run_index.sh
# Or directly:
./index/target/release/index
```

**Vitals:**
```bash
cd Vitals
./run.sh
# Or directly:
./target/release/vitals
```

**Spark:**
```bash
cd spark
./run.sh
# Or configure in compositor:
# bind = SUPER, R, exec, ~/.config/alloy/spark/open-launcher.sh
```

---

## 📁 Project Structure

```
alloy/
├── core/                    # Shared library
│   ├── src/
│   │   ├── system.rs       # System monitoring
│   │   ├── graph.rs        # Chart widgets
│   │   └── ui.rs           # UI utilities
│   └── Cargo.toml
│
├── fuse/                    # Settings Manager
│   ├── src/
│   │   ├── app.rs
│   │   ├── window.rs
│   │   ├── tabs/           # Settings tabs
│   │   ├── widgets/        # Custom widgets
│   │   └── resources/      # CSS styles
│   ├── install.sh
│   └── Cargo.toml
│
├── index/                   # File Explorer
│   ├── src/
│   │   ├── app.rs
│   │   ├── window.rs
│   │   ├── core/           # Core functionality
│   │   └── widgets/        # UI widgets
│   ├── install.sh
│   └── Cargo.toml
│
├── Vitals/                  # System Monitor
│   ├── src/
│   │   ├── app.rs
│   │   ├── window.rs
│   │   ├── ui/             # UI views
│   │   └── data/           # Data collection
│   ├── install.sh
│   └── Cargo.toml
│
├── spark/                   # Shell/Launcher (QML)
│   ├── shell.qml          # Main entry point
│   ├── components/        # QML components
│   ├── scripts/           # Shell scripts
│   ├── install.sh
│   └── colors.json        # Color configuration
│
├── rebuild_all.sh          # Rebuild all Rust apps
├── run_fuse.sh            # Run Fuse
├── run_index.sh           # Run Index
└── update.sh              # Update Core library
```

---

## ⚙️ Configuration

### Configuration Files

**Main configuration (Spark/Fuse):**
- `spark/colors.json` - Colors, wallpapers, sidebar settings
- Fallback: `~/.config/sharpshell/colors.json`

**Index:**
- `~/.config/index/.index_pinned` - Pinned folders
- `~/.config/index/.sidebar_prefs` - Sidebar preferences

**Vitals:**
- No external configuration (settings in UI)

### Settings Synchronization

- **Fuse** monitors `colors.json` every 0.5 seconds
- **Index** monitors `colors.json` every 1 second for `showHiddenFiles`
- **Spark** (Quickshell) automatically reloads colors from `colors.json`
- Changes in Fuse are immediately visible in Index and Spark

---

## 🎨 Design & Styling

### Common Design Elements

- **Glassmorphism**: Semi-transparent panels with background blur
- **Rounded corners**: 10-24px depending on element
- **Glowing effects**: Multi-layer shadows with accent colors
- **Smooth animations**: 300-400ms with cubic-bezier for natural movement
- **Hover effects**: Transformations (scale, translate) with animations
- **Gradient backgrounds**: Dynamic backgrounds with accent colors

### Color Themes

- 24 predefined themes in Fuse
- HEX color editing with live preview
- Global border-radius (0-25px)
- All applications use shared color palette

---

## ⌨️ Keyboard Shortcuts

### Index (File Explorer)

- **Backspace** - Go to parent folder
- **Enter** - Open selected file/folder
- **Delete** - Move to trash
- **Ctrl+H** - Toggle hidden files
- **Ctrl+A** - Select all
- **F5** - Refresh directory
- **m** - Open file in micro editor
- **h** - Open terminal in current directory

### Spark (Launcher)

- **Super+R** - Open launcher
- **Super+M** - Toggle dashboard
- **Super+V** - Open clipboard manager
- **Escape** - Close
- **Arrow keys** - Navigation
- **Enter/Space** - Select
- **!** - Web search (DuckDuckGo)
- **!w** - Wikipedia search
- **!r** - Reddit search
- **!y** - YouTube search
- **=** - Calculator (e.g., `= 2+2`)

---

## 🔧 Troubleshooting

### Problem: Applications won't build

**Solution:**
```bash
# Fix permissions
sudo chown -R $USER:$USER ~/.config/alloy/*/target/

# Rebuild
./rebuild_all.sh
```

### Problem: Hidden files don't appear in Index

**Solution:**
1. Open Fuse → General Settings
2. Enable "Show Hidden Files"
3. Wait 1-2 seconds (Index monitors changes automatically)

### Problem: Sidebar doesn't hide after disabling in Fuse

**Solution:**
Sidebar visibility change requires Quickshell restart:
```bash
killall quickshell && quickshell
```

### Problem: Spark launcher doesn't work

**Solution:**
1. Check if Quickshell is installed
2. Check path in `shell.qml`
3. Check keyboard shortcut in compositor
4. Run `./spark/run.sh` to test

### Problem: Audio visualizer doesn't work in Spark

**Solution:**
```bash
# Install cava
sudo pacman -S cava  # Arch
sudo apt install cava  # Debian/Ubuntu

# Check if PulseAudio is running
pactl list sinks
```

---

## 🛠️ Development

### Workspace Structure

The project uses Rust workspace for better organization:

```toml
[workspace]
members = ["core", "fuse", "index", "Vitals"]
```

### Dependencies

**Core:**
- `gtk4`, `libadwaita`, `sysinfo`, `cairo-rs`

**Fuse:**
- `gtk4`, `libadwaita`, `serde`, `serde_json`, `tokio`, `image`

**Index:**
- `gtk4`, `libadwaita`, `trash`, `chrono`, `open`, `regex`

**Vitals:**
- `gtk4`, `libadwaita`, `sysinfo`, `tokio`, `cairo-rs`, `core` (local)

### Testing

```bash
# Test single application
cd fuse
cargo test

# Test all applications
cargo test --workspace

# Run in debug mode
cargo run
```

---

## 📝 Changelog

### Recent Changes

**Design:**
- ✅ Glassmorphism effects in all applications
- ✅ Rounded corners (10-24px)
- ✅ Glowing effects with accent colors
- ✅ Smooth animations with elastic bounce
- ✅ Hover transformations

**Fuse:**
- ✅ 24 predefined color themes
- ✅ Full audio mixer with PulseAudio
- ✅ Sidebar visibility control
- ✅ Hidden files toggle
- ✅ Improved switches (smaller, more visible)

**Index:**
- ✅ Automatic hidden files detection
- ✅ Drag & drop between windows
- ✅ Mouse side buttons navigation
- ✅ Multi-selection support

**Vitals:**
- ✅ Dashboard with resource overview
- ✅ Detailed CPU, Memory, Network charts
- ✅ Process management with search
- ✅ Modern, intuitive interface

**Spark:**
- ✅ Notes system
- ✅ Multi-platform package manager support
- ✅ Audio mixer in settings
- ✅ Notification sounds
- ✅ Premium UI enhancements

---

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

### Code Style

- Follow Rust best practices
- Use `rustfmt` for formatting
- Follow modern UI/UX best practices
- Document public APIs
- Write meaningful commit messages

---

## 📄 License

This project is licensed under the MIT License.

---

## 🙏 Acknowledgments

- Built with [GTK4](https://www.gtk.org/) and [libadwaita](https://gnome.pages.gitlab.gnome.org/libadwaita/)
- Shell system powered by [Quickshell](https://github.com/Quickshell/Quickshell)
- Audio visualization by [cava](https://github.com/karlstav/cava)
- Inspired by modern desktop environments

---

<div align="center">

**Made with ❤️ for the Linux community**

[Report Issues](https://github.com/artwik22/alloy/issues) • [Request Features](https://github.com/artwik22/alloy/issues)

</div>
