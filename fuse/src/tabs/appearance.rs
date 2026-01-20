use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, Label, ScrolledWindow, Button, FlowBox, Picture, Overlay, gdk};
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;

use crate::core::config::ColorConfig;
use crate::core::quickshell;

// Helper function to set background color on a Box using CSS provider
fn set_box_background_color(box_widget: &gtk4::Box, color: &str) {
    // Create a unique CSS class name based on color
    let color_class = format!("color-bar-{}", color.replace("#", "c").replace(" ", ""));
    box_widget.add_css_class(&color_class);
    
    // Create CSS provider with the color
    let css_provider = gtk4::CssProvider::new();
    let css = format!(".{} {{ background-color: {}; }}", color_class, color);
    
    css_provider.load_from_string(&css);
    
    if let Some(display) = gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &css_provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

// 5 new color presets, each with light and dark variants
// Format: (name, bg, primary, secondary, text, accent)
const COLOR_PRESETS: &[(&str, &str, &str, &str, &str, &str, &str)] = &[
    // Preset 1: Ocean Breeze
    ("Ocean Breeze", "light", "#f0f8ff", "#e0f0ff", "#d0e8ff", "#1a1a2e", "#0066cc"),
    ("Ocean Breeze", "dark", "#0a0a1a", "#1a1a2e", "#151525", "#e0f0ff", "#4da6ff"),
    // Preset 2: Forest Mist
    ("Forest Mist", "light", "#f5faf5", "#e8f5e8", "#daf0da", "#1a2e1a", "#2d8659"),
    ("Forest Mist", "dark", "#0a1a0a", "#1a2e1a", "#152515", "#e8f5e8", "#4ade80"),
    // Preset 3: Sunset Glow
    ("Sunset Glow", "light", "#fff5e8", "#ffe8d0", "#ffdbc0", "#2e1a0a", "#ff6b35"),
    ("Sunset Glow", "dark", "#1a0a05", "#2e1a0a", "#251510", "#ffe8d0", "#ff8c5a"),
    // Preset 4: Monochrome
    ("Monochrome", "light", "#f5f5f5", "#e8e8e8", "#d0d0d0", "#1a1a1a", "#808080"),
    ("Monochrome", "dark", "#0a0a0a", "#1a1a1a", "#151515", "#e8e8e8", "#a0a0a0"),
    // Preset 5: Midnight Blue
    ("Midnight Blue", "light", "#e8f0ff", "#d0e0ff", "#c0d8ff", "#0a0a2e", "#3b82f6"),
    ("Midnight Blue", "dark", "#050a1a", "#0a0a2e", "#091025", "#d0e0ff", "#60a5fa"),
];

pub struct AppearanceTab {
    widget: ScrolledWindow,
    _config: Arc<Mutex<ColorConfig>>,
}

impl AppearanceTab {
    pub fn new(config: Arc<Mutex<ColorConfig>>) -> Self {
        let scrolled = ScrolledWindow::new();
        scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
        scrolled.set_hexpand(true);
        scrolled.set_vexpand(true);
        
        let content = GtkBox::new(Orientation::Vertical, 24);
        content.set_margin_start(12);
        content.set_margin_end(12);
        content.set_margin_top(12);
        content.set_margin_bottom(12);
        content.set_hexpand(true);
        content.set_vexpand(true);

        // Title
        let title = Label::new(Some("Appearance"));
        title.add_css_class("title");
        title.set_xalign(0.0);
        content.append(&title);

        // Theme selection (Light/Dark mode)
        let theme_section = create_theme_section(Arc::clone(&config));
        content.append(&theme_section);

        // Colors section with presets
        let colors_section = create_colors_section(Arc::clone(&config));
        content.append(&colors_section);

        // Background section
        let background_section = create_background_section(Arc::clone(&config));
        content.append(&background_section);

        // Rounding section
        let rounding_section = create_rounding_section(Arc::clone(&config));
        content.append(&rounding_section);

        scrolled.set_child(Some(&content));

        Self {
            widget: scrolled,
            _config: config,
        }
    }

    pub fn widget(&self) -> &ScrolledWindow {
        &self.widget
    }
}

fn create_theme_section(config: Arc<Mutex<ColorConfig>>) -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 0);
    section.add_css_class("settings-section");

    // Section header
    let header = GtkBox::new(Orientation::Vertical, 0);
    let section_title = Label::new(Some("Theme"));
    section_title.add_css_class("section-title");
    section_title.set_xalign(0.0);
    section_title.set_margin_start(18);
    section_title.set_margin_end(18);
    section_title.set_margin_top(18);
    section_title.set_margin_bottom(8);
    header.append(&section_title);

    let desc = Label::new(Some("Choose between light and dark theme"));
    desc.add_css_class("section-description");
    desc.set_xalign(0.0);
    desc.set_margin_start(18);
    desc.set_margin_end(18);
    desc.set_margin_bottom(18);
    header.append(&desc);

    // Theme cards container
    let cards_container = GtkBox::new(Orientation::Horizontal, 18);
    cards_container.set_margin_start(18);
    cards_container.set_margin_end(18);
    cards_container.set_margin_bottom(18);
    cards_container.set_halign(gtk4::Align::Start);

    // Light theme card
    let light_card = create_theme_card("Light", "light", true);
    let dark_card = create_theme_card("Dark", "dark", false);
    
    // Store references for selection management
    let light_card_clone = light_card.clone();
    let dark_card_clone = dark_card.clone();
    
    // Connect Light card click - apply light variant of current preset
    {
        let dark_ref = dark_card_clone.clone();
        let config_clone = Arc::clone(&config);
        light_card.connect_clicked(move |btn| {
            btn.add_css_class("theme-card-selected");
            dark_ref.remove_css_class("theme-card-selected");
            
            // Get current preset name or use first preset
            let current_cfg = config_clone.lock().unwrap();
            let preset_name = current_cfg.color_preset.clone().unwrap_or_else(|| "Ocean Breeze".to_string());
            drop(current_cfg);
            
            // Find light variant - try current preset first, then try all presets
            let light_preset = COLOR_PRESETS.iter()
                .find(|p| p.0 == preset_name && p.1 == "light")
                .or_else(|| COLOR_PRESETS.iter().find(|p| p.1 == "light"));
            
            if let Some(light_preset) = light_preset {
                let mut cfg = ColorConfig::load();
                cfg.update_colors(light_preset.2, light_preset.3, light_preset.4, light_preset.5, light_preset.6);
                cfg.set_preset(light_preset.0);
                if let Err(e) = cfg.save() {
                    eprintln!("Error saving light theme: {}", e);
                } else {
                    *config_clone.lock().unwrap() = cfg.clone();
                    std::thread::sleep(std::time::Duration::from_millis(300));
                    if let Err(e) = quickshell::notify_color_change() {
                        eprintln!("Error notifying quickshell: {}", e);
                    }
                    println!("Light theme applied: {}", light_preset.0);
                }
            } else {
                eprintln!("No light preset found!");
            }
        });
    }
    
    // Connect Dark card click - apply dark variant of current preset
    {
        let light_ref = light_card_clone.clone();
        let config_clone = Arc::clone(&config);
        dark_card.connect_clicked(move |btn| {
            btn.add_css_class("theme-card-selected");
            light_ref.remove_css_class("theme-card-selected");
            
            // Get current preset name or use first preset
            let current_cfg = config_clone.lock().unwrap();
            let preset_name = current_cfg.color_preset.clone().unwrap_or_else(|| "Ocean Breeze".to_string());
            drop(current_cfg);
            
            // Find dark variant - try current preset first, then try all presets
            let dark_preset = COLOR_PRESETS.iter()
                .find(|p| p.0 == preset_name && p.1 == "dark")
                .or_else(|| COLOR_PRESETS.iter().find(|p| p.1 == "dark"));
            
            if let Some(dark_preset) = dark_preset {
                let mut cfg = ColorConfig::load();
                cfg.update_colors(dark_preset.2, dark_preset.3, dark_preset.4, dark_preset.5, dark_preset.6);
                cfg.set_preset(dark_preset.0);
                if let Err(e) = cfg.save() {
                    eprintln!("Error saving dark theme: {}", e);
                } else {
                    *config_clone.lock().unwrap() = cfg.clone();
                    std::thread::sleep(std::time::Duration::from_millis(300));
                    if let Err(e) = quickshell::notify_color_change() {
                        eprintln!("Error notifying quickshell: {}", e);
                    }
                    println!("Dark theme applied: {}", dark_preset.0);
                }
            } else {
                eprintln!("No dark preset found!");
            }
        });
    }
    
    cards_container.append(&light_card);
    cards_container.append(&dark_card);

    section.append(&header);
    section.append(&cards_container);

    section
}

