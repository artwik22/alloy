use gtk4::prelude::*;
use libadwaita::prelude::*;
use crate::ui::custom_charts::DetailedChartWidget;
use crate::data::EnhancedSystemData;
use crate::core::ColorConfig;
use std::rc::Rc;

pub struct MemoryView {
    widget: gtk4::Box,
    chart: Rc<DetailedChartWidget>,
    used_label: gtk4::Label,
    total_label: gtk4::Label,
    percent_label: gtk4::Label,
}

impl MemoryView {
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
        // Adjust for memory (slightly different hue)
        let memory_color = Self::adjust_color(accent_color, 30.0);
        
        // Title
        let title = gtk4::Label::new(Some("Memory Usage"));
        title.add_css_class("title-1");
        title.set_halign(gtk4::Align::Start);
        widget.append(&title);
        
        // Main chart
        let chart = Rc::new(DetailedChartWidget::new(
            memory_color,
            "Memory Usage".to_string(),
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
        
        let used_label = gtk4::Label::new(Some("Used: 0 MB"));
        used_label.set_halign(gtk4::Align::Start);
        stats_box.append(&used_label);
        
        let total_label = gtk4::Label::new(Some("Total: 0 MB"));
        total_label.set_halign(gtk4::Align::Start);
        stats_box.append(&total_label);
        
        let percent_label = gtk4::Label::new(Some("Usage: 0.0%"));
        percent_label.set_halign(gtk4::Align::Start);
        stats_box.append(&percent_label);
        
        stats_card.add(&stats_box);
        widget.append(&stats_card);
        
        Self {
            widget,
            chart,
            used_label,
            total_label,
            percent_label,
        }
    }
    
    pub fn widget(&self) -> gtk4::Box {
        self.widget.clone()
    }
    
    pub fn update_data(&self, data: &EnhancedSystemData) {
        // Update chart
        self.chart.add_point(data.memory.used_percent);
        
        // Update labels
        let used_mb = data.memory.used / 1024 / 1024;
        let total_mb = data.memory.total / 1024 / 1024;
        
        self.used_label.set_text(&format!("Used: {} MB", used_mb));
        self.total_label.set_text(&format!("Total: {} MB", total_mb));
        self.percent_label.set_text(&format!("Usage: {:.1}%", data.memory.used_percent));
    }
    
    // Helper function to adjust color hue
    fn adjust_color(rgb: (f64, f64, f64), hue_shift: f64) -> (f64, f64, f64) {
        let (r, g, b) = rgb;
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;
        
        if delta == 0.0 {
            return rgb;
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
        
        h = (h + hue_shift) % 360.0;
        if h < 0.0 {
            h += 360.0;
        }
        
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
