use gtk4::prelude::*;
use libadwaita::prelude::*;
use crate::ui::custom_charts::DualLineChartWidget;
use crate::data::EnhancedSystemData;
use crate::core::ColorConfig;
use std::rc::Rc;

pub struct NetworkView {
    widget: gtk4::Box,
    chart: Rc<DualLineChartWidget>,
    download_label: gtk4::Label,
    upload_label: gtk4::Label,
    total_received_label: gtk4::Label,
    total_transmitted_label: gtk4::Label,
}

impl NetworkView {
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
        // Different colors for download/upload
        let download_color = Self::adjust_color(accent_color, -30.0);
        let upload_color = Self::adjust_color(accent_color, 30.0);
        
        // Title
        let title = gtk4::Label::new(Some("Network Activity"));
        title.add_css_class("title-1");
        title.set_halign(gtk4::Align::Start);
        widget.append(&title);
        
        // Legend
        let legend_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 16);
        legend_box.set_margin_bottom(8);
        
        let download_legend = gtk4::Box::new(gtk4::Orientation::Horizontal, 4);
        let download_color_widget = gtk4::DrawingArea::new();
        download_color_widget.set_content_width(16);
        download_color_widget.set_content_height(16);
        download_color_widget.set_draw_func(move |_, context, _, _| {
            context.set_source_rgb(download_color.0, download_color.1, download_color.2);
            context.rectangle(0.0, 0.0, 16.0, 16.0);
            let _ = context.fill();
        });
        download_legend.append(&download_color_widget);
        let download_text = gtk4::Label::new(Some("Download"));
        download_legend.append(&download_text);
        legend_box.append(&download_legend);
        
        let upload_legend = gtk4::Box::new(gtk4::Orientation::Horizontal, 4);
        let upload_color_widget = gtk4::DrawingArea::new();
        upload_color_widget.set_content_width(16);
        upload_color_widget.set_content_height(16);
        upload_color_widget.set_draw_func(move |_, context, _, _| {
            context.set_source_rgb(upload_color.0, upload_color.1, upload_color.2);
            context.rectangle(0.0, 0.0, 16.0, 16.0);
            let _ = context.fill();
        });
        upload_legend.append(&upload_color_widget);
        let upload_text = gtk4::Label::new(Some("Upload"));
        upload_legend.append(&upload_text);
        legend_box.append(&upload_legend);
        
        widget.append(&legend_box);
        
        // Dual chart
        let chart = Rc::new(DualLineChartWidget::new(
            download_color,
            upload_color,
            "Download".to_string(),
            "Upload".to_string(),
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
        
        let download_label = gtk4::Label::new(Some("Download Rate: 0 KB/s"));
        download_label.set_halign(gtk4::Align::Start);
        stats_box.append(&download_label);
        
        let upload_label = gtk4::Label::new(Some("Upload Rate: 0 KB/s"));
        upload_label.set_halign(gtk4::Align::Start);
        stats_box.append(&upload_label);
        
        let total_received_label = gtk4::Label::new(Some("Total Received: 0 MB"));
        total_received_label.set_halign(gtk4::Align::Start);
        stats_box.append(&total_received_label);
        
        let total_transmitted_label = gtk4::Label::new(Some("Total Transmitted: 0 MB"));
        total_transmitted_label.set_halign(gtk4::Align::Start);
        stats_box.append(&total_transmitted_label);
        
        stats_card.add(&stats_box);
        widget.append(&stats_card);
        
        Self {
            widget,
            chart,
            download_label,
            upload_label,
            total_received_label,
            total_transmitted_label,
        }
    }
    
    pub fn widget(&self) -> gtk4::Box {
        self.widget.clone()
    }
    
    pub fn update_data(&self, data: &EnhancedSystemData) {
        // Update chart - scale network rates to 0-100 for visualization
        let download_scaled = (data.network.received_rate / 10.0).min(100.0);
        let upload_scaled = (data.network.transmitted_rate / 10.0).min(100.0);
        self.chart.add_points(download_scaled, upload_scaled);
        
        // Update labels
        if data.network.received_rate > 1024.0 {
            self.download_label.set_text(&format!(
                "Download Rate: {:.2} MB/s",
                data.network.received_rate / 1024.0
            ));
        } else {
            self.download_label.set_text(&format!(
                "Download Rate: {:.1} KB/s",
                data.network.received_rate
            ));
        }
        
        if data.network.transmitted_rate > 1024.0 {
            self.upload_label.set_text(&format!(
                "Upload Rate: {:.2} MB/s",
                data.network.transmitted_rate / 1024.0
            ));
        } else {
            self.upload_label.set_text(&format!(
                "Upload Rate: {:.1} KB/s",
                data.network.transmitted_rate
            ));
        }
        
        let total_received_mb = data.network.received as f64 / 1024.0 / 1024.0;
        let total_transmitted_mb = data.network.transmitted as f64 / 1024.0 / 1024.0;
        
        self.total_received_label.set_text(&format!(
            "Total Received: {:.1} MB",
            total_received_mb
        ));
        self.total_transmitted_label.set_text(&format!(
            "Total Transmitted: {:.1} MB",
            total_transmitted_mb
        ));
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
