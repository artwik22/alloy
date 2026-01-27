import Quickshell
import Quickshell.Wayland
import Quickshell.Hyprland
import Quickshell.Io
import QtQuick
import "components"

ShellRoot {
    id: root

    ProcessHelper { id: processHelper }
    // Osobna kolejka tylko do czyszczenia /tmp/quickshell_command – bez czekania na cava/wallpaper/itd.
    ProcessHelper { id: processHelperClear }

    // Współdzielone właściwości (jeśli potrzebne)
    property var sharedData: QtObject {
        property var runCommand: processHelper ? processHelper.runCommand : function(){}
        property bool menuVisible: false
        property bool launcherVisible: false
        property bool volumeVisible: false
        property bool volumeEdgeHovered: false  // Czy myszka jest nad detektorem krawędzi
        property bool clipboardVisible: false
        property bool settingsVisible: false
        property bool lockScreenVisible: false  // Własny lock screen (zamiast swaylock/loginctl)
        property bool sidebarVisible: true  // Sidebar visibility toggle
        property string sidebarPosition: "left"  // Sidebar position: "left" or "top"
        property bool notificationsEnabled: true  // Enable/disable notifications
        property bool notificationSoundsEnabled: true  // Enable/disable notification sounds
        
        // Notification history for notification center
        property var notificationHistory: []

        // Color theme properties
        property string colorBackground: "#0a0a0a"
        property string colorPrimary: "#1a1a1a"
        property string colorSecondary: "#141414"
        property string colorText: "#ffffff"
        property string colorAccent: "#4a9eff"
        property real uiScale: 1.0
        property bool lowPerformanceMode: false  // true gdy ~/.config/alloy/low-perf istnieje – mniejsze zacinki na słabszym PC
        property string dashboardTileLeft: "battery"  // "battery" | "network" – co pokazywać na lewym kafelku dashboardu
    }
    
    // Color config file path - dynamically determined
    property string colorConfigPath: ""
    property string projectPath: ""

    // Polecenia z pliku: debounce i pojedyncza obsługa w czasie (unikamy „sam się otwiera i zamyka”)
    property bool commandHandlerBusy: false
    property string lastCommandHandled: ""
    property int lastCommandTime: 0
    
    // Single startup: one Process writes HOME and QUICKSHELL_PROJECT_PATH, then one read + loadColors + readLowPerf
    function initializeColorPath() {
        processHelper.runCommand(['sh', '-c', 'echo "$HOME|$QUICKSHELL_PROJECT_PATH" > /tmp/quickshell_init 2>/dev/null || true'], initPathsFromFile)
    }

    // Same path order as Fuse ColorConfig.get_config_path(): alloy → project → sharpshell → /tmp
    function initPathsFromFile() {
        var xhr = new XMLHttpRequest()
        xhr.open("GET", "file:///tmp/quickshell_init")
        xhr.onreadystatechange = function() {
            if (xhr.readyState !== XMLHttpRequest.DONE) return
            var line = (xhr.responseText || "").trim()
            var parts = line.split("|")
            var home = (parts[0] || "").trim()
            var projPath = (parts[1] || "").trim()
            root.projectPath = projPath

            function applyAndFinish(path) {
                colorConfigPath = path
                loadColors()
                readLowPerf()
            }
            function tryProjectThenFallback() {
                if (projPath && projPath.length > 0) {
                    var projectColors = projPath + "/colors.json"
                    var pxhr = new XMLHttpRequest()
                    pxhr.open("GET", "file://" + projectColors)
                    pxhr.onreadystatechange = function() {
                        if (pxhr.readyState === XMLHttpRequest.DONE && (pxhr.status === 200 || pxhr.status === 0))
                            applyAndFinish(projectColors)
                        else
                            applyAndFinish(home ? (home + "/.config/sharpshell/colors.json") : "/tmp/sharpshell/colors.json")
                    }
                    pxhr.send()
                } else {
                    applyAndFinish(home ? (home + "/.config/sharpshell/colors.json") : "/tmp/sharpshell/colors.json")
                }
            }
            function tryAlloy() {
                if (!(home && home.length > 0)) {
                    tryProjectThenFallback()
                    return
                }
                var alloyPath = home + "/.config/alloy/colors.json"
                var checkXhr = new XMLHttpRequest()
                checkXhr.open("GET", "file://" + alloyPath)
                checkXhr.onreadystatechange = function() {
                    if (checkXhr.readyState === XMLHttpRequest.DONE) {
                        if (checkXhr.status === 200 || checkXhr.status === 0)
                            applyAndFinish(alloyPath)
                        else
                            tryProjectThenFallback()
                    }
                }
                checkXhr.send()
            }
            tryAlloy()
        }
        xhr.send()
    }

    function readLowPerf() {
        var xhr = new XMLHttpRequest()
        xhr.open("GET", "file:///tmp/quickshell_low_perf")
        xhr.onreadystatechange = function() {
            if (xhr.readyState === XMLHttpRequest.DONE && (xhr.status === 200 || xhr.status === 0)) {
                var v = (xhr.responseText || "").trim()
                sharedData.lowPerformanceMode = (v === "1" || v === "true")
            }
        }
        xhr.send()
    }

    // Load colors on startup
    Component.onCompleted: {
        initializeColorPath()
    }
    
    // skipSidebarPrefs: when true (np. przy Fuse notify_color_change) nie nadpisujemy sidebarVisible/sidebarPosition,
    // żeby powtarzające się powiadomienia nie powodowały migotania panelu
    function loadColors(skipSidebarPrefs) {
        if (!colorConfigPath) {
            return
        }
        var skip = !!skipSidebarPrefs
        var xhr = new XMLHttpRequest()
        xhr.open("GET", "file://" + colorConfigPath)
        xhr.onreadystatechange = function() {
            if (xhr.readyState === XMLHttpRequest.DONE) {
                if (xhr.status === 200 || xhr.status === 0) {
                    try {
                        var json = JSON.parse(xhr.responseText)

                        // Check if a preset is selected and load its colors
                        if (json.colorPreset && json.presets && json.presets[json.colorPreset]) {
                            var preset = json.presets[json.colorPreset]
                            sharedData.colorBackground = preset.background
                            sharedData.colorPrimary = preset.primary
                            sharedData.colorSecondary = preset.secondary
                            sharedData.colorText = preset.text
                            sharedData.colorAccent = preset.accent
                        } else {
                            // Fall back to direct color values
                            if (json.background) sharedData.colorBackground = json.background
                            if (json.primary) sharedData.colorPrimary = json.primary
                            if (json.secondary) sharedData.colorSecondary = json.secondary
                            if (json.text) sharedData.colorText = json.text
                            if (json.accent) sharedData.colorAccent = json.accent
                        }
                        
                        // Load last wallpaper if available
                        if (json.lastWallpaper && json.lastWallpaper.length > 0) {
                            root.currentWallpaperPath = json.lastWallpaper
                        }
                        
                        if (!skip) {
                            // Sidebar prefs tylko przy starcie / init – przy Fuse notify nie nadpisujemy, żeby nie migotało
                            if (json.sidebarPosition && (json.sidebarPosition === "left" || json.sidebarPosition === "top")) {
                                sharedData.sidebarPosition = json.sidebarPosition
                            }
                            if (json.sidebarVisible !== undefined) {
                                var visible = json.sidebarVisible === true || json.sidebarVisible === "true"
                                sharedData.sidebarVisible = visible
                            }
                        }
                        
                        // Load color preset if available (for reference, not applied automatically)
                        if (json.colorPreset && json.colorPreset.length > 0) {
                        }
                        
                        // Load notification settings if available
                        if (json.notificationsEnabled !== undefined) {
                            sharedData.notificationsEnabled = json.notificationsEnabled === true || json.notificationsEnabled === "true"
                        }
                        if (json.notificationSoundsEnabled !== undefined) {
                            sharedData.notificationSoundsEnabled = json.notificationSoundsEnabled === true || json.notificationSoundsEnabled === "true"
                        }
                        if (json.uiScale === 75 || json.uiScale === 100 || json.uiScale === 125) {
                            sharedData.uiScale = json.uiScale / 100.0
                        }
                        if (json.dashboardTileLeft === "battery" || json.dashboardTileLeft === "network") {
                            sharedData.dashboardTileLeft = json.dashboardTileLeft
                        }
                    } catch (e) {
                    }
                }
            }
        }
        xhr.send()
    }
    
    // Funkcja do zamykania/otwierania menu
    function toggleMenu() {
        sharedData.menuVisible = !sharedData.menuVisible
    }

    // Funkcja otwierania launcher'a aplikacji
    function openLauncher() {
        sharedData.launcherVisible = !sharedData.launcherVisible
    }
    
    // Funkcja otwierania clipboard managera
    function openClipboardManager() {
        if (sharedData) {
            var oldState = sharedData.clipboardVisible
            sharedData.clipboardVisible = !oldState
        } else {
        }
    }

    // Funkcja otwierania aplikacji ustawień (fuse)
    function openSettings() {
        var scaleFactor = (sharedData && sharedData.uiScale) ? sharedData.uiScale : 1.0
        var scaleStr = String(scaleFactor)
        var cmd = "GTK_SCALE_FACTOR=" + scaleStr + " fuse 2>/dev/null || GTK_SCALE_FACTOR=" + scaleStr + " $HOME/.local/bin/fuse 2>/dev/null || GTK_SCALE_FACTOR=" + scaleStr + " $HOME/.config/alloy/fuse/target/release/fuse 2>/dev/null"
        processHelper.runCommand(['sh', '-c', cmd.replace(/'/g, "'\"'\"'")])
    }
    
    // Screenshot Service - Take screenshot with area selection
    function takeScreenshot() {
        processHelper.runCommand(['sh', '-c', 'if [ -n "$QUICKSHELL_PROJECT_PATH" ]; then echo "$QUICKSHELL_PROJECT_PATH/scripts/take-screenshot.sh"; elif [ -n "$HOME" ]; then echo "$HOME/.config/sharpshell/scripts/take-screenshot.sh"; else echo "/tmp/sharpshell/scripts/take-screenshot.sh"; fi > /tmp/quickshell_screenshot_script_path'], runScreenshotScript)
    }
    
    function runScreenshotScript() {
        var xhr = new XMLHttpRequest()
        xhr.open("GET", "file:///tmp/quickshell_screenshot_script_path")
        xhr.onreadystatechange = function() {
            if (xhr.readyState === XMLHttpRequest.DONE) {
                var scriptPath = xhr.responseText.trim()
                if (scriptPath && scriptPath.length > 0) {
                    var esc = scriptPath.replace(/\\/g, "\\\\").replace(/"/g, '\\"')
                    processHelper.runCommand(['sh', '-c', 'hyprctl dispatch exec "bash \\"' + esc + '\\""'])
                }
            }
        }
        xhr.send()
    }
    
    // Polecenia: odczyt XHR; gdy niepusty – akcja od razu, potem clear. busy=false dopiero w callbacku clear,
    // żeby następny poll nie zobaczył tego samego polecenia (brak migotania, bez okien „duplikat”).
    Timer {
        id: commandCheckTimer
        interval: (sharedData && sharedData.lowPerformanceMode) ? 600 : 40
        running: true
        repeat: true
        
        onTriggered: {
            if (root.commandHandlerBusy) return
            root.commandHandlerBusy = true
            var xhr = new XMLHttpRequest()
            xhr.open("GET", "file:///tmp/quickshell_command")
            xhr.onreadystatechange = function() {
                if (xhr.readyState !== XMLHttpRequest.DONE) return
                if (xhr.status !== 200 && xhr.status !== 0) {
                    root.commandHandlerBusy = false
                    return
                }
                var cmd = (xhr.responseText || "").trim()
                if (!cmd || cmd.length === 0) {
                    root.commandHandlerBusy = false
                    return
                }
                if (cmd === "openLauncher") {
                    root.openLauncher()
                } else if (cmd === "toggleMenu") {
                    root.toggleMenu()
                } else if (cmd === "openClipboardManager") {
                    root.openClipboardManager()
                } else if (cmd === "openSettings") {
                    root.openSettings()
                }
                processHelperClear.runCommand(['sh', '-c', ': > /tmp/quickshell_command'], function() {
                    root.commandHandlerBusy = false
                })
            }
            xhr.send()
        }
    }
    
    // Current wallpaper path - shared across all screens
    property string currentWallpaperPath: ""
    
    // Timer do monitorowania zmiany tapety
    Timer {
        id: wallpaperCheckTimer
        interval: (sharedData && sharedData.lowPerformanceMode) ? 1500 : 1000
        running: true
        repeat: true
        
        onTriggered: {
            var xhr = new XMLHttpRequest()
            xhr.open("GET", "file:///tmp/quickshell_wallpaper_path")
            xhr.onreadystatechange = function() {
                if (xhr.readyState === XMLHttpRequest.DONE) {
                    if (xhr.status === 200 || xhr.status === 0) {
                        var path = xhr.responseText.trim()
                        if (path && path.length > 0 && path !== root.currentWallpaperPath) {
                            root.currentWallpaperPath = path
                        }
                        // Plik pozostawiamy – kolejna zmiana tapety go nadpisze
                    }
                }
            }
            xhr.send()
        }
    }
    
    // Szybszy timer tylko dla sygnału Fuse – kolory odświeżane w ~500 ms po zmianie w Fuse
    Timer {
        id: fuseNotifyTimer
        interval: (sharedData && sharedData.lowPerformanceMode) ? 800 : 500
        running: true
        repeat: true
        onTriggered: {
            if (!root.colorConfigPath) return
            var xhr = new XMLHttpRequest()
            xhr.open("GET", "file:///tmp/quickshell_color_change?_=" + Date.now())
            xhr.onreadystatechange = function() {
                if (xhr.readyState === XMLHttpRequest.DONE && (xhr.status === 200 || xhr.status === 0)) {
                    var cmd = (xhr.responseText || "").trim()
                    if (cmd.length > 0) root.loadColors(true)
                }
            }
            xhr.send()
        }
    }
    
    // Timer do monitorowania zmiany kolorów i ustawień (colors.json + ponownie sygnał)
    Timer {
        id: colorCheckTimer
        interval: (sharedData && sharedData.lowPerformanceMode) ? 4000 : 2500
        running: true
        repeat: true
        
        onTriggered: {
            // Sprawdź czy colors.json się zmienił
            if (colorConfigPath) {
                var xhr = new XMLHttpRequest()
                xhr.open("GET", "file://" + colorConfigPath)
                xhr.onreadystatechange = function() {
                    if (xhr.readyState === XMLHttpRequest.DONE) {
                        if (xhr.status === 200 || xhr.status === 0) {
                            try {
                                var json = JSON.parse(xhr.responseText)
                                var changed = false
                                var bg, prim, sec, txt, acc
                                if (json.colorPreset && json.presets && json.presets[json.colorPreset]) {
                                    var p = json.presets[json.colorPreset]
                                    bg = p.background; prim = p.primary; sec = p.secondary; txt = p.text; acc = p.accent
                                } else {
                                    bg = json.background; prim = json.primary; sec = json.secondary; txt = json.text; acc = json.accent
                                }
                                if (bg && bg !== sharedData.colorBackground) { sharedData.colorBackground = bg; changed = true }
                                if (prim && prim !== sharedData.colorPrimary) { sharedData.colorPrimary = prim; changed = true }
                                if (sec && sec !== sharedData.colorSecondary) { sharedData.colorSecondary = sec; changed = true }
                                if (txt && txt !== sharedData.colorText) { sharedData.colorText = txt; changed = true }
                                if (acc && acc !== sharedData.colorAccent) { sharedData.colorAccent = acc; changed = true }
                                
                                // sidebarPosition/sidebarVisible tylko przy starcie (loadColors); periodiczne odświeżanie kolorów ich nie nadpisuje
                                if (json.dashboardTileLeft === "battery" || json.dashboardTileLeft === "network") {
                                    if (json.dashboardTileLeft !== sharedData.dashboardTileLeft) {
                                        sharedData.dashboardTileLeft = json.dashboardTileLeft
                                        changed = true
                                    }
                                }
                                
                                // Note: We don't auto-reload sidebarVisible from file watcher
                                // because user toggles it directly in the UI. Only load it on startup
                                // to avoid race condition with Dashboard save function
                                
                                if (changed) {
                                }
                            } catch (e) {
                            }
                        }
                    }
                }
                xhr.send()
            }
            
            // Sprawdź czy jest plik z poleceniem do przeładowania (Fuse notify_color_change)
            // Cache-busting: ?_=timestamp zapobiega zwracaniu cache’owanej treści przez file://
            var cmdXhr = new XMLHttpRequest()
            cmdXhr.open("GET", "file:///tmp/quickshell_color_change?_=" + Date.now())
            cmdXhr.onreadystatechange = function() {
                if (cmdXhr.readyState === XMLHttpRequest.DONE) {
                    if (cmdXhr.status === 200 || cmdXhr.status === 0) {
                        var cmd = (cmdXhr.responseText || "").trim()
                        if (cmd.length > 0) {
                            // Przeładuj kolory z colors.json (skipSidebarPrefs=true – bez migotania panelu)
                            root.loadColors(true)
                        }
                    }
                }
            }
            cmdXhr.send()
        }
    }
    
    // Per-screen: każdy typ okna ma własny Variants z delegate = PanelWindow (wymagane dla poprawnego bindowania do ekranu)
    Variants {
        model: Quickshell.screens
        delegate: Component {
            WallpaperBackground {
                required property var modelData
                screen: modelData
                currentWallpaper: root.currentWallpaperPath
            }
        }
    }
    Variants {
        model: Quickshell.screens
        delegate: Component {
            SidePanel {
                required property var modelData
                screen: modelData
                panelPosition: (root.sharedData && root.sharedData.sidebarPosition) ? root.sharedData.sidebarPosition : "left"
                sharedData: root.sharedData
                primaryScreen: Quickshell.screens.length > 0 ? Quickshell.screens[0] : null
                projectPath: root.projectPath
                launcherFunction: root.openLauncher
                screenshotFunction: root.takeScreenshot
            }
        }
    }
    Variants {
        model: Quickshell.screens
        delegate: Component {
            TopEdgeDetector {
                required property var modelData
                screen: modelData
                sharedData: root.sharedData
            }
        }
    }
    Variants {
        model: Quickshell.screens
        delegate: Component {
            RightEdgeDetector {
                required property var modelData
                screen: modelData
                sharedData: root.sharedData
            }
        }
    }
    Variants {
        model: Quickshell.screens
        delegate: Component {
            LockScreen {
                required property var modelData
                screen: modelData
                sharedData: root.sharedData
            }
        }
    }

    // Dashboard - jeden globalny (nie per-ekran)
    // Pokazuje się gdy myszka najedzie na górną krawędź ekranu
    Dashboard {
        id: dashboardInstance
        sharedData: root.sharedData
        projectPath: root.projectPath
    }

    // AppLauncher - launcher aplikacji (rofi-like)
    // Używamy pierwszego ekranu do wyśrodkowania
    AppLauncher {
        id: appLauncherInstance
        sharedData: root.sharedData
        screen: Quickshell.screens.length > 0 ? Quickshell.screens[0] : null
        projectPath: root.projectPath
    }

    // VolumeSlider - slider głośności na prawej krawędzi
    // Pokazuje się gdy myszka najedzie na prawą krawędź ekranu
    VolumeSlider {
        id: volumeSliderInstance
        sharedData: root.sharedData
        screen: Quickshell.screens.length > 0 ? Quickshell.screens[0] : null
    }

    // ClipboardManager - menedżer schowka (jeden na pierwszym ekranie)
    ClipboardManager {
        id: clipboardManagerInstance
        screen: Quickshell.screens.length > 0 ? Quickshell.screens[0] : null
        sharedData: root.sharedData
    }

    // NotificationDisplay - wyświetlanie powiadomień w prawym górnym rogu
    NotificationDisplay {
        id: notificationDisplayInstance
        sharedData: root.sharedData
    }
}