fn create_theme_card(name: &str, theme: &str, is_selected: bool) -> Button {
    let button = Button::new();
    button.add_css_class("theme-card");
    
    if is_selected {
        button.add_css_class("theme-card-selected");
    }

    let card_content = GtkBox::new(Orientation::Vertical, 0);
    card_content.set_hexpand(true);
    card_content.set_vexpand(true);

    // Preview area
    let preview_container = GtkBox::new(Orientation::Vertical, 0);
    preview_container.add_css_class("theme-preview");
    preview_container.add_css_class(&format!("theme-preview-{}", theme));
    preview_container.set_size_request(200, 120);
    preview_container.set_hexpand(true);
    preview_container.set_vexpand(true);

    card_content.append(&preview_container);

    // Theme name label
    let name_label = Label::new(Some(name));
    name_label.add_css_class("theme-name");
    name_label.set_margin_top(12);
    name_label.set_margin_bottom(12);
    card_content.append(&name_label);

    button.set_child(Some(&card_content));

    button
}

fn create_colors_section(config: Arc<Mutex<ColorConfig>>) -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 0);
    section.add_css_class("settings-section");

    // Section header
    let header = GtkBox::new(Orientation::Vertical, 0);
    let section_title = Label::new(Some("Colors"));
    section_title.add_css_class("section-title");
    section_title.set_xalign(0.0);
    section_title.set_margin_start(18);
    section_title.set_margin_end(18);
    section_title.set_margin_top(18);
    section_title.set_margin_bottom(8);
    header.append(&section_title);

    let desc = Label::new(Some("Choose from predefined color schemes"));
    desc.add_css_class("section-description");
    desc.set_xalign(0.0);
    desc.set_margin_start(18);
    desc.set_margin_end(18);
    desc.set_margin_bottom(18);
    header.append(&desc);

    // Presets container with FlowBox - responsive
    let flowbox = FlowBox::new();
    flowbox.set_column_spacing(12);
    flowbox.set_row_spacing(12);
    flowbox.set_halign(gtk4::Align::Fill);
    flowbox.set_hexpand(true);
    flowbox.set_vexpand(true);
    // Responsive: adjust columns based on available width (1-4 columns)
    flowbox.set_max_children_per_line(4);
    flowbox.set_min_children_per_line(1);
    flowbox.set_selection_mode(gtk4::SelectionMode::None);
    flowbox.set_homogeneous(true);

    // Group presets by name and create cards with light/dark variants
    let mut preset_groups: HashMap<&str, Vec<(&str, &str, &str, &str, &str, &str, &str)>> = HashMap::new();
    for preset in COLOR_PRESETS.iter() {
        let (name, theme, bg, primary, secondary, text, accent) = *preset;
        preset_groups.entry(name).or_insert_with(Vec::new).push((name, theme, bg, primary, secondary, text, accent));
    }

    for (name, variants) in preset_groups.iter() {
        let light_variant = variants.iter().find(|v| v.1 == "light");
        let dark_variant = variants.iter().find(|v| v.1 == "dark");
        
        if let (Some(light), Some(dark)) = (light_variant, dark_variant) {
            let preset_card = create_preset_card_with_variants(
                name,
                light.2, light.3, light.4, light.5, light.6, // light colors
                dark.2, dark.3, dark.4, dark.5, dark.6,   // dark colors
                Arc::clone(&config),
            );
            flowbox.append(&preset_card);
        }
    }

    let presets_container = GtkBox::new(Orientation::Vertical, 0);
    presets_container.set_margin_start(18);
    presets_container.set_margin_end(18);
    presets_container.set_margin_bottom(18);
    presets_container.append(&flowbox);

    section.append(&header);
    section.append(&presets_container);

    section
}

