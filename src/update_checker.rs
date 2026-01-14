const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub available: bool,
    pub latest_version: String,
    pub current_version: String,
}

pub struct UpdateChecker;

impl UpdateChecker {
    pub fn new() -> Self {
        Self
    }
    
    pub fn check_update(&self) -> UpdateInfo {
        // Update checking disabled for now
        UpdateInfo {
            available: false,
            latest_version: "unknown".to_string(),
            current_version: CURRENT_VERSION.to_string(),
        }
    }
}
