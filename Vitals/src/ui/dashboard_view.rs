use gtk4::prelude::*;
use libadwaita::prelude::*;
use crate::ui::custom_charts::SparklineWidget;
use crate::data::EnhancedSystemData;
use crate::core::ColorConfig;
use std::rc::Rc;

pub struct DashboardView {
    widget: gtk4::Box,
    cpu_sparkline: Rc<SparklineWidget>,
    memory_sparkline: Rc<SparklineWidget>,
    network_sparkline: Rc<SparklineWidget>,
    cpu_label: gtk4::Label,
    memory_label: gtk4::Label,
    disk_label: gtk4::Label,
    network_label: gtk4::Label,
    gpu_label: gtk4::Label,
}

impl DashboardView {
    pub fn new() -> Self {
        let widget = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
        // Responsive margins - smaller on mobile
        widget.set_margin_top(12);
        widget.set_margin_bottom(12);
        widget.set_margin_start(12);
        widget.set_margin_end(12);
        widget.set_hexpand(true);
        widget.set_vexpand(true);
        widget.add_css_class("dashboard-container");
        
        // Load initial colors from config (will be updated dynamically)
        let config = ColorConfig::load();
        let accent_color = config.accent_rgb();
        
        // Calculate color variations for different charts
        let cpu_color = accent_color;
        let memory_color = Self::adjust_color(accent_color, 30.0); // Shift hue
        let network_color = Self::adjust_color(accent_color, -30.0); // Shift hue opposite
        
        // Title
        let title = gtk4::Label::new(Some("System Overview"));
        title.add_css_class("title-1");
        title.set_halign(gtk4::Align::Start);
        title.set_margin_bottom(12);
        widget.append(&title);
        
        // Grid for cards - responsive
        let grid = gtk4::Grid::new();
        grid.set_row_spacing(8);   // Smaller spacing
        grid.set_column_spacing(8);
        grid.set_column_homogeneous(true);
        grid.set_row_homogeneous(false);
        grid.set_hexpand(true);
        grid.set_vexpand(true);
        grid.set_halign(gtk4::Align::Fill);
        grid.set_valign(gtk4::Align::Fill);
        
        // CPU Card
        let cpu_card = libadwaita::PreferencesGroup::new();
        cpu_card.set_hexpand(true);
        let cpu_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
        cpu_box.set_margin_top(12);
        cpu_box.set_margin_bottom(12);
        cpu_box.set_margin_start(12);
        cpu_box.set_margin_end(12);
        
        let cpu_title = gtk4::Label::new(Some("CPU"));
        cpu_title.add_css_class("title-3");
        cpu_title.set_halign(gtk4::Align::Start);
        cpu_box.append(&cpu_title);
        
        let cpu_label = gtk4::Label::new(Some("0.0%"));
        cpu_label.add_css_class("title-2");
        cpu_label.set_halign(gtk4::Align::Start);
        cpu_box.append(&cpu_label);
        
        let cpu_sparkline = Rc::new(SparklineWidget::new(cpu_color));
        cpu_box.append(cpu_sparkline.widget());
        
        cpu_card.add(&cpu_box);
        grid.attach(&cpu_card, 0, 0, 1, 1);
        
        // Memory Card
        let memory_card = libadwaita::PreferencesGroup::new();
        memory_card.set_hexpand(true);
        let memory_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
        memory_box.set_margin_top(12);
        memory_box.set_margin_bottom(12);
        memory_box.set_margin_start(12);
        memory_box.set_margin_end(12);
        
        let memory_title = gtk4::Label::new(Some("Memory"));
        memory_title.add_css_class("title-3");
        memory_title.set_halign(gtk4::Align::Start);
        memory_box.append(&memory_title);
        
        let memory_label = gtk4::Label::new(Some("0.0%"));
        memory_label.add_css_class("title-2");
        memory_label.set_halign(gtk4::Align::Start);
        memory_box.append(&memory_label);
        
        let memory_sparkline = Rc::new(SparklineWidget::new(memory_color));
        memory_box.append(memory_sparkline.widget());
        
        memory_card.add(&memory_box);
        grid.attach(&memory_card, 1, 0, 1, 1);
        
        // Disk Card
        let disk_card = libadwaita::PreferencesGroup::new();
        disk_card.set_hexpand(true);
        let disk_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
        disk_box.set_margin_top(12);
        disk_box.set_margin_bottom(12);
        disk_box.set_margin_start(12);
        disk_box.set_margin_end(12);
        
        let disk_title = gtk4::Label::new(Some("Disk"));
        disk_title.add_css_class("title-3");
        disk_title.set_halign(gtk4::Align::Start);
        disk_box.append(&disk_title);
        
        let disk_label = gtk4::Label::new(Some("0.0%"));
        disk_label.add_css_class("title-2");
        disk_label.set_halign(gtk4::Align::Start);
        disk_box.append(&disk_label);
        
        let disk_info = gtk4::Label::new(Some("No disks"));
        disk_info.add_css_class("dim-label");
        disk_info.set_halign(gtk4::Align::Start);
        disk_box.append(&disk_info);
        
        disk_card.add(&disk_box);
        grid.attach(&disk_card, 0, 1, 1, 1);
        
        // Network Card
        let network_card = libadwaita::PreferencesGroup::new();
        network_card.set_hexpand(true);
        let network_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
        network_box.set_margin_top(12);
        network_box.set_margin_bottom(12);
        network_box.set_margin_start(12);
        network_box.set_margin_end(12);
        
        let network_title = gtk4::Label::new(Some("Network"));
        network_title.add_css_class("title-3");
        network_title.set_halign(gtk4::Align::Start);
        network_box.append(&network_title);
        
        let network_label = gtk4::Label::new(Some("0 KB/s"));
        network_label.add_css_class("title-2");
        network_label.set_halign(gtk4::Align::Start);
        network_box.append(&network_label);
        
        let network_sparkline = Rc::new(SparklineWidget::new(network_color));
        network_box.append(network_sparkline.widget());
        
        network_card.add(&network_box);
        grid.attach(&network_card, 1, 1, 1, 1);
        
        // GPU Card
        let gpu_card = libadwaita::PreferencesGroup::new();
        gpu_card.set_hexpand(true);
        let gpu_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
        gpu_box.set_margin_top(12);
        gpu_box.set_margin_bottom(12);
        gpu_box.set_margin_start(12);
        gpu_box.set_margin_end(12);
        
        let gpu_title = gtk4::Label::new(Some("GPU"));
        gpu_title.add_css_class("title-3");
        gpu_title.set_halign(gtk4::Align::Start);
        gpu_box.append(&gpu_title);
        
        let gpu_label = gtk4::Label::new(Some("Not Available"));
        gpu_label.add_css_class("title-3");
        gpu_label.add_css_class("dim-label");
        gpu_label.set_halign(gtk4::Align::Start);
        gpu_box.append(&gpu_label);
        
        gpu_card.add(&gpu_box);
        grid.attach(&gpu_card, 0, 2, 2, 1);
        
        widget.append(&grid);
        
        Self {
            widget,
            cpu_sparkline,
            memory_sparkline,
            network_sparkline,
            cpu_label,
            memory_label,
            disk_label,
            network_label,
            gpu_label,
        }
    }
    
