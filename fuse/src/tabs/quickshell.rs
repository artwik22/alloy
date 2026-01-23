use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, Label, ScrolledWindow, Switch, Button};
use std::sync::{Arc, Mutex};

use crate::core::config::ColorConfig;
use crate::core::quickshell;

pub struct QuickshellTab {
    widget: ScrolledWindow,
    _config: Arc<Mutex<ColorConfig>>,
}

impl QuickshellTab {
    pub fn new(config: Arc<Mutex<ColorConfig>>) -> Self {
        let scrolled = ScrolledWindow::new();
        scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
        scrolled.set_hexpand(true);
        scrolled.set_vexpand(true);
        
        // GNOME spacing: 24px section gap, 12px container margins
        let content = GtkBox::new(Orientation::Vertical, 24);
        content.set_margin_start(12);
        content.set_margin_end(12);
        content.set_margin_top(12);
        content.set_margin_bottom(12);
        content.set_hexpand(true);
        content.set_vexpand(true);

        // Title
        let title = Label::new(Some("QuickShell"));
        title.add_css_class("title");
        title.set_xalign(0.0);
        title.set_halign(gtk4::Align::Start);
        content.append(&title);

        // Sidebar Visibility section
        let sidebar_visible_section = create_sidebar_visible_section(Arc::clone(&config));
        sidebar_visible_section.set_hexpand(true);
        content.append(&sidebar_visible_section);

        // Sidebar Position section
        let sidebar_position_section = create_sidebar_position_section(Arc::clone(&config));
        sidebar_position_section.set_hexpand(true);
        content.append(&sidebar_position_section);

        // Notifications section
        let notifications_section = create_notifications_section(Arc::clone(&config));
        notifications_section.set_hexpand(true);
        content.append(&notifications_section);

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

fn create_sidebar_visible_section(config: Arc<Mutex<ColorConfig>>) -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 0);
    section.add_css_class("settings-section");

    let header = GtkBox::new(Orientation::Horizontal, 12);
    header.set_margin_start(18);
    header.set_margin_end(18);
    header.set_margin_top(18);
    header.set_margin_bottom(12);
    header.set_valign(gtk4::Align::Center);

    let section_title = Label::new(Some("Sidebar Visibility"));
    section_title.add_css_class("section-title");
    section_title.set_xalign(0.0);
    section_title.set_halign(gtk4::Align::Start);
    section_title.set_hexpand(true);
    header.append(&section_title);

    // Sidebar visibility toggle
    let current_visible = config.lock().unwrap().sidebar_visible.unwrap_or(true);
    let sidebar_toggle = Switch::new();
    sidebar_toggle.set_active(current_visible);
    sidebar_toggle.set_halign(gtk4::Align::End);
    sidebar_toggle.set_valign(gtk4::Align::Center);
    sidebar_toggle.set_hexpand(false);
    sidebar_toggle.set_vexpand(false);
    
    {
        let config = Arc::clone(&config);
        sidebar_toggle.connect_active_notify(move |toggle| {
            let enabled = toggle.is_active();
            // Reload config from disk to preserve existing settings
            let mut cfg = ColorConfig::load();
            cfg.set_sidebar_visible(enabled);
            if let Err(e) = cfg.save() {
            } else {
                // Update the shared config
                *config.lock().unwrap() = cfg.clone();
                // Wait a bit for file to be written
                std::thread::sleep(std::time::Duration::from_millis(200));
                // Notify quickshell about change
                if let Err(e) = quickshell::notify_color_change() {
                }
            }
        });
    }
    header.append(&sidebar_toggle);

    let desc = Label::new(Some("Show or hide the sidebar panel"));
    desc.add_css_class("section-description");
    desc.set_xalign(0.0);
    desc.set_margin_start(18);
    desc.set_margin_end(18);
    desc.set_margin_bottom(18);

    section.append(&header);
    section.append(&desc);

    section
}

fn create_sidebar_position_section(config: Arc<Mutex<ColorConfig>>) -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 0);
    section.add_css_class("settings-section");

    let header = GtkBox::new(Orientation::Horizontal, 12);
    header.set_margin_start(18);
    header.set_margin_end(18);
    header.set_margin_top(18);
    header.set_margin_bottom(12);
    header.set_valign(gtk4::Align::Center);

    let section_title = Label::new(Some("Sidebar Position"));
    section_title.add_css_class("section-title");
    section_title.set_xalign(0.0);
    section_title.set_halign(gtk4::Align::Start);
    section_title.set_hexpand(true);
    header.append(&section_title);

    // Button box for position selection
    let button_box = GtkBox::new(Orientation::Horizontal, 8);
    button_box.set_halign(gtk4::Align::End);
    button_box.set_hexpand(false);

    let current_pos = config.lock().unwrap().sidebar_position.clone().unwrap_or_else(|| "left".to_string());
    let is_left = current_pos == "left";
    let is_top = current_pos == "top";

    let left_button = Button::with_label("Left");
    if is_left {
        left_button.add_css_class("suggested-action");
    }
    let top_button = Button::with_label("Top");
    if is_top {
        top_button.add_css_class("suggested-action");
    }
    
    {
        let config = Arc::clone(&config);
        let top_btn = top_button.clone();
        left_button.connect_clicked(move |btn| {
            // Reload config from disk to preserve existing settings
            let mut cfg = ColorConfig::load();
            cfg.set_sidebar_position("left");
            if let Err(e) = cfg.save() {
            } else {
                // Update the shared config
                *config.lock().unwrap() = cfg.clone();
                // Update button styles
                btn.add_css_class("suggested-action");
                top_btn.remove_css_class("suggested-action");
                // Wait a bit for file to be written and synced to disk
                std::thread::sleep(std::time::Duration::from_millis(200));
                if let Err(e) = quickshell::notify_color_change() {
                }
            }
        });
    }
    button_box.append(&left_button);

    {
        let config = Arc::clone(&config);
        let left_btn = left_button.clone();
        top_button.connect_clicked(move |btn| {
            // Reload config from disk to preserve existing settings
            let mut cfg = ColorConfig::load();
            cfg.set_sidebar_position("top");
            if let Err(e) = cfg.save() {
            } else {
                // Update the shared config
                *config.lock().unwrap() = cfg.clone();
                // Update button styles
                btn.add_css_class("suggested-action");
                left_btn.remove_css_class("suggested-action");
                // Wait a bit for file to be written and synced to disk
                std::thread::sleep(std::time::Duration::from_millis(200));
                if let Err(e) = quickshell::notify_color_change() {
                }
            }
        });
    }
    button_box.append(&top_button);

    header.append(&button_box);

    let desc = Label::new(Some("Choose sidebar position: Left or Top"));
    desc.add_css_class("section-description");
    desc.set_xalign(0.0);
    desc.set_margin_start(18);
    desc.set_margin_end(18);
    desc.set_margin_bottom(18);

    section.append(&header);
    section.append(&desc);

    section
}

