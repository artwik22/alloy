use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, Label, ScrolledWindow, Button, Entry, Grid, Separator};
use std::sync::{Arc, Mutex};

use crate::core::config::ColorConfig;
use crate::core::quickshell;

const PRESETS: &[(&str, &str, &str, &str, &str, &str)] = &[
    ("Dark", "#0a0a0a", "#252525", "#1a1a1a", "#ffffff", "#6bb6ff"),
    ("Ocean", "#0a1628", "#1e3a52", "#152535", "#ffffff", "#4fc3f7"),
    ("Forest", "#0d1a0d", "#1e3a1e", "#152515", "#ffffff", "#66bb6a"),
    ("Violet", "#1a0d26", "#2e1f3f", "#231a35", "#ffffff", "#ab47bc"),
    ("Crimson", "#1a0a0a", "#2e1a1a", "#231515", "#ffffff", "#ef5350"),
    ("Amber", "#1a150d", "#2e251a", "#231f15", "#ffffff", "#ffb74d"),
    ("Teal", "#0d1a1a", "#1e2e2e", "#152525", "#ffffff", "#26a69a"),
    ("Rose", "#1a0d15", "#2e1a23", "#23151f", "#ffffff", "#f06292"),
    ("Sunset", "#1a150d", "#2e251a", "#231f15", "#ffffff", "#ff9800"),
    ("Midnight", "#0a0d1a", "#1e1f2d", "#151a23", "#ffffff", "#78909c"),
    ("Emerald", "#0d1a0d", "#1e3a1e", "#152515", "#ffffff", "#4caf50"),
    ("Lavender", "#1a0d1a", "#2e1a2d", "#231523", "#ffffff", "#ba68c8"),
    ("Sapphire", "#0d0d1a", "#1e1f2d", "#151a23", "#ffffff", "#42a5f5"),
    ("Coral", "#1a0d0d", "#2e1a1a", "#231515", "#ffffff", "#ff7043"),
    ("Mint", "#0d1a15", "#1e3a23", "#15251f", "#ffffff", "#4db6ac"),
    ("Plum", "#1a0d1a", "#2e1a2d", "#231523", "#ffffff", "#ba68c8"),
    ("Gold", "#1a160d", "#2e281a", "#231f15", "#ffffff", "#ffca28"),
    ("Monochrome", "#0a0a0a", "#1a1a1a", "#121212", "#ffffff", "#9e9e9e"),
    ("Cherry", "#1a0a0a", "#2e1a1a", "#231515", "#ffffff", "#e57373"),
    ("Azure", "#0a151a", "#1a2e3a", "#152325", "#ffffff", "#2196f3"),
    ("Jade", "#0d1a0d", "#1e3a1e", "#152515", "#ffffff", "#66bb6a"),
    ("Ruby", "#1a0a0a", "#2e1a1a", "#231515", "#ffffff", "#f44336"),
    ("Indigo", "#0d0a1a", "#1a162e", "#151223", "#ffffff", "#3f51b5"),
];

pub struct ColorsTab {
    widget: ScrolledWindow,
    _config: Arc<Mutex<ColorConfig>>,
}

impl ColorsTab {
    pub fn new(config: Arc<Mutex<ColorConfig>>) -> Self {
        let scrolled = ScrolledWindow::new();
        scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
        scrolled.set_hexpand(true);
        scrolled.set_vexpand(true);
        
        let content = GtkBox::new(Orientation::Vertical, 24);
        content.set_margin_start(16);
        content.set_margin_end(16);
        content.set_margin_top(16);
        content.set_margin_bottom(16);
        content.set_hexpand(true);
        content.set_vexpand(true);

        let title = Label::new(Some("Color Presets"));
        title.add_css_class("title");
        title.set_xalign(0.0);
        content.append(&title);

        // Presets grid
        let presets_section = create_presets_section(Arc::clone(&config));
        content.append(&presets_section);

        // Separator
        let separator = Separator::new(Orientation::Horizontal);
        content.append(&separator);

        // Custom colors section
        let custom_section = create_custom_colors_section(Arc::clone(&config));
        content.append(&custom_section);

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

fn create_presets_section(config: Arc<Mutex<ColorConfig>>) -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 16);

