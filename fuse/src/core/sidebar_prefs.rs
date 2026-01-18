use std::path::PathBuf;
use dirs;

pub struct SidebarPrefs;

impl SidebarPrefs {
    fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("index")
            .join(".sidebar_prefs")
    }
}
