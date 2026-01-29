use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, Label, ScrolledWindow, Switch, Button, Entry};
use std::sync::{Arc, Mutex};

use crate::core::config::ColorConfig;
use crate::core::quickshell;

fn schedule_notify_color_change_ms(ms: u32) {
    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(ms as u64), move || {
        let _ = quickshell::notify_color_change();
        gtk4::glib::ControlFlow::Break
    });
}

pub struct QuickshellTab {
    widget: ScrolledWindow,
    _config: Arc<Mutex<ColorConfig>>,
}

impl QuickshellTab {
    pub fn new(config: Arc<Mutex<ColorConfig>>) -> Self {
        let scrolled = ScrolledWindow::new();
        scrolled.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
        scrolled.set_overlay_scrolling(false);
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

        let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
        sep.set_margin_top(12);
        sep.set_margin_bottom(12);
        content.append(&sep);

        // Dashboard Position section
        let dashboard_position_section = create_dashboard_position_section(Arc::clone(&config));
        dashboard_position_section.set_hexpand(true);
        content.append(&dashboard_position_section);

        // Scaling section
        let scaling_section = create_scaling_section(Arc::clone(&config));
        scaling_section.set_hexpand(true);
        content.append(&scaling_section);

        // Dashboard tile (left) section: Battery vs Network
        let dashboard_tile_section = create_dashboard_tile_section(Arc::clone(&config));
        dashboard_tile_section.set_hexpand(true);
        content.append(&dashboard_tile_section);

        // Sidepanel content section: Calendar vs GitHub activity + username
        let sidepanel_content_section = create_sidepanel_content_section(Arc::clone(&config));
        sidepanel_content_section.set_hexpand(true);
        content.append(&sidepanel_content_section);

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
            if let Err(_e) = cfg.save() {
            } else {
                // Update the shared config
                *config.lock().unwrap() = cfg.clone();
                schedule_notify_color_change_ms(200);
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

fn create_dashboard_position_section(config: Arc<Mutex<ColorConfig>>) -> GtkBox {
    let container = GtkBox::new(gtk4::Orientation::Vertical, 0);
    container.set_margin_bottom(24);

    let header = GtkBox::new(gtk4::Orientation::Horizontal, 0);
    header.set_margin_bottom(12);
    header.set_valign(gtk4::Align::Center);

    let section_title = Label::new(Some("Dashboard Position"));
    section_title.add_css_class("section-title");
    section_title.set_xalign(0.0);
    section_title.set_halign(gtk4::Align::Start);
    section_title.set_hexpand(true);

    header.append(&section_title);
    container.append(&header);

    // Get current position
    let current_pos = {
        let guard = config.lock().unwrap();
        guard.dashboard_position.clone().unwrap_or_else(|| "right".to_string())
    };

    let button_box = GtkBox::new(gtk4::Orientation::Horizontal, 10);
    button_box.set_halign(gtk4::Align::End);

    // Create buttons: Left, Bottom, Top, Right
    let btn_left = Button::with_label("Left");
    if current_pos == "left" { btn_left.add_css_class("suggested-action"); }

    let btn_bottom = Button::with_label("Bottom");
    if current_pos == "bottom" { btn_bottom.add_css_class("suggested-action"); }

    let btn_top = Button::with_label("Top");
    if current_pos == "top" { btn_top.add_css_class("suggested-action"); }

    let btn_right = Button::with_label("Right");
    if current_pos == "right" { btn_right.add_css_class("suggested-action"); }

    // Connect Left
    {
        let config = Arc::clone(&config);
        let b_l = btn_left.clone();
        let b_r = btn_right.clone();
        let b_t = btn_top.clone();
        let b_b = btn_bottom.clone();
        btn_left.connect_clicked(move |_| {
            let mut cfg = ColorConfig::load();
            cfg.set_dashboard_position("left");
            if let Err(_e) = cfg.save() {
                // handle error
            } else {
                *config.lock().unwrap() = cfg.clone();
                b_l.add_css_class("suggested-action");
                b_r.remove_css_class("suggested-action");
                b_t.remove_css_class("suggested-action");
                b_b.remove_css_class("suggested-action");
                schedule_notify_color_change_ms(200);
            }
        });
    }

    // Connect Bottom
    {
        let config = Arc::clone(&config);
        let b_l = btn_left.clone();
        let b_r = btn_right.clone();
        let b_t = btn_top.clone();
        let b_b = btn_bottom.clone();
        btn_bottom.connect_clicked(move |_| {
            let mut cfg = ColorConfig::load();
            cfg.set_dashboard_position("bottom");
            if let Err(_e) = cfg.save() {
            } else {
                *config.lock().unwrap() = cfg.clone();
                b_b.add_css_class("suggested-action");
                b_l.remove_css_class("suggested-action");
                b_r.remove_css_class("suggested-action");
                b_t.remove_css_class("suggested-action");
                schedule_notify_color_change_ms(200);
            }
        });
    }

    // Connect Top
    {
        let config = Arc::clone(&config);
        let b_l = btn_left.clone();
        let b_r = btn_right.clone();
        let b_t = btn_top.clone();
        let b_b = btn_bottom.clone();
        btn_top.connect_clicked(move |_| {
            let mut cfg = ColorConfig::load();
            cfg.set_dashboard_position("top");
            if let Err(_e) = cfg.save() {
            } else {
                *config.lock().unwrap() = cfg.clone();
                b_t.add_css_class("suggested-action");
                b_l.remove_css_class("suggested-action");
                b_r.remove_css_class("suggested-action");
                b_b.remove_css_class("suggested-action");
                schedule_notify_color_change_ms(200);
            }
        });
    }

    // Connect Right
    {
        let config = Arc::clone(&config);
        let b_l = btn_left.clone();
        let b_r = btn_right.clone();
        let b_t = btn_top.clone();
        let b_b = btn_bottom.clone();
        btn_right.connect_clicked(move |_| {
            let mut cfg = ColorConfig::load();
            cfg.set_dashboard_position("right");
            if let Err(_e) = cfg.save() {
            } else {
                *config.lock().unwrap() = cfg.clone();
                b_r.add_css_class("suggested-action");
                b_l.remove_css_class("suggested-action");
                b_t.remove_css_class("suggested-action");
                b_b.remove_css_class("suggested-action");
                schedule_notify_color_change_ms(200);
            }
        });
    }

    button_box.append(&btn_left);
    button_box.append(&btn_bottom);
    button_box.append(&btn_top);
    button_box.append(&btn_right);

    header.append(&button_box);

    let desc = Label::new(Some("Choose dashboard position: Left, Bottom, Top, or Right"));
    desc.add_css_class("section-description");
    desc.set_xalign(0.0);
    desc.set_margin_start(18);
    desc.set_margin_bottom(12);

    container.append(&desc);

    container
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

    let section_title = Label::new(Some("SidePanel Position"));
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

    // Create buttons manually to handle closures properly
    
    let btn_left = Button::with_label("Left");
    if current_pos == "left" { btn_left.add_css_class("suggested-action"); }
    
    let btn_bottom = Button::with_label("Bottom");
    if current_pos == "bottom" { btn_bottom.add_css_class("suggested-action"); }

    let btn_top = Button::with_label("Top");
    if current_pos == "top" { btn_top.add_css_class("suggested-action"); }

    let btn_right = Button::with_label("Right");
    if current_pos == "right" { btn_right.add_css_class("suggested-action"); }

    // Connect Left
    {
        let config = Arc::clone(&config);
        let b_l = btn_left.clone();
        let b_b = btn_bottom.clone();
        let b_t = btn_top.clone();
        let b_r = btn_right.clone();
        btn_left.connect_clicked(move |_| {
            let mut cfg = ColorConfig::load();
            cfg.set_sidebar_position("left");
            if cfg.save().is_ok() {
                *config.lock().unwrap() = cfg.clone();
                b_l.add_css_class("suggested-action");
                b_b.remove_css_class("suggested-action");
                b_t.remove_css_class("suggested-action");
                b_r.remove_css_class("suggested-action");
                schedule_notify_color_change_ms(200);
            }
        });
    }

    // Connect Bottom
    {
        let config = Arc::clone(&config);
        let b_l = btn_left.clone();
        let b_b = btn_bottom.clone();
        let b_t = btn_top.clone();
        let b_r = btn_right.clone();
        btn_bottom.connect_clicked(move |_| {
            let mut cfg = ColorConfig::load();
            cfg.set_sidebar_position("bottom");
            if cfg.save().is_ok() {
                *config.lock().unwrap() = cfg.clone();
                b_l.remove_css_class("suggested-action");
                b_b.add_css_class("suggested-action");
                b_t.remove_css_class("suggested-action");
                b_r.remove_css_class("suggested-action");
                schedule_notify_color_change_ms(200);
            }
        });
    }

    // Connect Top
    {
        let config = Arc::clone(&config);
        let b_l = btn_left.clone();
        let b_b = btn_bottom.clone();
        let b_t = btn_top.clone();
        let b_r = btn_right.clone();
        btn_top.connect_clicked(move |_| {
            let mut cfg = ColorConfig::load();
            cfg.set_sidebar_position("top");
            if cfg.save().is_ok() {
                *config.lock().unwrap() = cfg.clone();
                b_l.remove_css_class("suggested-action");
                b_b.remove_css_class("suggested-action");
                b_t.add_css_class("suggested-action");
                b_r.remove_css_class("suggested-action");
                schedule_notify_color_change_ms(200);
            }
        });
    }

    // Connect Right
    {
        let config = Arc::clone(&config);
        let b_l = btn_left.clone();
        let b_b = btn_bottom.clone();
        let b_t = btn_top.clone();
        let b_r = btn_right.clone();
        btn_right.connect_clicked(move |_| {
            let mut cfg = ColorConfig::load();
            cfg.set_sidebar_position("right");
            if cfg.save().is_ok() {
                *config.lock().unwrap() = cfg.clone();
                b_l.remove_css_class("suggested-action");
                b_b.remove_css_class("suggested-action");
                b_t.remove_css_class("suggested-action");
                b_r.add_css_class("suggested-action");
                schedule_notify_color_change_ms(200);
            }
        });
    }

    button_box.append(&btn_left);
    button_box.append(&btn_bottom);
    button_box.append(&btn_top);
    button_box.append(&btn_right);

    header.append(&button_box);

    let desc = Label::new(Some("Choose SidePanel position: Left, Bottom, Top, or Right"));
    desc.add_css_class("section-description");
    desc.set_xalign(0.0);
    desc.set_margin_start(18);
    desc.set_margin_end(18);
    desc.set_margin_bottom(18);

    section.append(&header);
    section.append(&desc);

    section
}

fn create_scaling_section(config: Arc<Mutex<ColorConfig>>) -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 0);
    section.add_css_class("settings-section");

    let header = GtkBox::new(Orientation::Horizontal, 12);
    header.set_margin_start(18);
    header.set_margin_end(18);
    header.set_margin_top(18);
    header.set_margin_bottom(12);
    header.set_valign(gtk4::Align::Center);

    let section_title = Label::new(Some("Scaling"));
    section_title.add_css_class("section-title");
    section_title.set_xalign(0.0);
    section_title.set_halign(gtk4::Align::Start);
    section_title.set_hexpand(true);
    header.append(&section_title);

    // Button box for scale selection: 75%, 100%, 125%
    let button_box = GtkBox::new(Orientation::Horizontal, 8);
    button_box.set_halign(gtk4::Align::End);
    button_box.set_hexpand(false);

    let current_scale = config.lock().unwrap().ui_scale.unwrap_or(100);
    let is_75 = current_scale == 75;
    let is_100 = current_scale == 100;
    let is_125 = current_scale == 125;

    let btn_75 = Button::with_label("75%");
    if is_75 {
        btn_75.add_css_class("suggested-action");
    }
    let btn_100 = Button::with_label("100%");
    if is_100 {
        btn_100.add_css_class("suggested-action");
    }
    let btn_125 = Button::with_label("125%");
    if is_125 {
        btn_125.add_css_class("suggested-action");
    }

    {
        let config = Arc::clone(&config);
        let b100 = btn_100.clone();
        let b125 = btn_125.clone();
        btn_75.connect_clicked(move |btn| {
            let mut cfg = ColorConfig::load();
            cfg.set_ui_scale(75);
            if cfg.save().is_ok() {
                *config.lock().unwrap() = cfg.clone();
                btn.add_css_class("suggested-action");
                b100.remove_css_class("suggested-action");
                b125.remove_css_class("suggested-action");
                schedule_notify_color_change_ms(200);
            }
        });
    }
    button_box.append(&btn_75);

    {
        let config = Arc::clone(&config);
        let b75 = btn_75.clone();
        let b125 = btn_125.clone();
        btn_100.connect_clicked(move |btn| {
            let mut cfg = ColorConfig::load();
            cfg.set_ui_scale(100);
            if cfg.save().is_ok() {
                *config.lock().unwrap() = cfg.clone();
                btn.add_css_class("suggested-action");
                b75.remove_css_class("suggested-action");
                b125.remove_css_class("suggested-action");
                schedule_notify_color_change_ms(200);
            }
        });
    }
    button_box.append(&btn_100);

    {
        let config = Arc::clone(&config);
        let b75 = btn_75.clone();
        let b100 = btn_100.clone();
        btn_125.connect_clicked(move |btn| {
            let mut cfg = ColorConfig::load();
            cfg.set_ui_scale(125);
            if cfg.save().is_ok() {
                *config.lock().unwrap() = cfg.clone();
                btn.add_css_class("suggested-action");
                b75.remove_css_class("suggested-action");
                b100.remove_css_class("suggested-action");
                schedule_notify_color_change_ms(200);
            }
        });
    }
    button_box.append(&btn_125);

    header.append(&button_box);

    let desc = Label::new(Some("UI scale (75%, 100%, 125%). Quickshell: restart via run.sh to apply. Fuse and GTK apps: use it on next launch. When opening Fuse from the shell it uses current scale."));
    desc.add_css_class("section-description");
    desc.set_xalign(0.0);
    desc.set_margin_start(18);
    desc.set_margin_end(18);
    desc.set_margin_bottom(18);

    section.append(&header);
    section.append(&desc);

    section
}

fn create_dashboard_tile_section(config: Arc<Mutex<ColorConfig>>) -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 0);
    section.add_css_class("settings-section");

    let header = GtkBox::new(Orientation::Horizontal, 12);
    header.set_margin_start(18);
    header.set_margin_end(18);
    header.set_margin_top(18);
    header.set_margin_bottom(12);
    header.set_valign(gtk4::Align::Center);

    let section_title = Label::new(Some("Kafelek dashboardu (lewy)"));
    section_title.add_css_class("section-title");
    section_title.set_xalign(0.0);
    section_title.set_halign(gtk4::Align::Start);
    section_title.set_hexpand(true);
    header.append(&section_title);

    let button_box = GtkBox::new(Orientation::Horizontal, 8);
    button_box.set_halign(gtk4::Align::End);
    button_box.set_hexpand(false);

    let current = config.lock().unwrap().dashboard_tile_left.clone().unwrap_or_else(|| "battery".to_string());
    let is_battery = current == "battery";
    let is_network = current == "network";

    let btn_battery = Button::with_label("Bateria");
    if is_battery {
        btn_battery.add_css_class("suggested-action");
    }
    let btn_network = Button::with_label("Pobieranie i wysyłanie");
    if is_network {
        btn_network.add_css_class("suggested-action");
    }

    {
        let config = Arc::clone(&config);
        let btn_net = btn_network.clone();
        btn_battery.connect_clicked(move |btn| {
            let mut cfg = ColorConfig::load();
            cfg.set_dashboard_tile_left("battery");
            if cfg.save().is_ok() {
                *config.lock().unwrap() = cfg.clone();
                btn.add_css_class("suggested-action");
                btn_net.remove_css_class("suggested-action");
                schedule_notify_color_change_ms(200);
            }
        });
    }
    button_box.append(&btn_battery);

    {
        let config = Arc::clone(&config);
        let btn_bat = btn_battery.clone();
        btn_network.connect_clicked(move |btn| {
            let mut cfg = ColorConfig::load();
            cfg.set_dashboard_tile_left("network");
            if cfg.save().is_ok() {
                *config.lock().unwrap() = cfg.clone();
                btn.add_css_class("suggested-action");
                btn_bat.remove_css_class("suggested-action");
                schedule_notify_color_change_ms(200);
            }
        });
    }
    button_box.append(&btn_network);

    header.append(&button_box);

    let desc = Label::new(Some("Co wyświetlać na lewym kafelku w dashboardzie: Bateria lub pobieranie/wysyłanie sieci"));
    desc.add_css_class("section-description");
    desc.set_xalign(0.0);
    desc.set_margin_start(18);
    desc.set_margin_end(18);
    desc.set_margin_bottom(18);

    section.append(&header);
    section.append(&desc);

    section
}

