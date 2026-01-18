use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, Label, ScrolledWindow, Scale, Button, Separator};
use std::sync::{Arc, Mutex};
use std::process::Command;

use crate::core::config::ColorConfig;

pub struct AudioTab {
    widget: ScrolledWindow,
    _config: Arc<Mutex<ColorConfig>>,
}

impl AudioTab {
    pub fn new(config: Arc<Mutex<ColorConfig>>) -> Self {
        let scrolled = ScrolledWindow::new();
        // GNOME spacing: 24px section gap, 18px container margins
        let content = GtkBox::new(Orientation::Vertical, 18);
        content.set_margin_start(12);
        content.set_margin_end(12);
        content.set_margin_top(12);
        content.set_margin_bottom(12);

        let title = Label::new(Some("Audio Settings"));
        title.add_css_class("title");
        title.set_xalign(0.0);
        content.append(&title);

        // Default Output Device
        let output_section = create_audio_device_section("Output", "󰓃", true);
        content.append(&output_section);

        // Default Input Device
        let input_section = create_audio_device_section("Input", "󰍬", false);
        content.append(&input_section);

        // Separator - GNOME margins
        let separator = Separator::new(Orientation::Horizontal);
        separator.set_margin_start(12);
        separator.set_margin_end(12);
        separator.set_margin_top(18);
        separator.set_margin_bottom(18);
        content.append(&separator);

        // All Output Devices
        let all_outputs_section = create_all_devices_section("All Output Devices", "󰓃", true);
        content.append(&all_outputs_section);

        // Separator - GNOME margins
        let separator2 = Separator::new(Orientation::Horizontal);
        separator2.set_margin_start(12);
        separator2.set_margin_end(12);
        separator2.set_margin_top(18);
        separator2.set_margin_bottom(18);
        content.append(&separator2);

        // All Input Devices
        let all_inputs_section = create_all_devices_section("All Input Devices", "󰍬", false);
        content.append(&all_inputs_section);

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

fn create_audio_device_section(title: &str, icon: &str, is_output: bool) -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 20);

    let header = GtkBox::new(Orientation::Horizontal, 12);
    
    let icon_label = Label::new(Some(icon));
    icon_label.set_margin_end(12);
    header.append(&icon_label);

    let text_box = GtkBox::new(Orientation::Vertical, 4);
    text_box.set_hexpand(true);

    let title_label = Label::new(Some(title));
    title_label.add_css_class("audio-device-title");
    title_label.set_xalign(0.0);
    text_box.append(&title_label);

    let device_name = if is_output {
        get_default_sink().unwrap_or_else(|| "No device".to_string())
    } else {
        get_default_source().unwrap_or_else(|| "No device".to_string())
    };
    let desc_label = Label::new(Some(&device_name));
    desc_label.add_css_class("audio-device-desc");
    desc_label.set_xalign(0.0);
    text_box.append(&desc_label);

    header.append(&text_box);
    section.append(&header);

    // Controls
    let controls = GtkBox::new(Orientation::Horizontal, 15);
    controls.set_margin_start(20);
    controls.set_margin_end(20);
    controls.set_margin_top(20);
    controls.set_margin_bottom(20);

    // Mute button
    let mute_button = Button::new();
    mute_button.set_icon_name(if is_output { "audio-volume-high-symbolic" } else { "audio-input-microphone-symbolic" });
    mute_button.add_css_class("mute-button");
    controls.append(&mute_button);

    // Volume slider
    let volume_box = GtkBox::new(Orientation::Horizontal, 10);
    volume_box.set_hexpand(true);

    let volume_icon = Label::new(Some("󰕾"));
    volume_icon.set_margin_end(10);
    volume_box.append(&volume_icon);

    let volume_scale = Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 1.0);
    volume_scale.set_value(50.0);
    volume_scale.set_hexpand(true);
    volume_scale.add_css_class("volume-scale");
    
    if is_output {
        volume_scale.connect_value_changed(move |scale| {
            let value = scale.value() as u32;
            set_default_sink_volume(value);
        });
    } else {
        volume_scale.connect_value_changed(move |scale| {
            let value = scale.value() as u32;
            set_default_source_volume(value);
        });
    }
    volume_box.append(&volume_scale);

    let volume_label = Label::new(Some("50%"));
    volume_label.set_size_request(40, -1);
    let volume_label_clone = volume_label.clone();
    volume_scale.connect_value_changed(move |scale| {
        volume_label_clone.set_text(&format!("{}%", scale.value() as u32));
    });
    volume_box.append(&volume_label);

    controls.append(&volume_box);
    section.append(&controls);

    section
}

fn create_all_devices_section(title: &str, icon: &str, _is_output: bool) -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 20);

    let header = GtkBox::new(Orientation::Horizontal, 12);
    
    let icon_label = Label::new(Some(icon));
    icon_label.set_margin_end(12);
    header.append(&icon_label);

    let title_label = Label::new(Some(title));
    title_label.add_css_class("audio-section-title");
    title_label.set_xalign(0.0);
    header.append(&title_label);

    section.append(&header);

    // Placeholder for device list
    let placeholder = Label::new(Some("Device list will appear here"));
    placeholder.add_css_class("placeholder");
    section.append(&placeholder);

    section
}

fn get_default_sink() -> Option<String> {
    Command::new("pactl")
        .arg("get-default-sink")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
}

fn get_default_source() -> Option<String> {
    Command::new("pactl")
        .arg("get-default-source")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
}

fn set_default_sink_volume(volume: u32) {
    let volume_pa = (volume as f64 * 65536.0 / 100.0) as u32;
    let _ = Command::new("pactl")
        .arg("set-sink-volume")
        .arg("@DEFAULT_SINK@")
        .arg(&volume_pa.to_string())
        .output();
}

fn set_default_source_volume(volume: u32) {
    let volume_pa = (volume as f64 * 65536.0 / 100.0) as u32;
    let _ = Command::new("pactl")
        .arg("set-source-volume")
        .arg("@DEFAULT_SOURCE@")
        .arg(&volume_pa.to_string())
        .output();
}
