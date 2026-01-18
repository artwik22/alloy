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
            sidebar_position: None,
            notifications_enabled: None,
            notification_sounds_enabled: None,
            sidebar_visible: None,
            rounding: Some("rounded".to_string()),
        }
    }
}

impl ColorConfig {
    pub fn get_config_path() -> PathBuf {
        // Try QUICKSHELL_PROJECT_PATH first
        if let Ok(project_path) = std::env::var("QUICKSHELL_PROJECT_PATH") {
            let path = PathBuf::from(project_path).join("colors.json");
            if path.exists() {
                return path;
            }
        }

        // Fallback to ~/.config/sharpshell/colors.json
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
    
    /// Convert hex color to RGB tuple (0.0-1.0 range for Cairo)
    pub fn hex_to_rgb(hex: &str) -> (f64, f64, f64) {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return (0.5, 0.5, 0.5); // Default gray
        }
        
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[0..2], 16),
            u8::from_str_radix(&hex[2..4], 16),
            u8::from_str_radix(&hex[4..6], 16),
        ) {
            (r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
        } else {
            (0.5, 0.5, 0.5) // Default gray
        }
    }
    
    /// Get accent color as RGB tuple
    pub fn accent_rgb(&self) -> (f64, f64, f64) {
        Self::hex_to_rgb(&self.accent)
    }
    
    /// Get primary color as RGB tuple
    pub fn primary_rgb(&self) -> (f64, f64, f64) {
        Self::hex_to_rgb(&self.primary)
    }
    
    /// Get secondary color as RGB tuple
    pub fn secondary_rgb(&self) -> (f64, f64, f64) {
        Self::hex_to_rgb(&self.secondary)
    }
}
