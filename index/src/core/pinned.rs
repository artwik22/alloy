use std::fs;
use std::path::{Path, PathBuf};

const PINNED_FILE: &str = ".index_pinned";

#[derive(Clone, Debug)]
pub struct PinnedFolder {
    pub name: String,
    pub path: PathBuf,
}

pub struct PinnedManager;

impl PinnedManager {
    fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("index")
            .join(PINNED_FILE)
    }

    pub fn load() -> Vec<PinnedFolder> {
        let config_path = Self::config_path();
        
        // If config doesn't exist, create default pinned folders
        if !config_path.exists() {
            let mut pinned = Vec::new();
            
            // Add default folders if they exist
            if let Some(home) = dirs::home_dir() {
                if home.exists() {
                    pinned.push(PinnedFolder {
                        name: "Home".to_string(),
                        path: home,
                    });
                }
            }
            
            if let Some(docs) = dirs::document_dir() {
                if docs.exists() {
                    pinned.push(PinnedFolder {
                        name: "Documents".to_string(),
                        path: docs,
                    });
                }
            }
            
            if let Some(downloads) = dirs::download_dir() {
                if downloads.exists() {
                    pinned.push(PinnedFolder {
                        name: "Downloads".to_string(),
                        path: downloads,
                    });
                }
            }
            
            // Save default pinned folders
            if let Err(e) = Self::save(&pinned) {
                eprintln!("Failed to save default pinned folders: {}", e);
            }
            
            return pinned;
        }

        match fs::read_to_string(&config_path) {
            Ok(content) => {
                let mut pinned = Vec::new();
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with('#') {
                        continue;
                    }
                    
                    // Format: path|name (name is optional)
                    let parts: Vec<&str> = trimmed.splitn(2, '|').collect();
                    if parts.is_empty() {
                        continue;
                    }
                    
                    let path_str = parts[0].trim();
                    let path = PathBuf::from(path_str);
                    
                    if !path.exists() {
                        continue;
                    }
                    
                    let name = if parts.len() > 1 {
                        parts[1].trim().to_string()
                    } else {
                        path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| path.to_string_lossy().to_string())
                    };
                    
                    pinned.push(PinnedFolder { name, path });
                }
                pinned
            }
            Err(_) => Vec::new(),
        }
    }

    pub fn save(pinned: &[PinnedFolder]) -> Result<(), std::io::Error> {
        let config_path = Self::config_path();
        
        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let mut content = String::from("# Index pinned folders\n");
        for item in pinned {
            content.push_str(&format!("{}|{}\n", item.path.to_string_lossy(), item.name));
        }
        
        fs::write(&config_path, content)
    }

    pub fn add(path: &Path) -> Result<(), std::io::Error> {
        let mut pinned = Self::load();
        
        // Check if already pinned
        if pinned.iter().any(|p| p.path == path) {
            return Ok(());
        }
        
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());
        
        pinned.push(PinnedFolder {
            name,
            path: path.to_path_buf(),
        });
        
        Self::save(&pinned)
    }

    pub fn remove(path: &Path) -> Result<(), std::io::Error> {
        let mut pinned = Self::load();
        pinned.retain(|p| p.path != path);
        Self::save(&pinned)
    }

    pub fn is_pinned(path: &Path) -> bool {
        Self::load().iter().any(|p| p.path == path)
    }
}
