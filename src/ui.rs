use gtk4::prelude::*;
use libadwaita::prelude::*;
use libadwaita::{ApplicationWindow, Banner, StyleManager};
use crate::graph::GraphWidget;
use crate::system::SystemData;
use std::rc::Rc;

pub struct Ui {
    window: ApplicationWindow,
    cpu_graph: Rc<GraphWidget>,
    memory_graph: Rc<GraphWidget>,
    network_graph: Rc<GraphWidget>,
    process_list: gtk4::Box,
    process_model: gtk4::StringList,
    processes_box: gtk4::Box,
    cpu_label: gtk4::Label,
    memory_label: gtk4::Label,
    network_label: gtk4::Label,
    update_banner: Option<Banner>,
}

impl Ui {
    pub fn new() -> Self {
        let window = ApplicationWindow::builder()
            .title("System Monitor")
            .default_width(1200)
            .default_height(800)
            .build();
        
        // Force dark theme
        let style_manager = StyleManager::default();
        style_manager.set_color_scheme(libadwaita::ColorScheme::ForceDark);
        
        // Main container
        let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
        main_box.set_margin_top(10);
        main_box.set_margin_bottom(10);
        main_box.set_margin_start(10);
        main_box.set_margin_end(10);
        
        window.set_content(Some(&main_box));
        
        // Update banner (initially hidden)
        let update_banner = Banner::new("New version available!");
        update_banner.set_revealed(false);
        main_box.append(&update_banner);
        
        // Grid layout
        let grid = gtk4::Grid::new();
        grid.set_row_spacing(10);
        grid.set_column_spacing(10);
        grid.set_hexpand(true);
        grid.set_vexpand(true);
        
        // CPU section (top, full width)
        let cpu_frame = gtk4::Frame::new(Some("CPU Usage"));
        cpu_frame.set_hexpand(true);
        let cpu_box = gtk4::Box::new(gtk4::Orientation::Vertical, 5);
        cpu_frame.set_child(Some(&cpu_box));
        
        let cpu_label = gtk4::Label::new(None);
        cpu_label.add_css_class("title-2");
        cpu_box.append(&cpu_label);
        
        let cpu_graph = Rc::new(GraphWidget::new(
            (0.0, 0.8, 0.8), // Turquoise
            "CPU".to_string(),
        ));
        cpu_box.append(cpu_graph.widget());
        
        grid.attach(&cpu_frame, 0, 0, 2, 1);
        
        // Memory section (left)
        let memory_frame = gtk4::Frame::new(Some("Memory"));
        memory_frame.set_hexpand(true);
        let memory_box = gtk4::Box::new(gtk4::Orientation::Vertical, 5);
        memory_frame.set_child(Some(&memory_box));
        
        let memory_label = gtk4::Label::new(None);
        memory_label.add_css_class("title-3");
        memory_box.append(&memory_label);
        
        let memory_graph = Rc::new(GraphWidget::new(
            (1.0, 0.65, 0.0), // Yellow
            "Memory".to_string(),
        ));
        memory_box.append(memory_graph.widget());
        
        grid.attach(&memory_frame, 0, 1, 1, 1);
        
        // Network section (right)
        let network_frame = gtk4::Frame::new(Some("Network"));
        network_frame.set_hexpand(true);
        let network_box = gtk4::Box::new(gtk4::Orientation::Vertical, 5);
        network_frame.set_child(Some(&network_box));
        
        let network_label = gtk4::Label::new(None);
        network_label.add_css_class("title-3");
        network_box.append(&network_label);
        
        let network_graph = Rc::new(GraphWidget::new(
            (1.0, 0.0, 1.0), // Magenta
            "Network".to_string(),
        ));
        network_box.append(network_graph.widget());
        
        grid.attach(&network_frame, 1, 1, 1, 1);
        
        // Processes section (bottom, full width)
        let processes_frame = gtk4::Frame::new(Some("Top Processes"));
        processes_frame.set_hexpand(true);
        processes_frame.set_vexpand(true);
        
        let processes_box = gtk4::Box::new(gtk4::Orientation::Vertical, 5);
        processes_frame.set_child(Some(&processes_box));
        
        // Simplified process list using ScrolledWindow and Box
        let process_model = gtk4::StringList::new(&[]);
        
        let processes_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        
        let scrolled = gtk4::ScrolledWindow::new();
        scrolled.set_child(Some(&processes_box));
        scrolled.set_min_content_height(200);
        
        // We'll update this box directly in update_data
        let process_list = gtk4::Box::new(gtk4::Orientation::Vertical, 0); // Placeholder
        process_list.set_hexpand(true);
        process_list.set_vexpand(true);
        
        processes_box.append(&scrolled);
        
        grid.attach(&processes_frame, 0, 2, 2, 1);
        
        main_box.append(&grid);
        
        Self {
            window,
            cpu_graph,
            memory_graph,
            network_graph,
            process_list,
            process_model,
            processes_box,
            cpu_label,
            memory_label,
            network_label,
            update_banner: Some(update_banner),
        }
    }
    
    pub fn window(&self) -> &ApplicationWindow {
        &self.window
    }
    
    pub fn update_data(&self, data: &SystemData) {
        // Update CPU graph
        self.cpu_graph.add_point(data.cpu.usage);
        self.cpu_label.set_text(&format!(
            "Global: {:.1}% | Cores: {}",
            data.cpu.usage,
            data.cpu.cores.len()
        ));
        
        // Update memory graph
        self.memory_graph.add_point(data.memory.used_percent);
        let mem_used_mb = data.memory.used / 1024 / 1024;
        let mem_total_mb = data.memory.total / 1024 / 1024;
        self.memory_label.set_text(&format!(
            "Used: {} MB / {} MB ({:.1}%)",
            mem_used_mb, mem_total_mb, data.memory.used_percent
        ));
        
        // Update network graph (use received rate)
        self.network_graph.add_point(data.network.received_rate.min(1000.0) / 10.0); // Scale to 0-100
        self.network_label.set_text(&format!(
            "↓ {:.1} KB/s | ↑ {:.1} KB/s",
            data.network.received_rate,
            data.network.transmitted_rate
        ));
        
        // Update process list - clear and rebuild
        while let Some(child) = self.processes_box.first_child() {
            self.processes_box.remove(&child);
        }
        
        for process in &data.processes {
            let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
            row.set_margin_start(10);
            row.set_margin_end(10);
            row.set_margin_top(5);
            row.set_margin_bottom(5);
            
            let name_label = gtk4::Label::new(Some(&process.name));
            name_label.set_hexpand(true);
            name_label.set_halign(gtk4::Align::Start);
            
            let cpu_label = gtk4::Label::new(Some(&format!("CPU: {:.1}%", process.cpu_usage)));
            cpu_label.set_width_chars(8);
            cpu_label.set_halign(gtk4::Align::End);
            
            let mem_label = gtk4::Label::new(Some(&format!("RAM: {:.1} MB", process.memory as f64 / 1024.0 / 1024.0)));
            mem_label.set_width_chars(10);
            mem_label.set_halign(gtk4::Align::End);
            
            row.append(&name_label);
            row.append(&cpu_label);
            row.append(&mem_label);
            
            self.processes_box.append(&row);
        }
    }
    
    pub fn show_update_banner(&self, message: &str) {
        if let Some(ref banner) = self.update_banner {
            banner.set_title(message);
            banner.set_revealed(true);
        }
    }
    
    pub fn hide_update_banner(&self) {
        if let Some(ref banner) = self.update_banner {
            banner.set_revealed(false);
        }
    }
}
