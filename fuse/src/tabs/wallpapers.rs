use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, Label, ScrolledWindow, Button, FlowBox};
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::fs;

use crate::core::config::ColorConfig;
use crate::core::quickshell;

pub struct WallpapersTab {
    widget: ScrolledWindow,
    config: Arc<Mutex<ColorConfig>>,
    flowbox: FlowBox,
}

impl WallpapersTab {
    pub fn new(config: Arc<Mutex<ColorConfig>>) -> Self {
        let scrolled = ScrolledWindow::new();
        let content = GtkBox::new(Orientation::Vertical, 32);
        content.set_margin_start(24);
        content.set_margin_end(24);
        content.set_margin_top(24);
        content.set_margin_bottom(24);
        content.set_hexpand(true);
        content.set_vexpand(true);

        let header = GtkBox::new(Orientation::Horizontal, 16);
        
        let title = Label::new(Some("Select Wallpaper"));
        title.add_css_class("title");
        title.set_xalign(0.0);
        title.set_hexpand(true);
        header.append(&title);

        let refresh_button = Button::with_label("Refresh");
        refresh_button.add_css_class("refresh-button");
        header.append(&refresh_button);

        content.append(&header);

        let flowbox = FlowBox::new();
        flowbox.set_column_spacing(16);
        flowbox.set_row_spacing(16);
        flowbox.set_halign(gtk4::Align::Fill);
        flowbox.set_hexpand(true);
        flowbox.set_vexpand(true);
        // Responsive: adjust columns based on available width
        // Will show 1-4 columns depending on window size
        flowbox.set_max_children_per_line(4);
        flowbox.set_min_children_per_line(1);
        flowbox.set_selection_mode(gtk4::SelectionMode::None);
        flowbox.set_homogeneous(true); // Make tiles equal size for better grid
        content.append(&flowbox);

        // Only vertical scrolling, no horizontal scrolling
        scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
        scrolled.set_child(Some(&content));

        let tab = Self {
            widget: scrolled,
            config: Arc::clone(&config),
            flowbox,
        };

        // Load wallpapers
        tab.load_wallpapers();

        // Connect refresh button
        {
            let flowbox_clone = tab.flowbox.clone();
            let config_clone = Arc::clone(&tab.config);
            refresh_button.connect_clicked(move |_| {
                load_wallpapers_into_flowbox(&flowbox_clone, &config_clone);
            });
        }

        tab
    }

    pub fn load_wallpapers(&self) {
        let wallpapers_path = quickshell::get_wallpapers_path();
        let wallpapers = find_wallpapers(&wallpapers_path);
        
        // Update title - store count for later use
        let _count = wallpapers.len();
        
        load_wallpapers_into_flowbox(&self.flowbox, &self.config);
        
        // Title update will be handled by the caller if needed
    }

    pub fn widget(&self) -> &ScrolledWindow {
        &self.widget
    }
}