    let section_title = Label::new(Some("Color Presets"));
    section_title.add_css_class("section-title");
    section_title.set_xalign(0.0);
    section.append(&section_title);

    let desc = Label::new(Some("Choose from predefined color schemes"));
    desc.add_css_class("section-description");
    desc.set_xalign(0.0);
    section.append(&desc);

    // Grid with 3 columns
    let grid = Grid::new();
    grid.set_column_spacing(16);
    grid.set_row_spacing(16);

    for (i, preset) in PRESETS.iter().enumerate() {
        let (name, bg, primary, secondary, text, accent) = *preset;
        let row = i / 3;
        let col = i % 3;

        let preset_button = create_preset_button(
            name,
            bg,
            primary,
            secondary,
            text,
            accent,
            Arc::clone(&config),
        );
        grid.attach(&preset_button, col as i32, row as i32, 1, 1);
    }

    section.append(&grid);
    section
}

fn create_preset_button(
    name: &str,
    bg: &str,
    primary: &str,
    secondary: &str,
    text: &str,
    accent: &str,
    config: Arc<Mutex<ColorConfig>>,
) -> Button {
    let button = Button::new();
    button.add_css_class("preset-button");
    button.set_size_request(200, 104);

    let content = GtkBox::new(Orientation::Vertical, 4);
    content.set_margin_start(16);
    content.set_margin_end(16);
    content.set_margin_top(16);
    content.set_margin_bottom(16);

    // Color preview bars
    let bg_bar = gtk4::Box::new(Orientation::Horizontal, 0);
    bg_bar.set_size_request(-1, 12);
    bg_bar.add_css_class("color-bar");
    // Set background color via CSS
    content.append(&bg_bar);

    let primary_bar = gtk4::Box::new(Orientation::Horizontal, 0);
    primary_bar.set_size_request(-1, 12);
    primary_bar.add_css_class("color-bar");
    content.append(&primary_bar);

    let secondary_bar = gtk4::Box::new(Orientation::Horizontal, 0);
    secondary_bar.set_size_request(-1, 12);
    secondary_bar.add_css_class("color-bar");
    content.append(&secondary_bar);

    let text_bar = gtk4::Box::new(Orientation::Horizontal, 0);
    text_bar.set_size_request(-1, 12);
    text_bar.add_css_class("color-bar");
    content.append(&text_bar);

    // Preset name
    let name_label = Label::new(Some(name));
    name_label.add_css_class("preset-name");
    name_label.set_margin_top(12);
    content.append(&name_label);

    button.set_child(Some(&content));

    let bg = bg.to_string();
    let primary = primary.to_string();
    let secondary = secondary.to_string();
    let text = text.to_string();
    let accent = accent.to_string();
    let name = name.to_string();
    button.connect_clicked(move |_| {
        // Reload config from disk to preserve existing settings (like sidebar position)
        let mut cfg = ColorConfig::load();
        cfg.update_colors(&bg, &primary, &secondary, &text, &accent);
        cfg.set_preset(&name);
        if let Err(e) = cfg.save() {
            eprintln!("Error saving colors: {}", e);
        } else {
            // Update the shared config
            *config.lock().unwrap() = cfg.clone();
            // Wait a bit for file to be written and synced to disk
            std::thread::sleep(std::time::Duration::from_millis(200));
            // Notify quickshell about color change
            if let Err(e) = quickshell::notify_color_change() {
                eprintln!("Error notifying quickshell: {}", e);
            }
            println!("Color preset '{}' applied and saved", name);
        }
    });

    button
}