fn create_notifications_section(config: Arc<Mutex<ColorConfig>>) -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 0);
    section.add_css_class("settings-section");

    let header = GtkBox::new(Orientation::Horizontal, 12);
    header.set_margin_start(18);
    header.set_margin_end(18);
    header.set_margin_top(18);
    header.set_margin_bottom(12);
    header.set_valign(gtk4::Align::Center);

    let section_title = Label::new(Some("Notifications"));
    section_title.add_css_class("section-title");
    section_title.set_xalign(0.0);
    section_title.set_halign(gtk4::Align::Start);
    section_title.set_hexpand(true);
    header.append(&section_title);

    // Notifications enabled toggle
    let current_enabled = config.lock().unwrap().notifications_enabled.unwrap_or(true);
    let notifications_toggle = Switch::new();
    notifications_toggle.set_active(current_enabled);
    notifications_toggle.set_halign(gtk4::Align::End);
    notifications_toggle.set_valign(gtk4::Align::Center);
    notifications_toggle.set_hexpand(false);
    notifications_toggle.set_vexpand(false);
    
    {
        let config = Arc::clone(&config);
        notifications_toggle.connect_active_notify(move |toggle| {
            let enabled = toggle.is_active();
            // Reload config from disk to preserve existing settings
            let mut cfg = ColorConfig::load();
            cfg.set_notifications_enabled(enabled);
            if let Err(e) = cfg.save() {
            } else {
                // Update the shared config
                *config.lock().unwrap() = cfg.clone();
                // Wait a bit for file to be written
                std::thread::sleep(std::time::Duration::from_millis(200));
                // Notify quickshell about change
                if let Err(e) = quickshell::notify_color_change() {
                }
            }
        });
    }
    header.append(&notifications_toggle);

    let desc = Label::new(Some("Enable or disable notifications"));
    desc.add_css_class("section-description");
    desc.set_xalign(0.0);
    desc.set_margin_start(18);
    desc.set_margin_end(18);
    desc.set_margin_bottom(12);

    section.append(&header);
    section.append(&desc);

    // Notification sounds row
    let sounds_row = GtkBox::new(Orientation::Horizontal, 12);
    sounds_row.add_css_class("settings-row");
    sounds_row.set_margin_start(18);
    sounds_row.set_margin_end(18);
    sounds_row.set_margin_bottom(18);

    let sounds_text = GtkBox::new(Orientation::Vertical, 2);
    sounds_text.set_hexpand(true);

    let sounds_title = Label::new(Some("Notification Sounds"));
    sounds_title.add_css_class("row-title");
    sounds_title.set_xalign(0.0);
    sounds_text.append(&sounds_title);

    let sounds_desc = Label::new(Some("Play sound when notification appears"));
    sounds_desc.add_css_class("row-description");
    sounds_desc.set_xalign(0.0);
    sounds_text.append(&sounds_desc);

    sounds_row.append(&sounds_text);

    // Notification sounds toggle
    let current_sounds_enabled = config.lock().unwrap().notification_sounds_enabled.unwrap_or(true);
    let sounds_toggle = Switch::new();
    sounds_toggle.set_active(current_sounds_enabled);
    sounds_toggle.set_halign(gtk4::Align::End);
    sounds_toggle.set_valign(gtk4::Align::Center);
    sounds_toggle.set_hexpand(false);
    sounds_toggle.set_vexpand(false);
    
    {
        let config = Arc::clone(&config);
        sounds_toggle.connect_active_notify(move |toggle| {
            let enabled = toggle.is_active();
            // Reload config from disk to preserve existing settings
            let mut cfg = ColorConfig::load();
            cfg.set_notification_sounds_enabled(enabled);
            if let Err(e) = cfg.save() {
            } else {
                // Update the shared config
                *config.lock().unwrap() = cfg.clone();
                // Wait a bit for file to be written
                std::thread::sleep(std::time::Duration::from_millis(200));
                // Notify quickshell about change
                if let Err(e) = quickshell::notify_color_change() {
                }
            }
        });
    }
    sounds_row.append(&sounds_toggle);

    section.append(&sounds_row);

    section
}
