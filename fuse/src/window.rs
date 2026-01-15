use libadwaita::prelude::*;
use libadwaita::ApplicationWindow;
use gtk4::{
    Box as GtkBox, Orientation, Label, Stack, StackSwitcher,
};
use std::sync::{Arc, Mutex};

use crate::core::config::ColorConfig;
use crate::tabs::{general::GeneralTab, colors::ColorsTab, wallpapers::WallpapersTab,
                  bar::BarTab, system::SystemTab, audio::AudioTab};

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
            .title("Settings")
            .default_width(1000)
            .default_height(700)
            .resizable(true)
            .build();
        
        // Set minimum size after building
        window.set_default_size(1000, 700);
        // Set minimum size constraints
        window.set_size_request(800, 600);

        // Content area with stack
        let stack = Stack::new();
        stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
        stack.set_transition_duration(250);
        
        // Store stack reference for sidebar
        let stack_clone = stack.clone();

        // Main container
        let main_box = GtkBox::new(Orientation::Horizontal, 0);
        main_box.set_margin_start(8);
        main_box.set_margin_end(8);
        main_box.set_margin_top(8);
        main_box.set_margin_bottom(8);
        main_box.set_hexpand(true);
        main_box.set_vexpand(true);

        // Sidebar (220px width) with stack switcher
        let sidebar = create_sidebar_with_switcher(&stack_clone);
        main_box.append(&sidebar);

        // Create tabs
        let general_tab = GeneralTab::new(Arc::clone(&config));
        let colors_tab = ColorsTab::new(Arc::clone(&config));
        let wallpapers_tab = WallpapersTab::new(Arc::clone(&config));
        let bar_tab = BarTab::new(Arc::clone(&config));
        let system_tab = SystemTab::new(Arc::clone(&config));
        let audio_tab = AudioTab::new(Arc::clone(&config));

        stack.add_titled(general_tab.widget(), Some("general"), "General");
        stack.add_titled(colors_tab.widget(), Some("colors"), "Color Presets");
        stack.add_titled(wallpapers_tab.widget(), Some("wallpapers"), "Wallpapers");
        stack.add_titled(bar_tab.widget(), Some("bar"), "Bar");
        stack.add_titled(system_tab.widget(), Some("system"), "System");
        stack.add_titled(audio_tab.widget(), Some("audio"), "Audio");

        // Content area - stack directly, each tab has its own ScrolledWindow
        stack.set_hexpand(true);
        stack.set_vexpand(true);
        main_box.append(&stack);

        // AdwApplicationWindow already has a header bar, we don't need to set it
        // Just set the title
        window.set_title(Some("Settings"));

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
    let sidebar = GtkBox::new(Orientation::Vertical, 4);
    sidebar.set_size_request(220, -1);
    sidebar.add_css_class("sidebar");
    sidebar.set_margin_start(8);
    sidebar.set_margin_end(8);
    sidebar.set_margin_top(8);
    sidebar.set_margin_bottom(8);

    // Use StackSwitcher for navigation
    let switcher = StackSwitcher::new();
    switcher.set_stack(Some(stack));
    switcher.set_orientation(Orientation::Vertical);
    switcher.set_halign(gtk4::Align::Fill);
    switcher.add_css_class("sidebar-switcher");
    sidebar.append(&switcher);

    // Version text at bottom
    let version_label = Label::new(Some("v2.2.0"));
    version_label.add_css_class("version-label");
    version_label.set_margin_start(16);
    version_label.set_margin_end(16);
    version_label.set_margin_top(16);
    version_label.set_margin_bottom(16);
    sidebar.append(&version_label);

    sidebar
}
