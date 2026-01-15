use std::fs;
use std::path::PathBuf;
use dirs;
use crate::core::config::ColorConfig;

const WALLPAPER_PATH_FILE: &str = "/tmp/quickshell_wallpaper_path";
const COLOR_CHANGE_FILE: &str = "/tmp/quickshell_color_change";

pub fn set_wallpaper(wallpaper_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Write wallpaper path to file that quickshell monitors
    fs::write(WALLPAPER_PATH_FILE, wallpaper_path)?;
    
    // Also update colors.json
    let mut config = ColorConfig::load();
    config.set_wallpaper(wallpaper_path);
    config.save()?;
    
    Ok(())
}

pub fn notify_color_change() -> Result<(), Box<dyn std::error::Error>> {
    // Write command to trigger quickshell to reload colors.json
    // Quickshell should monitor this file and reload colors when it changes
    // Use timestamp to ensure file change is detected
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    fs::write(COLOR_CHANGE_FILE, format!("reload_colors_{}", timestamp))?;
    // Sync to ensure file is written to disk
    if let Ok(file) = std::fs::File::create(COLOR_CHANGE_FILE) {
        file.sync_all().ok();
    }
    Ok(())
}

pub fn get_wallpapers_path() -> PathBuf {
    if let Some(home) = dirs::home_dir() {
        home.join("Pictures").join("Wallpapers")
    } else {
        PathBuf::from("/tmp/wallpapers")
    }
}
