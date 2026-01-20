use libadwaita::prelude::*;
use libadwaita::ApplicationWindow;
use gtk4::{
    Box as GtkBox, Orientation, Label, Stack, ListBox, ListBoxRow, Separator,
};
use std::sync::{Arc, Mutex};

use crate::core::config::ColorConfig;
use crate::tabs::{appearance::AppearanceTab,
                  system::SystemTab, audio::AudioTab, index::IndexTab, bluetooth::BluetoothTab, network::NetworkTab, notifications::NotificationsTab, about::AboutTab};

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
        
        // Set minimum size after building - allow smaller sizes for responsiveness
        window.set_default_size(1100, 750);
        // Set minimum size constraints - very flexible for responsiveness
        window.set_size_request(600, 400);

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
        main_box.set_homogeneous(false);

        // Sidebar (200px width) with custom layout
        let sidebar = create_custom_sidebar(&stack_clone);
        main_box.append(&sidebar);

        // Create tabs
        let appearance_tab = AppearanceTab::new(Arc::clone(&config));
        let system_tab = SystemTab::new(Arc::clone(&config));
        let audio_tab = AudioTab::new(Arc::clone(&config));
        let index_tab = IndexTab::new(Arc::clone(&config));
        let bluetooth_tab = BluetoothTab::new(Arc::clone(&config));
        let network_tab = NetworkTab::new(Arc::clone(&config));
        let notifications_tab = NotificationsTab::new(Arc::clone(&config));
        let about_tab = AboutTab::new(Arc::clone(&config));

        // Add tabs in order: Network and Bluetooth first, then separator, then others
        stack.add_titled(network_tab.widget(), Some("network"), "󰤨 Network");
        stack.add_titled(bluetooth_tab.widget(), Some("bluetooth"), "󰂯 Bluetooth");
        stack.add_titled(appearance_tab.widget(), Some("appearance"), "󰋺 Appearance");
        stack.add_titled(system_tab.widget(), Some("system"), "󰍛 System");
        stack.add_titled(audio_tab.widget(), Some("audio"), "󰕧 Audio");
        stack.add_titled(index_tab.widget(), Some("index"), "󰉋 Index");
        stack.add_titled(notifications_tab.widget(), Some("notifications"), "󰂚 Notifications");
        stack.add_titled(about_tab.widget(), Some("about"), "󰋼 About");

        // Content area - GNOME spacing (12px margins), fully responsive
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

fn create_custom_sidebar(stack: &Stack) -> GtkBox {
    // GNOME Settings style: flexible sidebar width
    let sidebar = GtkBox::new(Orientation::Vertical, 0);
    sidebar.set_size_request(-1, -1); // Fully responsive - no fixed size
    sidebar.set_hexpand(false);
    sidebar.set_vexpand(true);
    sidebar.add_css_class("sidebar");
    sidebar.set_margin_start(0);
    sidebar.set_margin_end(0);
    sidebar.set_margin_top(12);
    sidebar.set_margin_bottom(0);

    // Create ListBox for custom sidebar
    let list_box = ListBox::new();
    list_box.add_css_class("sidebar-listbox");
    list_box.set_selection_mode(gtk4::SelectionMode::Single);
    
    // Helper function to create a sidebar row
    let create_row = |name: &str, icon: &str, page_name: &str| -> (ListBoxRow, String) {
        let row = ListBoxRow::new();
        row.add_css_class("sidebar-row");
        
        let hbox = GtkBox::new(Orientation::Horizontal, 0);
        hbox.set_margin_start(12);
        hbox.set_margin_end(12);
        hbox.set_margin_top(6);
        hbox.set_margin_bottom(6);
        hbox.set_halign(gtk4::Align::Fill);
        
        // Icon label (using emoji/icon font)
        let icon_label = Label::new(Some(icon));
        icon_label.add_css_class("sidebar-icon");
        icon_label.set_margin_end(8); // Reduced spacing to 8px
        hbox.append(&icon_label);
        
        // Text label
        let label = Label::new(Some(name));
        label.set_halign(gtk4::Align::Start);
        label.set_hexpand(true);
        hbox.append(&label);
        
        row.set_child(Some(&hbox));
        
        // Connect click to switch stack page
        let stack_clone = stack.clone();
        let page_name_str = page_name.to_string();
        row.connect_activate(move |_| {
            stack_clone.set_visible_child_name(&page_name_str);
        });
        
        (row, page_name.to_string())
    };
    
    // Store page names in order (excluding separators)
    let page_names = vec!["network", "bluetooth", "appearance", "audio", "index", "notifications", "system", "about"];
    
    // Add Network and Bluetooth at the top
    let network_row = create_row("Network", "󰤨", "network").0;
    list_box.append(&network_row);
    
    let bluetooth_row = create_row("Bluetooth", "󰂯", "bluetooth").0;
    list_box.append(&bluetooth_row);
    
    // Add separator
    let separator = Separator::new(Orientation::Horizontal);
    separator.set_margin_top(8);
    separator.set_margin_bottom(8);
    separator.set_margin_start(12);
    separator.set_margin_end(12);
    let separator_row = ListBoxRow::new();
    separator_row.set_selectable(false);
    separator_row.set_activatable(false);
    separator_row.set_child(Some(&separator));
    list_box.append(&separator_row);
    
    // Add main tabs
    let appearance_row = create_row("Appearance", "󰋺", "appearance").0;
    list_box.append(&appearance_row);
    
    let audio_row = create_row("Audio", "󰕧", "audio").0;
    list_box.append(&audio_row);
    
    let index_row = create_row("Index", "󰉋", "index").0;
    list_box.append(&index_row);
    
    let notifications_row = create_row("Notifications", "󰂚", "notifications").0;
    list_box.append(&notifications_row);
    
    // Add separator before System and About
    let separator2 = Separator::new(Orientation::Horizontal);
    separator2.set_margin_top(8);
    separator2.set_margin_bottom(8);
    separator2.set_margin_start(12);
    separator2.set_margin_end(12);
    let separator_row2 = ListBoxRow::new();
    separator_row2.set_selectable(false);
    separator_row2.set_activatable(false);
    separator_row2.set_child(Some(&separator2));
    list_box.append(&separator_row2);
    
    // Add System and About at the bottom
    let system_row = create_row("System", "󰍛", "system").0;
    list_box.append(&system_row);
    
    let about_row = create_row("About", "󰋼", "about").0;
    list_box.append(&about_row);
    
    // Connect list box selection to stack using row index
    let stack_clone = stack.clone();
    let page_names_clone = page_names.clone();
    list_box.connect_row_selected(move |_list_box, row| {
        if let Some(row) = row {
            // Get the row index directly (returns i32, not Option)
            let row_index = row.index();
            // Map index to page (skip separators at index 2 and index 7)
            // Sidebar: 0=Network, 1=Bluetooth, 2=Separator, 3=Appearance, 4=Audio, 5=Index, 6=Notifications, 7=Separator, 8=System, 9=About
            let page_idx = if row_index < 2 { 
                row_index as usize
            } else if row_index < 7 {
                (row_index - 1) as usize
            } else {
                (row_index - 2) as usize
            };
            if page_idx < page_names_clone.len() {
                stack_clone.set_visible_child_name(page_names_clone[page_idx]);
            }
        }
    });
    
    sidebar.append(&list_box);
    
    sidebar
}