fn create_sidepanel_content_section(config: Arc<Mutex<ColorConfig>>) -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 0);
    section.add_css_class("settings-section");

    let header = GtkBox::new(Orientation::Horizontal, 12);
    header.set_margin_start(18);
    header.set_margin_end(18);
    header.set_margin_top(18);
    header.set_margin_bottom(12);
    header.set_valign(gtk4::Align::Center);

    let section_title = Label::new(Some("Panel boczny – zawartość"));
    section_title.add_css_class("section-title");
    section_title.set_xalign(0.0);
    section_title.set_halign(gtk4::Align::Start);
    section_title.set_hexpand(true);
    header.append(&section_title);

    let button_box = GtkBox::new(Orientation::Horizontal, 8);
    button_box.set_halign(gtk4::Align::End);
    button_box.set_hexpand(false);

    let current = config
        .lock()
        .unwrap()
        .sidepanel_content
        .clone()
        .unwrap_or_else(|| "calendar".to_string());
    // Be tolerant to old/typo values so one tile is always selected.
    // (Otherwise both buttons can look "unselected" when the stored value doesn't match exactly.)
    let current_norm = current.trim().to_lowercase();
    let is_calendar = matches!(current_norm.as_str(), "calendar" | "cal" | "date" | "kalendarz");
    let is_github = matches!(
        current_norm.as_str(),
        "github" | "github_activity" | "github-activity" | "githubactivity"
    );

    let btn_calendar = Button::with_label("Kalendarz");
    if is_calendar {
        btn_calendar.add_css_class("suggested-action");
    }
    let btn_github = Button::with_label("GitHub activity");
    if is_github {
        btn_github.add_css_class("suggested-action");
    }

    {
        let config = Arc::clone(&config);
        let btn_github_clone = btn_github.clone();
        btn_calendar.connect_clicked(move |btn| {
            let mut cfg = ColorConfig::load();
            cfg.set_sidepanel_content("calendar");
            if cfg.save().is_ok() {
                *config.lock().unwrap() = cfg.clone();
                btn.add_css_class("suggested-action");
                btn_github_clone.remove_css_class("suggested-action");
                schedule_notify_color_change_ms(200);
            }
        });
    }
    button_box.append(&btn_calendar);

    {
        let config = Arc::clone(&config);
        let btn_calendar_clone = btn_calendar.clone();
        btn_github.connect_clicked(move |btn| {
            let mut cfg = ColorConfig::load();
            cfg.set_sidepanel_content("github");
            if cfg.save().is_ok() {
                *config.lock().unwrap() = cfg.clone();
                btn.add_css_class("suggested-action");
                btn_calendar_clone.remove_css_class("suggested-action");
                schedule_notify_color_change_ms(200);
            }
        });
    }
    button_box.append(&btn_github);

    header.append(&button_box);

    let desc = Label::new(Some(
        "Wybierz, czy w panelu bocznym pokazywać kalendarz czy aktywność GitHuba.",
    ));
    desc.add_css_class("section-description");
    desc.set_xalign(0.0);
    desc.set_margin_start(18);
    desc.set_margin_end(18);
    desc.set_margin_bottom(8);

    section.append(&header);
    section.append(&desc);

    // GitHub username row
    let username_row = GtkBox::new(Orientation::Horizontal, 8);
    username_row.set_margin_start(18);
    username_row.set_margin_end(18);
    username_row.set_margin_bottom(18);

    let username_label = Label::new(Some("GitHub username"));
    username_label.add_css_class("row-title");
    username_label.set_xalign(0.0);
    username_label.set_halign(gtk4::Align::Start);
    username_label.set_hexpand(true);
    username_row.append(&username_label);

    let username_entry = Entry::new();
    let current_username = config
        .lock()
        .unwrap()
        .github_username
        .clone()
        .unwrap_or_default();
    username_entry.set_text(&current_username);
    username_entry.set_placeholder_text(Some("nazwa użytkownika GitHub"));
    username_entry.set_hexpand(true);

    {
        let config = Arc::clone(&config);
        username_entry.connect_changed(move |entry| {
            let text = entry.text().to_string();
            let mut cfg = ColorConfig::load();
            cfg.set_github_username(&text);
            if cfg.save().is_ok() {
                *config.lock().unwrap() = cfg.clone();
                schedule_notify_color_change_ms(200);
            }
        });
    }

    username_row.append(&username_entry);
    section.append(&username_row);

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
            if let Err(_e) = cfg.save() {
            } else {
                // Update the shared config
                *config.lock().unwrap() = cfg.clone();
                schedule_notify_color_change_ms(200);
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
            if let Err(_e) = cfg.save() {
            } else {
                // Update the shared config
                *config.lock().unwrap() = cfg.clone();
                schedule_notify_color_change_ms(200);
            }
        });
    }
    sounds_row.append(&sounds_toggle);

    section.append(&sounds_row);

    section
}