fn load_wallpapers_into_flowbox(flowbox: &FlowBox, config: &Arc<Mutex<ColorConfig>>) {
    let wallpapers_path = quickshell::get_wallpapers_path();
    let wallpapers = find_wallpapers(&wallpapers_path);

    println!("Loading {} wallpapers from {:?}", wallpapers.len(), wallpapers_path);

    // Get current wallpaper to highlight active one
    let current_wallpaper = {
        let cfg = config.lock().unwrap();
        cfg.last_wallpaper.clone()
    };

    // Clear existing - remove all children
    let mut child = flowbox.first_child();
    while let Some(c) = child {
        let next = c.next_sibling();
        flowbox.remove(&c);
        child = next;
    }

    // Store all buttons to update them when one is clicked
    let mut buttons: Vec<Button> = Vec::new();

    // Add wallpapers - FlowBox will automatically wrap based on available width
    for (idx, wallpaper_path) in wallpapers.iter().enumerate() {
        println!("  [{}/{}] Adding wallpaper: {:?}", idx + 1, wallpapers.len(), wallpaper_path);
        if !wallpaper_path.exists() {
            eprintln!("    WARNING: File does not exist!");
            continue;
        }
        let is_active = current_wallpaper.as_ref()
            .map(|w| w.as_str() == wallpaper_path.to_string_lossy().as_ref())
            .unwrap_or(false);
        let tile = create_wallpaper_tile(wallpaper_path, is_active, Arc::clone(config));
        buttons.push(tile.clone());
        flowbox.append(&tile);
    }
    
    // Update all buttons when one is clicked
    let buttons_clone = buttons.clone();
    for (idx, button) in buttons.iter().enumerate() {
        let button_clone = button.clone();
        let all_buttons = buttons_clone.clone();
        let path_str = wallpapers[idx].to_string_lossy().to_string();
        let path_str_clone = path_str.clone();
        let config_clone = Arc::clone(config);
        
        button.connect_clicked(move |_| {
            // Update all buttons - remove active class from all
            for btn in &all_buttons {
                btn.remove_css_class("wallpaper-active");
                btn.remove_css_class("wallpaper-applying");
            }
            button_clone.add_css_class("wallpaper-applying");
            
            if let Err(e) = quickshell::set_wallpaper(&path_str_clone) {
                eprintln!("Error setting wallpaper: {}", e);
                button_clone.remove_css_class("wallpaper-applying");
            } else {
                // Mark as active after successful set
                button_clone.remove_css_class("wallpaper-applying");
                button_clone.add_css_class("wallpaper-active");
                
                // Update config
                let mut cfg = config_clone.lock().unwrap();
                cfg.set_wallpaper(&path_str_clone);
                let _ = cfg.save();
            }
        });
    }
    
    println!("Finished loading wallpapers");
}

fn find_wallpapers(path: &PathBuf) -> Vec<PathBuf> {
    let mut wallpapers = Vec::new();
    
    if !path.exists() {
        return wallpapers;
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext_lower = ext.to_string_lossy().to_lowercase();
                    if matches!(ext_lower.as_str(), "jpg" | "jpeg" | "png" | "webp" | "gif") {
                        wallpapers.push(path);
                    }
                }
            } else if path.is_dir() {
                // Check subdirectories (maxdepth 2)
                if let Ok(sub_entries) = fs::read_dir(&path) {
                    for sub_entry in sub_entries.flatten() {
                        let sub_path = sub_entry.path();
                        if sub_path.is_file() {
                            if let Some(ext) = sub_path.extension() {
                                let ext_lower = ext.to_string_lossy().to_lowercase();
                                if matches!(ext_lower.as_str(), "jpg" | "jpeg" | "png" | "webp" | "gif") {
                                    wallpapers.push(sub_path);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    wallpapers.sort();
    wallpapers
}

fn create_wallpaper_tile(path: &PathBuf, is_active: bool, _config: Arc<Mutex<ColorConfig>>) -> Button {
    let button = Button::new();
    button.add_css_class("wallpaper-tile");
    
    if is_active {
        button.add_css_class("wallpaper-active");
    }
    
    // Use Picture widget for better image loading and display
    let picture = gtk4::Picture::new();
    let file = gtk4::gio::File::for_path(path.as_path());
    picture.set_file(Some(&file));
    picture.set_content_fit(gtk4::ContentFit::Cover);
    picture.set_can_shrink(true); // Allow shrinking for responsiveness
    // Responsive scaling - expand to fill available space
    picture.set_halign(gtk4::Align::Fill);
    picture.set_valign(gtk4::Align::Fill);
    picture.set_vexpand(true);
    picture.set_hexpand(true);

    button.set_child(Some(&picture));
    
    // Make button expand to fill available space in FlowBox
    // FlowBox will automatically distribute space based on available width
    button.set_hexpand(true);
    button.set_vexpand(true);
    // Don't set size_request here - let CSS handle minimum sizes for better responsiveness

    button
}
