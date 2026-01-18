use gtk4::prelude::*;
use gtk4::{glib, CssProvider};
use libadwaita as adw;
use std::cell::RefCell;
use std::rc::Rc;

use crate::window::IndexWindow;
use crate::core::ColorConfig;

const APP_ID: &str = "com.index.fileexplorer";

pub struct IndexApp {
    app: adw::Application,
    css_provider: Rc<RefCell<Option<CssProvider>>>,
}

impl IndexApp {
    pub fn new() -> Self {
        let app = adw::Application::builder().application_id(APP_ID).build();
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
    
    // Replace Adwaita CSS variables with colors from colors.json
    let mut dynamic_css = base_css
        .replace("@define-color window_bg_color #242424", &format!("@define-color window_bg_color {}", config.background))
        .replace("@define-color window_fg_color #ffffff", &format!("@define-color window_fg_color {}", config.text))
        .replace("@define-color headerbar_bg_color #303030", &format!("@define-color headerbar_bg_color {}", config.primary))
        .replace("@define-color headerbar_fg_color #ffffff", &format!("@define-color headerbar_fg_color {}", config.text))
        .replace("@define-color card_bg_color #383838", &format!("@define-color card_bg_color {}", config.secondary))
        .replace("@define-color card_fg_color #ffffff", &format!("@define-color card_fg_color {}", config.text))
        .replace("@define-color accent_bg_color #3584e4", &format!("@define-color accent_bg_color {}", config.accent))
        .replace("@define-color accent_color #3584e4", &format!("@define-color accent_color {}", config.accent))
        .replace("@define-color sidebar_bg_color #2a2a2a", &format!("@define-color sidebar_bg_color {}", config.secondary))
        .replace("@define-color view_bg_color #1e1e1e", &format!("@define-color view_bg_color {}", config.background));
    
    // Apply rounding setting
    let rounding = config.rounding.as_deref().unwrap_or("rounded");
    if rounding == "sharp" {
        // Replace all border-radius values with 0px
        use regex::Regex;
        let re = Regex::new(r"border-radius:\s*[^;]+;").unwrap();
        dynamic_css = re.replace_all(&dynamic_css, "border-radius: 0px;").to_string();
    }
    
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
