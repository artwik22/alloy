use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, Label, ScrolledWindow, Switch, Entry};
use std::sync::{Arc, Mutex};

use crate::core::config::ColorConfig;
use crate::core::quickshell;
use crate::core::autostart;

fn schedule_notify_color_change_ms(ms: u32) {
    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(ms as u64), move || {
        let _ = quickshell::notify_color_change();
        gtk4::glib::ControlFlow::Break
    });
}

pub struct ScriptsTab {
    widget: ScrolledWindow,
    _config: Arc<Mutex<ColorConfig>>,
}

impl ScriptsTab {
    pub fn new(config: Arc<Mutex<ColorConfig>>) -> Self {
        let scrolled = ScrolledWindow::new();
        scrolled.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
        scrolled.set_overlay_scrolling(false);
        scrolled.set_hexpand(true);
        scrolled.set_vexpand(true);
        
        let content = GtkBox::new(Orientation::Vertical, 0);
        content.set_margin_start(24);
        content.set_margin_end(24);
        content.set_margin_top(24);
        content.set_margin_bottom(48);
        content.set_hexpand(true);
        content.set_vexpand(true);

        // Title
        let title = Label::new(Some("Scripts & Automation"));
        title.add_css_class("title");
        title.set_halign(gtk4::Align::Start);
        title.set_margin_bottom(24);
        content.append(&title);

        let add_group_header = |box_: &GtkBox, label: &str| {
            let l = Label::new(Some(label));
            l.add_css_class("group-header");
            l.set_halign(gtk4::Align::Start);
            box_.append(&l);
        };

        // --- Battery Monitor ---
        add_group_header(&content, "Battery Monitor");
        let battery_card = GtkBox::new(Orientation::Vertical, 0);
        battery_card.add_css_class("card");
        
        // Autostart
        battery_card.append(&create_battery_autostart_row(Arc::clone(&config)));
        // Threshold
        battery_card.append(&create_battery_threshold_row(Arc::clone(&config)));
        
        content.append(&battery_card);

        // --- Screensaver ---
        add_group_header(&content, "Screensaver");
        let screensaver_card = GtkBox::new(Orientation::Vertical, 0);
        screensaver_card.add_css_class("card");

        // Autostart
        screensaver_card.append(&create_screensaver_autostart_row(Arc::clone(&config)));
        // Timeout
        screensaver_card.append(&create_screensaver_timeout_row(Arc::clone(&config)));

        content.append(&screensaver_card);

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

// --- Helper for consistent rows ---
fn create_card_row(label: &str, widget: impl IsA<gtk4::Widget>) -> GtkBox {
    let row = GtkBox::new(Orientation::Horizontal, 12);
    row.add_css_class("card-row");
    row.set_valign(gtk4::Align::Center);

    let l = Label::new(Some(label));
    l.add_css_class("row-title");
    l.set_hexpand(true);
    l.set_halign(gtk4::Align::Start);
    
    row.append(&l);
    row.append(&widget);
    row
}

// --- Row Creators ---

fn create_battery_autostart_row(config: Arc<Mutex<ColorConfig>>) -> GtkBox {
    let switch = Switch::new();
    let current = config.lock().unwrap().scripts_autostart_battery.unwrap_or(false);
    switch.set_active(current);
    switch.set_valign(gtk4::Align::Center);

    {
        let config = config.clone();
        switch.connect_active_notify(move |s| {
            let active = s.is_active();
            // 1. Update Hyprland Autostart
            // Fetch current threshold for args
            let threshold = config.lock().unwrap().battery_threshold.unwrap_or(10);
            let _ = autostart::update_script("battery_monitor.sh", Some(threshold.to_string()), active);

            // 2. Update ColorConfig (for immediate effect via apply-settings)
            let mut cfg = ColorConfig::load();
            cfg.set_scripts_autostart_battery(active);
            if cfg.save().is_ok() {
                *config.lock().unwrap() = cfg.clone();
                // Delay slightly to let python script invoke apply-settings
                schedule_notify_color_change_ms(500);
            }
        });
    }

    create_card_row("Enable Battery Monitor", switch)
}

fn create_battery_threshold_row(config: Arc<Mutex<ColorConfig>>) -> GtkBox {
    let entry = Entry::new();
    let current = config.lock().unwrap().battery_threshold.unwrap_or(10);
    entry.set_text(&current.to_string());
    entry.set_placeholder_text(Some("10"));
    entry.set_width_chars(5);
    entry.set_valign(gtk4::Align::Center);

    entry.connect_changed(move |e| {
        let text = e.text();
        if let Ok(val) = text.parse::<u8>() {
             let mut cfg = ColorConfig::load();
             cfg.set_battery_threshold(val);
             if cfg.save().is_ok() {
                 *config.lock().unwrap() = cfg.clone();
                 schedule_notify_color_change_ms(500);
                 
                 // Update Autostart args if enabled
                 let enabled = config.lock().unwrap().scripts_autostart_battery.unwrap_or(false);
                 if enabled {
                     let _ = autostart::update_script("battery_monitor.sh", Some(val.to_string()), true);
                 }
             }
        }
    });

    create_card_row("Critical Threshold (%)", entry)
}

fn create_screensaver_autostart_row(config: Arc<Mutex<ColorConfig>>) -> GtkBox {
    let switch = Switch::new();
    let current = config.lock().unwrap().scripts_autostart_screensaver.unwrap_or(false);
    switch.set_active(current);
    switch.set_valign(gtk4::Align::Center);

    {
        let config = config.clone();
        switch.connect_active_notify(move |s| {
            let active = s.is_active();
            // 1. Update Hyprland Autostart
            let timeout = config.lock().unwrap().screensaver_timeout.unwrap_or(30);
            let _ = autostart::update_script("idle-screensaver.sh", Some(timeout.to_string()), active);

            // 2. Update ColorConfig
            let mut cfg = ColorConfig::load();
            cfg.set_scripts_autostart_screensaver(active);
            if cfg.save().is_ok() {
                *config.lock().unwrap() = cfg.clone();
                schedule_notify_color_change_ms(500);
            }
        });
    }

    create_card_row("Enable Idle Screensaver", switch)
}

fn create_screensaver_timeout_row(config: Arc<Mutex<ColorConfig>>) -> GtkBox {
    let entry = Entry::new();
    let current = config.lock().unwrap().screensaver_timeout.unwrap_or(30);
    entry.set_text(&current.to_string());
    entry.set_placeholder_text(Some("30"));
    entry.set_width_chars(5);
    entry.set_valign(gtk4::Align::Center);

    entry.connect_changed(move |e| {
        let text = e.text();
        if let Ok(val) = text.parse::<u32>() {
             let mut cfg = ColorConfig::load();
             cfg.set_screensaver_timeout(val);
             if cfg.save().is_ok() {
                 *config.lock().unwrap() = cfg.clone();
                 schedule_notify_color_change_ms(500);

                 // Update Autostart args if enabled
                 let enabled = config.lock().unwrap().scripts_autostart_screensaver.unwrap_or(false);
                 if enabled {
                     let _ = autostart::update_script("idle-screensaver.sh", Some(val.to_string()), true);
                 }
             }
        }
    });

    create_card_row("Idle Timeout (seconds)", entry)
}
