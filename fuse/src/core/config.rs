use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use dirs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorConfig {
    pub background: String,
    pub primary: String,
    pub secondary: String,
    pub text: String,
    pub accent: String,
    #[serde(rename = "lastWallpaper", skip_serializing_if = "Option::is_none")]
    pub last_wallpaper: Option<String>,
    #[serde(rename = "colorPreset", skip_serializing_if = "Option::is_none")]
    pub color_preset: Option<String>,
    #[serde(rename = "sidebarPosition", skip_serializing_if = "Option::is_none")]
    pub sidebar_position: Option<String>,
    #[serde(rename = "notificationsEnabled", skip_serializing_if = "Option::is_none")]
    pub notifications_enabled: Option<bool>,
    #[serde(rename = "notificationSoundsEnabled", skip_serializing_if = "Option::is_none")]
    pub notification_sounds_enabled: Option<bool>,
    #[serde(rename = "sidebarVisible", skip_serializing_if = "Option::is_none")]
    pub sidebar_visible: Option<bool>,
    #[serde(rename = "rounding", skip_serializing_if = "Option::is_none")]
    pub rounding: Option<String>,
    #[serde(rename = "showHiddenFiles", skip_serializing_if = "Option::is_none")]
    pub show_hidden_files: Option<bool>,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            background: "#0a0a0a".to_string(),
            primary: "#1a1a1a".to_string(),
            secondary: "#121212".to_string(),
            text: "#ffffff".to_string(),
            accent: "#4a9eff".to_string(),
            last_wallpaper: None,
            color_preset: None,
            sidebar_position: Some("left".to_string()),
            notifications_enabled: Some(true),
            notification_sounds_enabled: Some(true),
            sidebar_visible: Some(true),
            rounding: Some("rounded".to_string()),
            show_hidden_files: Some(false),
        }
    }
}

impl ColorConfig {
    pub fn get_config_path() -> PathBuf {
        // 1. Try ~/.config/alloy/colors.json (Global Alloy Config)
        if let Some(home) = dirs::home_dir() {
            let path = home.join(".config").join("alloy").join("colors.json");
            if path.exists() {
                return path;
            }
        }

        // 2. Try QUICKSHELL_PROJECT_PATH first
        if let Ok(project_path) = std::env::var("QUICKSHELL_PROJECT_PATH") {
            let path = PathBuf::from(project_path).join("colors.json");
            if path.exists() {
                return path;
            }
        }

        // 3. Fallback to ~/.config/sharpshell/colors.json
        if let Some(home) = dirs::home_dir() {
            let path = home.join(".config").join("sharpshell").join("colors.json");
            if path.exists() {
                return path;
            }
            // Create directory if it doesn't exist
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            return path;
        }

        // Last resort: /tmp/sharpshell/colors.json
        PathBuf::from("/tmp/sharpshell/colors.json")
    }

    pub fn load() -> Self {
        let path = Self::get_config_path();
        if !path.exists() {
            return Self::default();
        }

        match fs::read_to_string(&path) {
            Ok(content) => {
                match serde_json::from_str::<ColorConfig>(&content) {
                    Ok(config) => config,
                    Err(e) => {
                        eprintln!("Error parsing colors.json: {}", e);
                        Self::default()
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading colors.json: {}", e);
                Self::default()
            }
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_config_path();
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Use Python script to save (same as quickshell) for compatibility
        self.save_via_python_script()
    }

    fn save_via_python_script(&self) -> Result<(), Box<dyn std::error::Error>> {
        use std::process::Command;
        
        let path = Self::get_config_path();
        let path_str = path.to_string_lossy();
        
        // Try to find Python script in QUICKSHELL_PROJECT_PATH or use direct save
        let script_path = if let Ok(project_path) = std::env::var("QUICKSHELL_PROJECT_PATH") {
            PathBuf::from(project_path).join("scripts").join("save-colors.py")
        } else if let Some(home) = dirs::home_dir() {
            home.join(".config").join("sharpshell").join("scripts").join("save-colors.py")
        } else {
            // Fallback: save directly
            return self.save_direct();
        };

        if !script_path.exists() {
            // Fallback to direct save if script doesn't exist
            return self.save_direct();
        }

        let script_str = script_path.to_string_lossy();
        
        // Build command with all arguments
        let mut cmd = Command::new("python3");
        cmd.arg(script_str.as_ref());
        cmd.arg(&self.background);
        cmd.arg(&self.primary);
        cmd.arg(&self.secondary);
        cmd.arg(&self.text);
        cmd.arg(&self.accent);
        cmd.arg(path_str.as_ref());
        
        // Add optional arguments
        if let Some(ref wp) = self.last_wallpaper {
            cmd.arg(wp);
        } else {
            cmd.arg("");
        }
        
        if let Some(ref preset) = self.color_preset {
            cmd.arg(preset);
        } else {
            cmd.arg("");
        }
        
        if let Some(ref pos) = self.sidebar_position {
            cmd.arg(pos);
        } else {
            cmd.arg("");
        }
        
        if let Some(enabled) = self.notifications_enabled {
            cmd.arg(if enabled { "true" } else { "false" });
        } else {
            cmd.arg("");
        }
        
        if let Some(enabled) = self.notification_sounds_enabled {
            cmd.arg(if enabled { "true" } else { "false" });
        } else {
            cmd.arg("");
        }
        
        if let Some(visible) = self.sidebar_visible {
            cmd.arg(if visible { "true" } else { "false" });
        } else {
            cmd.arg("");
        }
        
        if let Some(ref rounding) = self.rounding {
            cmd.arg(rounding);
        } else {
            cmd.arg("");
        }
        
        if let Some(show_hidden) = self.show_hidden_files {
            cmd.arg(if show_hidden { "true" } else { "false" });
        } else {
            cmd.arg("");
        }
        
        let output = cmd.output()?;
        if !output.status.success() {
            // Fallback to direct save on error
            return self.save_direct();
        }
        
        // Ensure file is synced to disk
        use std::fs::OpenOptions;
        if let Ok(file) = OpenOptions::new().write(true).open(&path) {
            file.sync_all().ok();
        }
        
        Ok(())
    }

    fn save_direct(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_config_path();
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        fs::write(&path, json)?;
        Ok(())
    }

    pub fn update_colors(&mut self, background: &str, primary: &str, secondary: &str, text: &str, accent: &str) {
        self.background = background.to_string();
        self.primary = primary.to_string();
        self.secondary = secondary.to_string();
        self.text = text.to_string();
        self.accent = accent.to_string();
    }

    pub fn set_wallpaper(&mut self, wallpaper_path: &str) {
        self.last_wallpaper = Some(wallpaper_path.to_string());
    }

    pub fn set_preset(&mut self, preset_name: &str) {
        self.color_preset = Some(preset_name.to_string());
    }

    pub fn set_sidebar_position(&mut self, position: &str) {
        self.sidebar_position = Some(position.to_string());
    }

    pub fn set_notifications_enabled(&mut self, enabled: bool) {
        self.notifications_enabled = Some(enabled);
    }

    pub fn set_notification_sounds_enabled(&mut self, enabled: bool) {
        self.notification_sounds_enabled = Some(enabled);
    }

    pub fn set_sidebar_visible(&mut self, visible: bool) {
        self.sidebar_visible = Some(visible);
    }

    pub fn set_rounding(&mut self, rounding: &str) {
        self.rounding = Some(rounding.to_string());
    }

    pub fn set_show_hidden_files(&mut self, show_hidden: bool) {
        self.show_hidden_files = Some(show_hidden);
    }
}