fn create_preset_card_with_variants(
    name: &str,
    light_bg: &str, light_primary: &str, light_secondary: &str, light_text: &str, light_accent: &str,
    dark_bg: &str, dark_primary: &str, dark_secondary: &str, dark_text: &str, dark_accent: &str,
    config: Arc<Mutex<ColorConfig>>,
) -> Button {
    let button = Button::new();
    button.add_css_class("preset-button");
    button.set_hexpand(true);
    button.set_vexpand(false);
    button.set_can_shrink(true);
    // Make button responsive - allow it to shrink and expand
    button.set_size_request(140, -1); // Minimum width for preset card

    let content = GtkBox::new(Orientation::Vertical, 12);
    content.set_margin_start(16);
    content.set_margin_end(16);
    content.set_margin_top(16);
    content.set_margin_bottom(16);

    // Light variant preview
    let light_label = Label::new(Some("Light"));
    light_label.add_css_class("preset-variant-label");
    light_label.set_margin_bottom(8);
    content.append(&light_label);

    let light_colors = GtkBox::new(Orientation::Vertical, 4);
    let bg_bar = gtk4::Box::new(Orientation::Horizontal, 0);
    bg_bar.set_size_request(-1, 10);
    bg_bar.add_css_class("color-bar");
    set_box_background_color(&bg_bar, light_bg);
    light_colors.append(&bg_bar);

    let primary_bar = gtk4::Box::new(Orientation::Horizontal, 0);
    primary_bar.set_size_request(-1, 10);
    primary_bar.add_css_class("color-bar");
    set_box_background_color(&primary_bar, light_primary);
    light_colors.append(&primary_bar);

    let accent_bar = gtk4::Box::new(Orientation::Horizontal, 0);
    accent_bar.set_size_request(-1, 10);
    accent_bar.add_css_class("color-bar");
    set_box_background_color(&accent_bar, light_accent);
    light_colors.append(&accent_bar);

    content.append(&light_colors);

    // Separator
    let separator = gtk4::Separator::new(Orientation::Horizontal);
    separator.set_margin_top(8);
    separator.set_margin_bottom(8);
    content.append(&separator);

    // Dark variant preview
    let dark_label = Label::new(Some("Dark"));
    dark_label.add_css_class("preset-variant-label");
    dark_label.set_margin_bottom(8);
    content.append(&dark_label);

    let dark_colors = GtkBox::new(Orientation::Vertical, 4);
    let bg_bar_dark = gtk4::Box::new(Orientation::Horizontal, 0);
    bg_bar_dark.set_size_request(-1, 10);
    bg_bar_dark.add_css_class("color-bar");
    set_box_background_color(&bg_bar_dark, dark_bg);
    dark_colors.append(&bg_bar_dark);

    let primary_bar_dark = gtk4::Box::new(Orientation::Horizontal, 0);
    primary_bar_dark.set_size_request(-1, 10);
    primary_bar_dark.add_css_class("color-bar");
    set_box_background_color(&primary_bar_dark, dark_primary);
    dark_colors.append(&primary_bar_dark);

    let accent_bar_dark = gtk4::Box::new(Orientation::Horizontal, 0);
    accent_bar_dark.set_size_request(-1, 10);
    accent_bar_dark.add_css_class("color-bar");
    set_box_background_color(&accent_bar_dark, dark_accent);
    dark_colors.append(&accent_bar_dark);

    content.append(&dark_colors);

    // Preset name
    let name_label = Label::new(Some(name));
    name_label.add_css_class("preset-name");
    name_label.set_margin_top(12);
    content.append(&name_label);

    button.set_child(Some(&content));

    // Store dark variant for click handler (light variant stored for potential future use)
    let _light_bg = light_bg.to_string();
    let _light_primary = light_primary.to_string();
    let _light_secondary = light_secondary.to_string();
    let _light_text = light_text.to_string();
    let _light_accent = light_accent.to_string();
    let dark_bg = dark_bg.to_string();
    let dark_primary = dark_primary.to_string();
    let dark_secondary = dark_secondary.to_string();
    let dark_text = dark_text.to_string();
    let dark_accent = dark_accent.to_string();
    let name = name.to_string();

    button.connect_clicked(move |_| {
        // For now, apply dark variant (could add theme selection later)
        let mut cfg = ColorConfig::load();
        cfg.update_colors(&dark_bg, &dark_primary, &dark_secondary, &dark_text, &dark_accent);
        cfg.set_preset(&name);
        if let Err(e) = cfg.save() {
            eprintln!("Error saving colors: {}", e);
        } else {
            *config.lock().unwrap() = cfg.clone();
            std::thread::sleep(std::time::Duration::from_millis(200));
            if let Err(e) = quickshell::notify_color_change() {
                eprintln!("Error notifying quickshell: {}", e);
            }
            println!("Color preset '{}' applied and saved", name);
        }
    });

    button
}


