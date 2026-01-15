use gtk4::prelude::*;
use gtk4::{glib, Application, CssProvider};
use std::cell::RefCell;
use std::rc::Rc;

use crate::window::IndexWindow;
use crate::core::ColorConfig;

const APP_ID: &str = "com.index.fileexplorer";

pub struct IndexApp {
    app: Application,
    css_provider: Rc<RefCell<Option<CssProvider>>>,
}

impl IndexApp {
    pub fn new() -> Self {
        let app = Application::builder().application_id(APP_ID).build();
        let css_provider = Rc::new(RefCell::new(None));

        let css_provider_clone = css_provider.clone();
        app.connect_startup(move |_| {
            load_css_with_colors(&css_provider_clone);
        });

        app.connect_activate(|app| {
            let window = IndexWindow::new(app);
            window.present();
        });

        // Start monitoring for color changes
        let css_provider_monitor = css_provider.clone();
        start_color_monitoring(css_provider_monitor);

        Self { 
            app,
            css_provider,
        }
    }

    pub fn run(&self) -> glib::ExitCode {
        self.app.run()
    }
}

fn load_css_with_colors(css_provider_rc: &Rc<RefCell<Option<CssProvider>>>) {
    let config = ColorConfig::load();
    
    // Load base CSS
    let base_css = include_str!("style.css");
    
    // Replace CSS variables with colors from colors.json
    let dynamic_css = base_css
        .replace("@define-color index_bg #000000", &format!("@define-color index_bg {}", config.background))
        .replace("@define-color index_bg_secondary #0a0a0a", &format!("@define-color index_bg_secondary {}", config.primary))
        .replace("@define-color index_bg_tertiary #1a1a1a", &format!("@define-color index_bg_tertiary {}", config.secondary))
        .replace("@define-color index_fg #ffffff", &format!("@define-color index_fg {}", config.text))
        .replace("@define-color index_accent #ffffff", &format!("@define-color index_accent {}", config.accent));
    
    let provider = CssProvider::new();
    provider.load_from_string(&dynamic_css);
    
    let display = gtk4::gdk::Display::default().expect("Could not connect to display");
    
    // Remove old provider if exists
    if let Some(old_provider) = css_provider_rc.borrow().as_ref() {
        gtk4::style_context_remove_provider_for_display(&display, old_provider);
    }
    
    // Add new provider
    gtk4::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    
    // Store provider reference
    *css_provider_rc.borrow_mut() = Some(provider);
}

fn start_color_monitoring(css_provider_rc: Rc<RefCell<Option<CssProvider>>>) {
    let mut last_modified = std::time::UNIX_EPOCH;
    let mut last_notification_check = std::time::UNIX_EPOCH;
    
    glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
        let config_path = ColorConfig::get_config_path();
        let mut reload_needed = false;
        
        // Check if colors.json was modified
        if config_path.exists() {
            if let Ok(metadata) = std::fs::metadata(&config_path) {
                if let Ok(modified) = metadata.modified() {
                    if modified > last_modified {
                        last_modified = modified;
                        reload_needed = true;
                    }
                }
            }
        }
        
        // Check for color change notification file
        let notification_file = std::path::Path::new("/tmp/quickshell_color_change");
        if notification_file.exists() {
            if let Ok(metadata) = std::fs::metadata(notification_file) {
                if let Ok(modified) = metadata.modified() {
                    if modified > last_notification_check {
                        last_notification_check = modified;
                        reload_needed = true;
                        // Remove notification file after reading
                        let _ = std::fs::remove_file(notification_file);
                    }
                }
            }
        }
        
        if reload_needed {
            load_css_with_colors(&css_provider_rc);
        }
        
        glib::ControlFlow::Continue
    });
}
