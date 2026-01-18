use gtk4::prelude::*;
use libadwaita::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationPage {
    Dashboard,
    Cpu,
    Gpu,
    Memory,
    Disk,
    Network,
    Processes,
}

pub struct Sidebar {
    widget: gtk4::Box,
    list_box: gtk4::ListBox,
}

impl Sidebar {
    pub fn new() -> Self {
        let widget = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        // Remove fixed width - let NavigationSplitView handle it
        widget.set_hexpand(false);
        widget.set_vexpand(true);
        widget.add_css_class("sidebar-container");
        
        let list_box = gtk4::ListBox::new();
        list_box.add_css_class("navigation-sidebar");
        list_box.set_selection_mode(gtk4::SelectionMode::Single);
        list_box.set_hexpand(true);
        list_box.set_vexpand(true);
        
        // Helper function to create a row
        let create_row = |title: &str| {
            let row = gtk4::ListBoxRow::builder()
                .css_classes(["sidebar-row"])
                .build();
            
            let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
            hbox.set_margin_start(12);
            hbox.set_margin_end(12);
            hbox.set_margin_top(6);
            hbox.set_margin_bottom(6);
            hbox.set_hexpand(true);
            
            let label = gtk4::Label::new(Some(title));
            label.set_halign(gtk4::Align::Start);
            label.set_hexpand(true);
            
            hbox.append(&label);
            row.set_child(Some(&hbox));
            
            row
        };
        
        // Dashboard
        let dashboard_row = create_row("Dashboard");
        list_box.append(&dashboard_row);
        
        // CPU
        let cpu_row = create_row("CPU");
        list_box.append(&cpu_row);
        
        // GPU
        let gpu_row = create_row("GPU");
        list_box.append(&gpu_row);
        
        // Memory
        let memory_row = create_row("Memory");
        list_box.append(&memory_row);
        
        // Disk
        let disk_row = create_row("Disk");
        list_box.append(&disk_row);
        
        // Network
        let network_row = create_row("Network");
        list_box.append(&network_row);
        
        // Processes
        let processes_row = create_row("Processes");
        list_box.append(&processes_row);
        
        // Select first item by default
        list_box.select_row(list_box.row_at_index(0).as_ref());
        
        widget.append(&list_box);
        
        Self { widget, list_box }
    }
    
    pub fn widget(&self) -> &gtk4::Box {
        &self.widget
    }
    
    pub fn list_box(&self) -> &gtk4::ListBox {
        &self.list_box
    }
    
    pub fn connect_row_selected<F>(&self, callback: F)
    where
        F: Fn(NavigationPage) + 'static,
    {
        self.list_box.connect_row_selected(move |_, row| {
            if let Some(row) = row {
                let index = row.index();
                let page = match index {
                    0 => NavigationPage::Dashboard,
                    1 => NavigationPage::Cpu,
                    2 => NavigationPage::Gpu,
                    3 => NavigationPage::Memory,
                    4 => NavigationPage::Disk,
                    5 => NavigationPage::Network,
                    6 => NavigationPage::Processes,
                    _ => NavigationPage::Dashboard,
                };
                callback(page);
            }
        });
    }
}