fn create_background_section(config: Arc<Mutex<ColorConfig>>) -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 0);
    section.add_css_class("settings-section");

    // Section header with title and Add Picture button
    let header = GtkBox::new(Orientation::Horizontal, 0);
    header.set_margin_start(18);
    header.set_margin_end(18);
    header.set_margin_top(18);
    header.set_margin_bottom(18);
    header.set_valign(gtk4::Align::Center);

    let section_title = Label::new(Some("Background"));
    section_title.add_css_class("section-title");
    section_title.set_xalign(0.0);
    section_title.set_hexpand(true);
    section_title.set_halign(gtk4::Align::Start);
    header.append(&section_title);

    // Add Picture button
    let add_button = Button::with_label("+ Add Picture…");
    add_button.add_css_class("flat");
    add_button.add_css_class("add-picture-button");
    add_button.set_halign(gtk4::Align::End);
    
    add_button.connect_clicked(move |_| {
        println!("Add Picture clicked - file chooser would open here");
    });
    
    header.append(&add_button);

    // Wallpaper grid - 3x3 initially
    let wallpapers_path = quickshell::get_wallpapers_path();
    let all_wallpapers = find_wallpapers(&wallpapers_path);
    let current_wallpaper = config.lock().unwrap().last_wallpaper.clone();

    // Create responsive container using FlowBox instead of Grid
    let grid_container = GtkBox::new(Orientation::Vertical, 12);
    grid_container.set_margin_start(18);
    grid_container.set_margin_end(18);
    grid_container.set_margin_bottom(18);
    grid_container.set_hexpand(true);
    grid_container.set_vexpand(true);

    // Create responsive FlowBox for initial wallpapers (3 columns)
    let flowbox = FlowBox::new();
    flowbox.set_column_spacing(12);
    flowbox.set_row_spacing(12);
    flowbox.set_halign(gtk4::Align::Fill);
    flowbox.set_hexpand(true);
    flowbox.set_vexpand(true);
    flowbox.set_max_children_per_line(3);
    flowbox.set_min_children_per_line(1);
    flowbox.set_selection_mode(gtk4::SelectionMode::None);
    flowbox.set_homogeneous(true);

    // Show first 9 wallpapers
    let initial_wallpapers: Vec<_> = all_wallpapers.iter().take(9).collect();
    let remaining_wallpapers: Vec<_> = all_wallpapers.iter().skip(9).collect();

    for wallpaper_path in initial_wallpapers.iter() {
        let is_selected = current_wallpaper.as_ref()
            .map(|w| w == wallpaper_path.to_string_lossy().as_ref())
            .unwrap_or(false);
        
        let tile = create_wallpaper_tile(wallpaper_path, is_selected, Arc::clone(&config));
        flowbox.append(&tile);
    }

    grid_container.append(&flowbox);

    // Expand button to show more wallpapers
    if !remaining_wallpapers.is_empty() {
        let expand_button = Button::with_label("Show More");
        expand_button.add_css_class("flat");
        expand_button.add_css_class("expand-wallpapers-button");
        expand_button.set_halign(gtk4::Align::Center);
        expand_button.set_margin_top(12);

        // Create expanded FlowBox (hidden initially) - also responsive
        let expanded_flowbox = FlowBox::new();
        expanded_flowbox.set_column_spacing(12);
        expanded_flowbox.set_row_spacing(12);
        expanded_flowbox.set_halign(gtk4::Align::Fill);
        expanded_flowbox.set_hexpand(true);
        expanded_flowbox.set_vexpand(true);
        expanded_flowbox.set_max_children_per_line(3);
        expanded_flowbox.set_min_children_per_line(1);
        expanded_flowbox.set_selection_mode(gtk4::SelectionMode::None);
        expanded_flowbox.set_homogeneous(true);
        expanded_flowbox.set_visible(false);

        // Add remaining wallpapers to expanded FlowBox
        for wallpaper_path in remaining_wallpapers.iter() {
            let is_selected = current_wallpaper.as_ref()
                .map(|w| w == wallpaper_path.to_string_lossy().as_ref())
                .unwrap_or(false);
            
            let tile = create_wallpaper_tile(wallpaper_path, is_selected, Arc::clone(&config));
            expanded_flowbox.append(&tile);
        }

        let expanded_flowbox_clone = expanded_flowbox.clone();
        expand_button.connect_clicked(move |btn| {
            let is_visible = expanded_flowbox_clone.is_visible();
            expanded_flowbox_clone.set_visible(!is_visible);
            if is_visible {
                btn.set_label("Show More");
            } else {
                btn.set_label("Show Less");
            }
        });

        grid_container.append(&expanded_flowbox);
        grid_container.append(&expand_button);
    }

    section.append(&header);
    section.append(&grid_container);

    section
}

