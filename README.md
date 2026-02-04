# Alloy

**Alloy** is a modern, modular desktop environment suite designed for Linux systems. It combines a high-performance shell overlay, a native settings manager, and a lightweight file explorer to provide a cohesive and aesthetically pleasing user experience.

Alloy is composed of three main components: **Spark**, **Fuse**, and **Index**.

---

## ⚡ Spark (Shell)

**Spark** is the visual heart of Alloy. It is a shell built with [Quickshell](https://github.com/outfoxxed/quickshell) that provides the desktop interface elements.

### Key Features

*   **Advanced Launcher**:
    *   Fast, keyboard-centric application search.
    *   **Package Management**: Search for packages (AUR/Pacman) and install or remove them directly from the launcher interface.
*   **Custom Wallpaper System**:
    *   Supports distinct wallpapers per screen.

*   **Notification Center**:
    *   Fully custom notification server implementation.
    *   History view to review past notifications.
    *   "Do Not Disturb" mode and sound toggles.
*   **Screensaver**:
    *   Aesthetics-first screensaver with large clock and widgets.
    *   Automatically activates on system idle.
*   **Interactive Dashboard**:
    *   Global dashboard with tile-based widgets.
    *   **Swappable Tiles**: Customize layout (e.g., switch the main view between **Calendar** and **GitHub Activity**).
    *   Quick access to system stats (CPU/RAM, Network etc.).
*   **Dynamic Sidebar**:
    *   Toggleable visibility (can auto-hide).
    *   Flexible positioning: Left, Top, Right, or Bottom.
*   **Clipboard Manager**:
    *   Retains clipboard history for quick access to previous copies.
*   **Volume Control**:
    *   Edge-activated slider for quick volume adjustments.

### Usage
To start the Spark shell:
```bash
cd spark
./run.sh
```

---

## ⚙️ Fuse (Settings Manager)

**Fuse** is the configuration center for Alloy. It is a native application written in **Rust** using **GTK4** and **Libadwaita**.

### Features
*   **Theme Engine & Presets**:
    *   Create, save, and load **Color Presets**.
    *   Instantly switch themes across the entire suite (Spark, Index, Fuse).
*   **Appearance**:
    *   Fine-tune colors (Background, Primary, Accent, etc.).
    *   Manage the wallpaper library.
*   **System Customization**:
    *   **UI Scale**: Adjust scaling (75%, 100%, 125%) for different displays.
    *   **Sidebar**: Configure position and default visibility.
    *   **Notifications**: Toggle popups and sounds.

### Build & Run
```bash
cd fuse
cargo run --release
```

---

## 📂 Index (File Explorer)

**Index** is a lightweight, fast file explorer designed to fit perfectly with the Alloy aesthetic. It is built with **Rust**, **GTK4**, and **Libadwaita**.

### Features
*   **Minimalist Interface**: Clean and focused design.
*   **Fast Navigation**: Optimized for speed.
*   **Essential Operations**: Tailored for common file management tasks.

### Build & Run
```bash
cd index
cargo run --release
```

---

## 🛠️ Scripts & UX Utilities

Alloy includes global automation scripts located in `scripts/`:

*   `battery_monitor.sh`: Monitors battery status and sends low battery notifications.
*   `idle-screensaver.sh`: Manages idle time detection to automatically launch the screensaver.

Note: Internal scripts for Spark (like package management and screenshot tools) are located in `spark/scripts/`.

## 📸 Gallery

i am to lazy to do pretty screenshots today

## 🎨 Configuration

Configuration is centralized in `~/.config/alloy/colors.json`. While **Fuse** is the recommended editor, you can manually verify settings here.

**Low Performance Mode**:
If you are running on older hardware, you can disable blur effects and complex animations:
```bash
touch ~/.config/alloy/low-perf
```
Restart Spark for changes to take effect.
