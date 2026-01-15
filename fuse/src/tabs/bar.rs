use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, Label, Switch, ScrolledWindow, Button};
use std::sync::{Arc, Mutex};

use crate::core::config::ColorConfig;
use crate::core::quickshell;

pub struct BarTab {
    widget: ScrolledWindow,
    _config: Arc<Mutex<ColorConfig>>,
}

impl BarTab {
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

        let title = Label::new(Some("Bar Settings"));
        title.add_css_class("title");
        title.set_xalign(0.0);
        content.append(&title);

        // Sidebar Visibility toggle
        let current_visible = config.lock().unwrap().sidebar_visible.unwrap_or(true);
        let sidebar_visible_row = create_toggle_row(
            "Sidebar Visibility",
            "Show or hide the sidebar",
            {
                let config = Arc::clone(&config);
                move |enabled| {
                    // Reload config from disk to preserve existing settings
                    let mut cfg = ColorConfig::load();
                    cfg.set_sidebar_visible(enabled);
                    if let Err(e) = cfg.save() {
                        eprintln!("Error saving sidebar visibility: {}", e);
                    } else {
                        // Update the shared config
                        *config.lock().unwrap() = cfg.clone();
                        // Wait a bit for file to be written
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        // Notify quickshell about change
                        if let Err(e) = quickshell::notify_color_change() {
                            eprintln!("Error notifying quickshell: {}", e);
                        }
                        println!("Sidebar visibility set to: {}", enabled);
                    }
                }
            },
            current_visible,
        );
        content.append(&sidebar_visible_row);

        // Sidebar Position
        let position_section = create_sidebar_position_section(Arc::clone(&config));
        content.append(&position_section);

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

fn create_toggle_row(
    title: &str,
    description: &str,
    on_toggle: impl Fn(bool) + 'static,
    initial_value: bool,
) -> GtkBox {
    let row = GtkBox::new(Orientation::Horizontal, 15);
    row.add_css_class("settings-row");
    row.set_margin_start(16);
    row.set_margin_end(16);
    row.set_margin_top(16);
    row.set_margin_bottom(16);
    row.set_hexpand(true);
    row.set_halign(gtk4::Align::Fill);

    let text_box = GtkBox::new(Orientation::Vertical, 4);
    text_box.set_hexpand(true);
    text_box.set_halign(gtk4::Align::Fill);

    let title_label = Label::new(Some(title));
    title_label.add_css_class("row-title");
    title_label.set_xalign(0.0);
    title_label.set_halign(gtk4::Align::Start);
    text_box.append(&title_label);

    let desc_label = Label::new(Some(description));
    desc_label.add_css_class("row-description");
    desc_label.set_xalign(0.0);
    desc_label.set_halign(gtk4::Align::Start);
    text_box.append(&desc_label);

    row.append(&text_box);

    let toggle = Switch::new();
    toggle.set_active(initial_value);
    toggle.set_halign(gtk4::Align::End);
    toggle.set_hexpand(false);
    toggle.connect_active_notify(move |toggle| {
        on_toggle(toggle.is_active());
    });
    row.append(&toggle);

    row
}

fn create_sidebar_position_section(config: Arc<Mutex<ColorConfig>>) -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 16);
    section.add_css_class("settings-row");
    section.set_margin_start(15);
    section.set_margin_end(15);
    section.set_margin_top(15);
    section.set_margin_bottom(15);

    let header = GtkBox::new(Orientation::Horizontal, 15);
    
    let icon = Label::new(Some("󰍇"));
    icon.set_margin_end(12);
    header.append(&icon);

    let text_box = GtkBox::new(Orientation::Vertical, 2);
    text_box.set_hexpand(true);

    let title = Label::new(Some("Sidebar Position"));
    title.add_css_class("row-title");
    title.set_xalign(0.0);
    text_box.append(&title);

    let desc = Label::new(Some("Choose sidebar position: Left or Top"));
    desc.add_css_class("row-description");
    desc.set_xalign(0.0);
    text_box.append(&desc);

    header.append(&text_box);

    let button_box = GtkBox::new(Orientation::Horizontal, 10);
    
    let current_pos = config.lock().unwrap().sidebar_position.clone().unwrap_or_else(|| "left".to_string());
    let is_left = current_pos == "left";
    let is_top = current_pos == "top";

    let left_button = Button::with_label("Left");
    if is_left {
        left_button.add_css_class("suggested-action");
    }
    {
        let config = Arc::clone(&config);
        left_button.connect_clicked(move |_| {
            // Reload config from disk to preserve existing settings (like color preset)
            let mut cfg = ColorConfig::load();
            cfg.set_sidebar_position("left");
            if let Err(e) = cfg.save() {
                eprintln!("Error saving sidebar position: {}", e);
            } else {
                // Update the shared config
                *config.lock().unwrap() = cfg.clone();
                // Wait a bit for file to be written and synced to disk
                std::thread::sleep(std::time::Duration::from_millis(200));
                if let Err(e) = quickshell::notify_color_change() {
                    eprintln!("Error notifying quickshell: {}", e);
                }
            }
        });
    }
    button_box.append(&left_button);

    let top_button = Button::with_label("Top");
    if is_top {
        top_button.add_css_class("suggested-action");
    }
    {
        let config = Arc::clone(&config);
        top_button.connect_clicked(move |_| {
            // Reload config from disk to preserve existing settings (like color preset)
            let mut cfg = ColorConfig::load();
            cfg.set_sidebar_position("top");
            if let Err(e) = cfg.save() {
                eprintln!("Error saving sidebar position: {}", e);
            } else {
                // Update the shared config
                *config.lock().unwrap() = cfg.clone();
                // Wait a bit for file to be written and synced to disk
                std::thread::sleep(std::time::Duration::from_millis(200));
                if let Err(e) = quickshell::notify_color_change() {
                    eprintln!("Error notifying quickshell: {}", e);
                }
            }
        });
    }
    button_box.append(&top_button);

    header.append(&button_box);
    section.append(&header);

    section
}
