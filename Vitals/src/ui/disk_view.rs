use gtk4::prelude::*;
use libadwaita::prelude::*;
use crate::data::EnhancedSystemData;

pub struct DiskView {
    widget: gtk4::Box,
    disks_container: gtk4::Box,
}

impl DiskView {
    pub fn new() -> Self {
        let widget = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
        // Responsive margins
        widget.set_margin_top(12);
        widget.set_margin_bottom(12);
        widget.set_margin_start(12);
        widget.set_margin_end(12);
        widget.set_hexpand(true);
        widget.set_vexpand(true);
        
        // Title
        let title = gtk4::Label::new(Some("Disk Usage"));
        title.add_css_class("title-1");
        title.set_halign(gtk4::Align::Start);
        widget.append(&title);
        
        // Disks container
        let disks_container = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
        widget.append(&disks_container);
        
        Self {
            widget,
            disks_container,
        }
    }
    
    pub fn widget(&self) -> gtk4::Box {
        self.widget.clone()
    }
    
    pub fn update_data(&self, data: &EnhancedSystemData) {
        // Clear existing disks
        while let Some(child) = self.disks_container.first_child() {
            self.disks_container.remove(&child);
        }
        
        if data.disks.is_empty() {
            let no_disks = gtk4::Label::new(Some("No disks detected"));
            no_disks.add_css_class("dim-label");
            self.disks_container.append(&no_disks);
            return;
        }
        
        for disk in &data.disks {
            let disk_card = libadwaita::PreferencesGroup::new();
            let disk_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
            disk_box.set_margin_top(16);
            disk_box.set_margin_bottom(16);
            disk_box.set_margin_start(16);
            disk_box.set_margin_end(16);
            
            // Disk name
            let name_label = gtk4::Label::new(Some(&disk.name));
            name_label.add_css_class("title-3");
            name_label.set_halign(gtk4::Align::Start);
            disk_box.append(&name_label);
            
            // Mount point
            let mount_label = gtk4::Label::new(Some(&format!("Mounted at: {}", disk.mount_point)));
            mount_label.add_css_class("dim-label");
            mount_label.set_halign(gtk4::Align::Start);
            disk_box.append(&mount_label);
            
            // Usage bar
            let usage_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
            
            let usage_progress = gtk4::ProgressBar::new();
            usage_progress.set_fraction(disk.used_percent / 100.0);
            usage_progress.set_hexpand(true);
            usage_box.append(&usage_progress);
            
            let usage_percent = gtk4::Label::new(Some(&format!("{:.1}%", disk.used_percent)));
            usage_box.append(&usage_percent);
            
            disk_box.append(&usage_box);
            
            // Space info
            let used_gb = disk.total_space.saturating_sub(disk.available_space) as f64 / 1024.0 / 1024.0 / 1024.0;
            let total_gb = disk.total_space as f64 / 1024.0 / 1024.0 / 1024.0;
            let space_label = gtk4::Label::new(Some(&format!(
                "{:.1} GB used of {:.1} GB",
                used_gb, total_gb
            )));
            space_label.set_halign(gtk4::Align::Start);
            disk_box.append(&space_label);
            
            disk_card.add(&disk_box);
            self.disks_container.append(&disk_card);
        }
    }
}