fn create_wallpaper_tile(path: &PathBuf, is_selected: bool, config: Arc<Mutex<ColorConfig>>) -> Button {
    let button = Button::new();
    button.add_css_class("wallpaper-tile-appearance");
    
    if is_selected {
        button.add_css_class("wallpaper-tile-selected");
    }

    // Use Overlay to add checkmark on selected tile
    let overlay = Overlay::new();
    
    // Picture widget for wallpaper - responsive
    let picture = Picture::new();
    let file = gtk4::gio::File::for_path(path.as_path());
    picture.set_file(Some(&file));
    picture.set_content_fit(gtk4::ContentFit::Cover);
    picture.set_hexpand(true);
    picture.set_vexpand(true);
    picture.set_can_shrink(true);
    overlay.set_child(Some(&picture));

    // Selected indicator (checkmark)
    if is_selected {
        let checkmark_container = GtkBox::new(Orientation::Horizontal, 0);
        checkmark_container.add_css_class("checkmark-container");
        checkmark_container.set_halign(gtk4::Align::End);
        checkmark_container.set_valign(gtk4::Align::End);
        checkmark_container.set_margin_end(8);
        checkmark_container.set_margin_bottom(8);

        let checkmark = Label::new(Some("✓"));
        checkmark.add_css_class("checkmark-icon");
        checkmark_container.append(&checkmark);

        overlay.add_overlay(&checkmark_container);
    }

    button.set_child(Some(&overlay));
    button.set_hexpand(true);
    button.set_vexpand(true);
    button.set_can_shrink(true);
    // Set minimum size but allow expansion
    button.set_size_request(100, 80); // Minimum size for wallpaper tile

    let path_str = path.to_string_lossy().to_string();
    button.connect_clicked(move |_| {
        if let Err(e) = quickshell::set_wallpaper(&path_str) {
            eprintln!("Error setting wallpaper: {}", e);
        } else {
            // Update config
            let mut cfg = config.lock().unwrap();
            cfg.last_wallpaper = Some(path_str.clone());
            drop(cfg);
            // Note: Save would typically happen here
            println!("Wallpaper set to: {}", path_str);
        }
    });

    button
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
            }
        }
    }

    // Sort and limit to first 6 for demo
    wallpapers.sort();
    wallpapers.truncate(6);
    wallpapers
}

