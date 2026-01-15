use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, Label, ScrolledWindow, Button, FlowBox, Image};
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
        let content = GtkBox::new(Orientation::Vertical, 24);
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
        flowbox.set_max_children_per_line(3);
        flowbox.set_min_children_per_line(1);
        flowbox.set_selection_mode(gtk4::SelectionMode::None);
        flowbox.set_homogeneous(false);
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

    // Clear existing - remove all children
    let mut child = flowbox.first_child();
    while let Some(c) = child {
        let next = c.next_sibling();
        flowbox.remove(&c);
        child = next;
    }

    // Add wallpapers - FlowBox will automatically wrap based on available width
    for (idx, wallpaper_path) in wallpapers.iter().enumerate() {
        println!("  [{}/{}] Adding wallpaper: {:?}", idx + 1, wallpapers.len(), wallpaper_path);
        if !wallpaper_path.exists() {
            eprintln!("    WARNING: File does not exist!");
            continue;
        }
        let tile = create_wallpaper_tile(wallpaper_path, Arc::clone(config));
        flowbox.append(&tile);
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

fn create_wallpaper_tile(path: &PathBuf, _config: Arc<Mutex<ColorConfig>>) -> Button {
    let button = Button::new();
    button.add_css_class("wallpaper-tile");
    
    // Use responsive sizing - FlowBox will distribute space
    // Set minimum width for smaller windows, but allow expansion
    // FlowBox with max_children_per_line(3) will ensure 3 columns max
    let min_width = 300;
    let min_height = (min_width as f64 * 9.0 / 16.0) as i32;

    // Clone path for the closure
    let path_str = path.to_string_lossy().to_string();
    
    // Image - use proper scaling with aspect ratio
    // Set minimum size but allow expansion
    println!("Loading image from: {:?}", path);
    let image = Image::from_file(path.as_path());
    image.set_size_request(min_width, min_height);
    image.set_halign(gtk4::Align::Fill);
    image.set_valign(gtk4::Align::Fill);
    image.set_vexpand(false);
    image.set_hexpand(false);
    image.set_opacity(1.0);

    button.set_child(Some(&image));
    
    // Make button expand to fill available space in FlowBox
    // This ensures tiles fill the available width in their column
    button.set_hexpand(true);
    button.set_vexpand(false);
    button.set_size_request(min_width, min_height);

    button.connect_clicked(move |_| {
        if let Err(e) = quickshell::set_wallpaper(&path_str) {
            eprintln!("Error setting wallpaper: {}", e);
        }
    });

    button
}