    pub fn widget(&self) -> gtk4::Box {
        self.widget.clone()
    }
    
    pub fn update_data(&self, data: &EnhancedSystemData) {
        // Update CPU
        self.cpu_label.set_text(&format!("{:.1}%", data.cpu.usage));
        self.cpu_sparkline.add_point(data.cpu.usage);
        
        // Update Memory
        self.memory_label.set_text(&format!("{:.1}%", data.memory.used_percent));
        self.memory_sparkline.add_point(data.memory.used_percent);
        
        // Update Disk (show first disk or average)
        if !data.disks.is_empty() {
            let avg_usage: f64 = data.disks.iter().map(|d| d.used_percent).sum::<f64>() 
                / data.disks.len() as f64;
            self.disk_label.set_text(&format!("{:.1}%", avg_usage));
        } else {
            self.disk_label.set_text("No disks");
        }
        
        // Update Network
        let total_rate = data.network.received_rate + data.network.transmitted_rate;
        if total_rate > 1024.0 {
            self.network_label.set_text(&format!("{:.1} MB/s", total_rate / 1024.0));
        } else {
            self.network_label.set_text(&format!("{:.1} KB/s", total_rate));
        }
        self.network_sparkline.add_point((total_rate / 10.0).min(100.0));
        
        // Update GPU
        if data.gpu.available {
            self.gpu_label.set_text(&format!("{:.1}%", data.gpu.usage));
        } else {
            self.gpu_label.set_text("Not Available");
        }
    }
    
    // Helper function to adjust color hue (shift in HSV space)
    fn adjust_color(rgb: (f64, f64, f64), hue_shift: f64) -> (f64, f64, f64) {
        // Convert RGB to HSV
        let (r, g, b) = rgb;
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;
        
        if delta == 0.0 {
            return rgb; // Gray, no hue
        }
        
        let mut h = if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };
        
        if h < 0.0 {
            h += 360.0;
        }
        
        let s = if max == 0.0 { 0.0 } else { delta / max };
        let v = max;
        
        // Shift hue
        h = (h + hue_shift) % 360.0;
        if h < 0.0 {
            h += 360.0;
        }
        
        // Convert back to RGB
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;
        
        let (r1, g1, b1) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        
        (r1 + m, g1 + m, b1 + m)
    }
}
