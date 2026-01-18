use gtk4::prelude::*;
use libadwaita::prelude::*;
use crate::data::EnhancedSystemData;

pub struct GpuView {
    widget: gtk4::Box,
}

impl GpuView {
    pub fn new() -> Self {
        let widget = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
        // Responsive margins
        widget.set_margin_top(12);
        widget.set_margin_bottom(12);
        widget.set_margin_start(12);
        widget.set_margin_end(12);
        widget.set_hexpand(true);
        widget.set_vexpand(true);
        widget.set_valign(gtk4::Align::Center);
        
        // Status page for placeholder
        let status_page = libadwaita::StatusPage::builder()
            .icon_name("video-display-symbolic")
            .title("GPU Monitoring")
            .description("GPU monitoring is not yet available.\nFuture updates will include NVIDIA and AMD GPU support.")
            .build();
        
        widget.append(&status_page);
        
        Self { widget }
    }
    
    pub fn widget(&self) -> gtk4::Box {
        self.widget.clone()
    }
    
    pub fn update_data(&self, _data: &EnhancedSystemData) {
        // Placeholder - no GPU data yet
    }
}
