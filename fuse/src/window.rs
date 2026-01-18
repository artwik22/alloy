use libadwaita::prelude::*;
use libadwaita::ApplicationWindow;
use gtk4::{
    Box as GtkBox, Orientation, Label, Stack, StackSwitcher,
};
use std::sync::{Arc, Mutex};

use crate::core::config::ColorConfig;
use crate::tabs::{general::GeneralTab, look_and_feel::LookAndFeelTab, wallpapers::WallpapersTab,
                  system::SystemTab, audio::AudioTab, index::IndexTab, bluetooth::BluetoothTab};

pub struct FuseWindow {
    window: ApplicationWindow,
    _config: Arc<Mutex<ColorConfig>>,
    _stack: Stack,
}

impl FuseWindow {
    pub fn new(app: &libadwaita::Application) -> Self {
        // Load config
        let config = Arc::new(Mutex::new(ColorConfig::load()));

        let window = ApplicationWindow::builder()
            .application(app)
            .title("⚙️ Fuse Settings")
            .default_width(1100)
            .default_height(750)
            .resizable(true)
            .build();
        
        // Set minimum size after building
        window.set_default_size(1100, 750);
        // Set minimum size constraints
        window.set_size_request(800, 650);

        // Content area with stack
        let stack = Stack::new();
        stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
        stack.set_transition_duration(250);
        
        // Store stack reference for sidebar
        let stack_clone = stack.clone();

        // Main container - GNOME spacing (no gap, use margins)
        let main_box = GtkBox::new(Orientation::Horizontal, 0);
        main_box.set_margin_start(0);
        main_box.set_margin_end(0);
        main_box.set_margin_top(0);
        main_box.set_margin_bottom(0);
        main_box.set_hexpand(true);
        main_box.set_vexpand(true);

        // Sidebar (200px width) with stack switcher
        let sidebar = create_sidebar_with_switcher(&stack_clone);
        main_box.append(&sidebar);

        // Create tabs
        let general_tab = GeneralTab::new(Arc::clone(&config));
        let look_and_feel_tab = LookAndFeelTab::new(Arc::clone(&config));
        let wallpapers_tab = WallpapersTab::new(Arc::clone(&config));
        let system_tab = SystemTab::new(Arc::clone(&config));
        let audio_tab = AudioTab::new(Arc::clone(&config));
        let index_tab = IndexTab::new(Arc::clone(&config));
        let bluetooth_tab = BluetoothTab::new(Arc::clone(&config));

        stack.add_titled(general_tab.widget(), Some("general"), "General");
        stack.add_titled(look_and_feel_tab.widget(), Some("look_and_feel"), "Look & Feel");
        stack.add_titled(wallpapers_tab.widget(), Some("wallpapers"), "Wallpapers");
        stack.add_titled(system_tab.widget(), Some("system"), "System");
        stack.add_titled(audio_tab.widget(), Some("audio"), "Audio");
        stack.add_titled(index_tab.widget(), Some("index"), "Index");
        stack.add_titled(bluetooth_tab.widget(), Some("bluetooth"), "Bluetooth");

        // Content area - GNOME spacing (12px margins)
        stack.set_hexpand(true);
        stack.set_vexpand(true);
        stack.set_margin_start(0);
        stack.set_margin_end(12);
        stack.set_margin_top(12);
        stack.set_margin_bottom(12);
        main_box.append(&stack);

        // AdwApplicationWindow already has a header bar, we don't need to set it
        // Just set the title
        window.set_title(Some("⚙️ Fuse Settings"));

        window.set_content(Some(&main_box));

        Self {
            window,
            _config: config,
            _stack: stack,
        }
    }

    pub fn present(&self) {
        self.window.present();
    }
}

fn create_sidebar_with_switcher(stack: &Stack) -> GtkBox {
    // GNOME Settings style: 240px sidebar width
    let sidebar = GtkBox::new(Orientation::Vertical, 0);
    sidebar.set_size_request(240, -1);
    sidebar.add_css_class("sidebar");
    sidebar.set_margin_start(0);
    sidebar.set_margin_end(0);
    sidebar.set_margin_top(0);
    sidebar.set_margin_bottom(0);

    // Title "Settings" above tabs - GNOME style
    let title_label = Label::new(Some("Settings"));
    title_label.add_css_class("sidebar-title");
    title_label.set_xalign(0.0);
    title_label.set_halign(gtk4::Align::Start);
    sidebar.append(&title_label);

    // Use StackSwitcher for navigation
    let switcher = StackSwitcher::new();
    switcher.set_stack(Some(stack));
    switcher.set_orientation(Orientation::Vertical);
    switcher.set_halign(gtk4::Align::Fill);
    switcher.add_css_class("sidebar-switcher");
    sidebar.append(&switcher);

    // Add spacer to push version to bottom
    let spacer = GtkBox::new(Orientation::Vertical, 0);
    spacer.set_vexpand(true);
    sidebar.append(&spacer);
    
    // Version text at bottom
    let version_label = Label::new(Some("Fuse v2.2.0"));
    version_label.add_css_class("version-label");
    sidebar.append(&version_label);

    sidebar
}