fn create_rounding_section(config: Arc<Mutex<ColorConfig>>) -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 0);
    section.add_css_class("settings-section");

    let section_title = Label::new(Some("Rounding"));
    section_title.add_css_class("section-title");
    section_title.set_xalign(0.0);
    section_title.set_margin_start(18);
    section_title.set_margin_end(18);
    section_title.set_margin_top(18);
    section_title.set_margin_bottom(8);
    section.append(&section_title);

    let desc = Label::new(Some("Choose corner rounding style"));
    desc.add_css_class("section-description");
    desc.set_xalign(0.0);
    desc.set_margin_start(18);
    desc.set_margin_end(18);
    desc.set_margin_bottom(18);
    section.append(&desc);

    let content = GtkBox::new(Orientation::Horizontal, 12);
    content.set_margin_start(18);
    content.set_margin_end(18);
    content.set_margin_bottom(18);
    content.set_valign(gtk4::Align::Center);
    
    let text_box = GtkBox::new(Orientation::Vertical, 2);
    text_box.set_hexpand(true);

    let title = Label::new(Some("Global Rounding"));
    title.add_css_class("row-title");
    title.set_xalign(0.0);
    text_box.append(&title);

    let desc_text = Label::new(Some("Choose corner rounding style: Rounded or Sharp"));
    desc_text.add_css_class("row-description");
    desc_text.set_xalign(0.0);
    text_box.append(&desc_text);

    content.append(&text_box);

    let button_box = GtkBox::new(Orientation::Horizontal, 10);
    button_box.set_valign(gtk4::Align::Center);
    
    let current_rounding = config.lock().unwrap().rounding.clone().unwrap_or_else(|| "rounded".to_string());
    let is_rounded = current_rounding == "rounded";
    let is_sharp = current_rounding == "sharp";

    let rounded_button = Button::with_label("Rounded");
    if is_rounded {
        rounded_button.add_css_class("suggested-action");
    }
    let sharp_button = Button::with_label("Sharp");
    if is_sharp {
        sharp_button.add_css_class("suggested-action");
    }
    
    {
        let config = Arc::clone(&config);
        let sharp_btn = sharp_button.clone();
        rounded_button.connect_clicked(move |btn| {
            let mut cfg = ColorConfig::load();
            cfg.set_rounding("rounded");
            if let Err(e) = cfg.save() {
                eprintln!("Error saving rounding: {}", e);
            } else {
                *config.lock().unwrap() = cfg.clone();
                btn.add_css_class("suggested-action");
                sharp_btn.remove_css_class("suggested-action");
                std::thread::sleep(std::time::Duration::from_millis(200));
                if let Err(e) = quickshell::notify_color_change() {
                    eprintln!("Error notifying quickshell: {}", e);
                }
                println!("Rounding set to: rounded");
            }
        });
    }
    button_box.append(&rounded_button);

    {
        let config = Arc::clone(&config);
        let rounded_btn = rounded_button.clone();
        sharp_button.connect_clicked(move |btn| {
            let mut cfg = ColorConfig::load();
            cfg.set_rounding("sharp");
            if let Err(e) = cfg.save() {
                eprintln!("Error saving rounding: {}", e);
            } else {
                *config.lock().unwrap() = cfg.clone();
                btn.add_css_class("suggested-action");
                rounded_btn.remove_css_class("suggested-action");
                std::thread::sleep(std::time::Duration::from_millis(200));
                if let Err(e) = quickshell::notify_color_change() {
                    eprintln!("Error notifying quickshell: {}", e);
                }
                println!("Rounding set to: sharp");
            }
        });
    }
    button_box.append(&sharp_button);

    content.append(&button_box);
    section.append(&content);

    section
}