fn create_custom_colors_section(config: Arc<Mutex<ColorConfig>>) -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 16);

    let section_title = Label::new(Some("Custom Colors"));
    section_title.add_css_class("section-title");
    section_title.set_xalign(0.0);
    section.append(&section_title);

    let desc = Label::new(Some("Enter custom HEX color values"));
    desc.add_css_class("section-description");
    desc.set_xalign(0.0);
    section.append(&desc);

    let cfg = config.lock().unwrap();
    let bg_entry = create_color_input("Background", &cfg.background, Arc::clone(&config));
    let primary_entry = create_color_input("Primary", &cfg.primary, Arc::clone(&config));
    let secondary_entry = create_color_input("Secondary", &cfg.secondary, Arc::clone(&config));
    let text_entry = create_color_input("Text", &cfg.text, Arc::clone(&config));
    let accent_entry = create_color_input("Accent", &cfg.accent, Arc::clone(&config));
    drop(cfg);

    section.append(&bg_entry);
    section.append(&primary_entry);
    section.append(&secondary_entry);
    section.append(&text_entry);
    section.append(&accent_entry);

    // Apply button
    let apply_button = Button::with_label("Apply Custom Colors");
    apply_button.add_css_class("apply-button");
    apply_button.set_margin_top(16);
    {
        let config = Arc::clone(&config);
        apply_button.connect_clicked(move |_| {
            // Get current color values from config (updated by input fields)
            let current_cfg = config.lock().unwrap();
            let bg = current_cfg.background.clone();
            let primary = current_cfg.primary.clone();
            let secondary = current_cfg.secondary.clone();
            let text = current_cfg.text.clone();
            let accent = current_cfg.accent.clone();
            drop(current_cfg);
            
            // Reload config from disk to preserve existing settings (like sidebar position)
            let mut cfg = ColorConfig::load();
            // Apply the custom colors from input fields
            cfg.update_colors(&bg, &primary, &secondary, &text, &accent);
            cfg.set_preset(""); // Clear preset when using custom
            if let Err(e) = cfg.save() {
                eprintln!("Error saving custom colors: {}", e);
            } else {
                // Update the shared config
                *config.lock().unwrap() = cfg.clone();
                // Wait a bit for file to be written and synced to disk
                std::thread::sleep(std::time::Duration::from_millis(200));
                // Notify quickshell about color change
                if let Err(e) = quickshell::notify_color_change() {
                    eprintln!("Error notifying quickshell: {}", e);
                }
                println!("Custom colors applied and saved");
            }
        });
    }
    section.append(&apply_button);

    section
}

fn create_color_input(
    label: &str,
    initial_value: &str,
    config: Arc<Mutex<ColorConfig>>,
) -> GtkBox {
    let row = GtkBox::new(Orientation::Horizontal, 12);

    // Color preview
    let preview = gtk4::Box::new(Orientation::Horizontal, 0);
    preview.set_size_request(40, 40);
    preview.add_css_class("color-preview");
    row.append(&preview);

    let text_box = GtkBox::new(Orientation::Vertical, 4);
    text_box.set_hexpand(true);

    let label_widget = Label::new(Some(label));
    label_widget.add_css_class("color-label");
    label_widget.set_xalign(0.0);
    text_box.append(&label_widget);

    let entry = Entry::new();
    entry.set_text(initial_value);
    entry.add_css_class("color-entry");
    
    {
        let config = Arc::clone(&config);
        let field = match label {
            "Background" => "background",
            "Primary" => "primary",
            "Secondary" => "secondary",
            "Text" => "text",
            "Accent" => "accent",
            _ => return row,
        };
        entry.connect_changed(move |entry| {
            let text = entry.text();
            let mut cfg = config.lock().unwrap();
            match field {
                "background" => cfg.background = text.to_string(),
                "primary" => cfg.primary = text.to_string(),
                "secondary" => cfg.secondary = text.to_string(),
                "text" => cfg.text = text.to_string(),
                "accent" => cfg.accent = text.to_string(),
                _ => {}
            }
            // Auto-save on change (optional - could be debounced)
            // let _ = cfg.save();
        });
    }
    text_box.append(&entry);

    row.append(&text_box);
    row
}
