use gtk4::prelude::*;
use libadwaita::prelude::*;
use crate::ui::custom_charts::DetailedChartWidget;
use crate::data::EnhancedSystemData;
use crate::core::ColorConfig;
use std::rc::Rc;

pub struct CpuView {
    widget: gtk4::Box,
    chart: Rc<DetailedChartWidget>,
    usage_label: gtk4::Label,
    cores_label: gtk4::Label,
    cores_container: gtk4::Box,
}

impl CpuView {
    pub fn new() -> Self {
        let widget = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
        // Responsive margins
        widget.set_margin_top(12);
        widget.set_margin_bottom(12);
        widget.set_margin_start(12);
        widget.set_margin_end(12);
        widget.set_hexpand(true);
        widget.set_vexpand(true);
        
        // Load colors from config
        let config = ColorConfig::load();
        let accent_color = config.accent_rgb();
        
        // Title
        let title = gtk4::Label::new(Some("CPU Usage"));
        title.add_css_class("title-1");
        title.set_halign(gtk4::Align::Start);
        widget.append(&title);
        
        // Main chart
        let chart = Rc::new(DetailedChartWidget::new(
            accent_color,
            "CPU Usage".to_string(),
        ));
        widget.append(chart.widget());
        
        // Statistics card
        let stats_card = libadwaita::PreferencesGroup::new();
        let stats_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
        stats_box.set_margin_top(16);
        stats_box.set_margin_bottom(16);
        stats_box.set_margin_start(16);
        stats_box.set_margin_end(16);
        
        let stats_title = gtk4::Label::new(Some("Statistics"));
        stats_title.add_css_class("title-3");
        stats_title.set_halign(gtk4::Align::Start);
        stats_box.append(&stats_title);
        
        let usage_label = gtk4::Label::new(Some("Overall Usage: 0.0%"));
        usage_label.set_halign(gtk4::Align::Start);
        stats_box.append(&usage_label);
        
        let cores_label = gtk4::Label::new(Some("Cores: 0"));
        cores_label.set_halign(gtk4::Align::Start);
        stats_box.append(&cores_label);
        
        stats_card.add(&stats_box);
        widget.append(&stats_card);
        
        // Per-core breakdown
        let cores_card = libadwaita::PreferencesGroup::new();
        let cores_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
        cores_box.set_margin_top(16);
        cores_box.set_margin_bottom(16);
        cores_box.set_margin_start(16);
        cores_box.set_margin_end(16);
        
        let cores_title = gtk4::Label::new(Some("Per-Core Usage"));
        cores_title.add_css_class("title-3");
        cores_title.set_halign(gtk4::Align::Start);
        cores_box.append(&cores_title);
        
        let cores_container = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
        cores_box.append(&cores_container);
        
        cores_card.add(&cores_box);
        widget.append(&cores_card);
        
        Self {
            widget,
            chart,
            usage_label,
            cores_label,
            cores_container,
        }
    }
    
    pub fn widget(&self) -> gtk4::Box {
        self.widget.clone()
    }
    
    pub fn update_data(&self, data: &EnhancedSystemData) {
        // Update chart
        self.chart.add_point(data.cpu.usage);
        
        // Update labels
        self.usage_label.set_text(&format!("Overall Usage: {:.1}%", data.cpu.usage));
        self.cores_label.set_text(&format!("Cores: {}", data.cpu.cores.len()));
        
        // Update per-core display
        while let Some(child) = self.cores_container.first_child() {
            self.cores_container.remove(&child);
        }
        
        for (i, &core_usage) in data.cpu.cores.iter().enumerate() {
            let core_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
            
            let core_label = gtk4::Label::new(Some(&format!("Core {}", i)));
            core_label.set_width_chars(10);
            core_label.set_halign(gtk4::Align::Start);
            core_box.append(&core_label);
            
            let progress_bar = gtk4::ProgressBar::new();
            progress_bar.set_fraction(core_usage / 100.0);
            progress_bar.set_text(Some(&format!("{:.1}%", core_usage)));
            progress_bar.set_show_text(true);
            progress_bar.set_hexpand(true);
            core_box.append(&progress_bar);
            
            self.cores_container.append(&core_box);
        }
    }
}
