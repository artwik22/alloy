use gtk4::prelude::*;
use libadwaita::prelude::*;
use crate::data::EnhancedSystemData;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ProcessView {
    widget: gtk4::Box,
    search_entry: gtk4::SearchEntry,
    list_box: gtk4::ListBox,
    processes_data: Rc<RefCell<Vec<ProcessRow>>>,
}

struct ProcessRow {
    name: String,
    pid: u32,
    cpu: f64,
    memory: f64,
}

impl ProcessView {
    pub fn new() -> Self {
        let widget = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        widget.set_hexpand(true);
        widget.set_vexpand(true);
        
        // Search bar
        let search_bar = gtk4::SearchBar::new();
        let search_entry = gtk4::SearchEntry::new();
        search_entry.set_hexpand(true);
        search_bar.set_child(Some(&search_entry));
        search_bar.set_search_mode(true);
        widget.append(&search_bar);
        
        // Scrolled window for process list
        let scrolled = gtk4::ScrolledWindow::new();
        scrolled.set_vexpand(true);
        scrolled.set_hexpand(true);
        
        let list_box = gtk4::ListBox::new();
        list_box.set_selection_mode(gtk4::SelectionMode::Single);
        list_box.add_css_class("boxed-list");
        
        // Header row
        let header_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        header_row.set_margin_top(12);
        header_row.set_margin_bottom(8);
        header_row.set_margin_start(12);
        header_row.set_margin_end(12);
        
        let name_header = gtk4::Label::new(Some("Name"));
        name_header.add_css_class("heading");
        name_header.set_width_chars(30);
        name_header.set_halign(gtk4::Align::Start);
        header_row.append(&name_header);
        
        let pid_header = gtk4::Label::new(Some("PID"));
        pid_header.add_css_class("heading");
        pid_header.set_width_chars(8);
        pid_header.set_halign(gtk4::Align::End);
        header_row.append(&pid_header);
        
        let cpu_header = gtk4::Label::new(Some("CPU %"));
        cpu_header.add_css_class("heading");
        cpu_header.set_width_chars(10);
        cpu_header.set_halign(gtk4::Align::End);
        header_row.append(&cpu_header);
        
        let mem_header = gtk4::Label::new(Some("Memory %"));
        mem_header.add_css_class("heading");
        mem_header.set_width_chars(12);
        mem_header.set_halign(gtk4::Align::End);
        header_row.append(&mem_header);
        
        widget.append(&header_row);
        
        scrolled.set_child(Some(&list_box));
        widget.append(&scrolled);
        
        let processes_data = Rc::new(RefCell::new(Vec::<ProcessRow>::new()));
        
        // Setup search filtering
        let list_box_clone = list_box.clone();
        let processes_data_clone = processes_data.clone();
        search_entry.connect_search_changed(move |entry| {
            let search_text = entry.text().to_lowercase();
            let search_text_clone = search_text.clone();
            let processes_clone = processes_data_clone.clone();
            list_box_clone.set_filter_func(move |row: &gtk4::ListBoxRow| {
                let index = row.index();
                if index < 0 {
                    return false;
                }
                let processes = processes_clone.borrow();
                if let Some(process) = processes.get(index as usize) {
                    if search_text_clone.is_empty() {
                        return true;
                    }
                    process.name.to_lowercase().contains(&search_text_clone)
                        || process.pid.to_string().contains(&search_text_clone)
                } else {
                    false
                }
            });
        });
        
        Self {
            widget,
            search_entry,
            list_box,
            processes_data,
        }
    }
    
    pub fn widget(&self) -> gtk4::Box {
        self.widget.clone()
    }
    
    pub fn update_data(&self, data: &EnhancedSystemData) {
        // Clear existing rows
        while let Some(child) = self.list_box.first_child() {
            self.list_box.remove(&child);
        }
        
        // Update processes data
        let mut processes_vec = Vec::new();
        
        // Sort processes by CPU usage and limit to top 100
        let mut sorted_processes = data.processes.clone();
        sorted_processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
        sorted_processes.truncate(100);
        
        for process in sorted_processes {
            processes_vec.push(ProcessRow {
                name: process.name.clone(),
                pid: process.pid,
                cpu: process.cpu_usage,
                memory: process.memory_percent,
            });
            
            let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
            row_box.set_margin_top(8);
            row_box.set_margin_bottom(8);
            row_box.set_margin_start(12);
            row_box.set_margin_end(12);
            
            let name_label = gtk4::Label::new(Some(&process.name));
            name_label.set_width_chars(30);
            name_label.set_halign(gtk4::Align::Start);
            name_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
            name_label.set_xalign(0.0);
            row_box.append(&name_label);
            
            let pid_label = gtk4::Label::new(Some(&process.pid.to_string()));
            pid_label.set_width_chars(8);
            pid_label.set_halign(gtk4::Align::End);
            row_box.append(&pid_label);
            
            let cpu_label = gtk4::Label::new(Some(&format!("{:.1}", process.cpu_usage)));
            cpu_label.set_width_chars(10);
            cpu_label.set_halign(gtk4::Align::End);
            row_box.append(&cpu_label);
            
            let mem_label = gtk4::Label::new(Some(&format!("{:.1}", process.memory_percent)));
            mem_label.set_width_chars(12);
            mem_label.set_halign(gtk4::Align::End);
            row_box.append(&mem_label);
            
            self.list_box.append(&row_box);
        }
        
        *self.processes_data.borrow_mut() = processes_vec;
        
        // Reapply filter if search is active
        if !self.search_entry.text().is_empty() {
            self.list_box.invalidate_filter();
        }
    }
}
