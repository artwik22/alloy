use gtk4::prelude::*;
use libadwaita::prelude::*;
use crate::ui::sidebar::{Sidebar, NavigationPage};
use crate::data::EnhancedSystemData;
use std::rc::Rc;
use std::cell::RefCell;

use crate::ui::{
    dashboard_view::DashboardView,
    cpu_view::CpuView,
    gpu_view::GpuView,
    memory_view::MemoryView,
    disk_view::DiskView,
    network_view::NetworkView,
    process_view::ProcessView,
};

pub struct VitalsWindow {
    window: libadwaita::ApplicationWindow,
    view_stack: libadwaita::ViewStack,
    dashboard_view: Rc<RefCell<DashboardView>>,
    cpu_view: Rc<RefCell<CpuView>>,
    gpu_view: Rc<RefCell<GpuView>>,
    memory_view: Rc<RefCell<MemoryView>>,
    disk_view: Rc<RefCell<DiskView>>,
    network_view: Rc<RefCell<NetworkView>>,
    process_view: Rc<RefCell<ProcessView>>,
}

impl VitalsWindow {
    pub fn new(app: &libadwaita::Application) -> Self {
        let window = libadwaita::ApplicationWindow::builder()
            .application(app)
            .title("Vitals")
            .default_width(1200)
            .default_height(800)
            .resizable(true)
            .build();
        
        // Set minimum size constraints (responsive like fuse/index)
        // Smaller minimum for better mobile support
        window.set_size_request(700, 500);
        
        // Header bar
        let header_bar = libadwaita::HeaderBar::new();
        
        // Main content
        let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        main_box.set_hexpand(true);
        main_box.set_vexpand(true);
        
        // Navigation split view (sidebar + content)
        let split_view = libadwaita::NavigationSplitView::new();
        split_view.set_show_content(true);
        split_view.set_hexpand(true);
        split_view.set_vexpand(true);
        // Responsive sidebar widths
        split_view.set_min_sidebar_width(180.0);  // Smaller minimum for mobile
        split_view.set_max_sidebar_width(280.0);
        split_view.set_sidebar_width_fraction(0.20);
        // Collapse sidebar on small windows
        split_view.set_collapsed(false);
        
        // Sidebar
        let sidebar = Sidebar::new();
        let sidebar_page = libadwaita::NavigationPage::builder()
            .title("Navigation")
            .child(sidebar.widget())
            .build();
        split_view.set_sidebar(Some(&sidebar_page));
        
        // View stack for content
        let view_stack = libadwaita::ViewStack::new();
        view_stack.set_hexpand(true);
        view_stack.set_vexpand(true);
        
        // Create all views
        let dashboard_view = Rc::new(RefCell::new(DashboardView::new()));
        let cpu_view = Rc::new(RefCell::new(CpuView::new()));
        let gpu_view = Rc::new(RefCell::new(GpuView::new()));
        let memory_view = Rc::new(RefCell::new(MemoryView::new()));
        let disk_view = Rc::new(RefCell::new(DiskView::new()));
        let network_view = Rc::new(RefCell::new(NetworkView::new()));
        let process_view = Rc::new(RefCell::new(ProcessView::new()));
        
        // Add pages to view stack
        view_stack.add_titled(
            &dashboard_view.borrow().widget(),
            Some("dashboard"),
            "Dashboard",
        );
        view_stack.add_titled(&cpu_view.borrow().widget(), Some("cpu"), "CPU");
        view_stack.add_titled(&gpu_view.borrow().widget(), Some("gpu"), "GPU");
        view_stack.add_titled(&memory_view.borrow().widget(), Some("memory"), "Memory");
        view_stack.add_titled(&disk_view.borrow().widget(), Some("disk"), "Disk");
        view_stack.add_titled(&network_view.borrow().widget(), Some("network"), "Network");
        view_stack.add_titled(&process_view.borrow().widget(), Some("processes"), "Processes");
        
        // Wrap content in scrolled window for responsiveness
        let scrolled = gtk4::ScrolledWindow::new();
        scrolled.set_hexpand(true);
        scrolled.set_vexpand(true);
        scrolled.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
        scrolled.set_propagate_natural_width(true);
        scrolled.set_propagate_natural_height(true);
        
        // Wrap in clamp for optimal readability (responsive max-width)
        let clamp = libadwaita::Clamp::new();
        clamp.set_maximum_size(1400);
        clamp.set_tightening_threshold(600);  // Start tightening below 600px
        clamp.set_hexpand(true);
        clamp.set_vexpand(true);
        clamp.set_child(Some(&view_stack));
        
        scrolled.set_child(Some(&clamp));
        
        let content_page = libadwaita::NavigationPage::builder()
            .title("Content")
            .child(&scrolled)
            .build();
        split_view.set_content(Some(&content_page));
        
        // Connect sidebar navigation
        let view_stack_clone = view_stack.clone();
        sidebar.connect_row_selected(move |page| {
            let page_name = match page {
                NavigationPage::Dashboard => "dashboard",
                NavigationPage::Cpu => "cpu",
                NavigationPage::Gpu => "gpu",
                NavigationPage::Memory => "memory",
                NavigationPage::Disk => "disk",
                NavigationPage::Network => "network",
                NavigationPage::Processes => "processes",
            };
            view_stack_clone.set_visible_child_name(page_name);
        });
        
        main_box.append(&header_bar);
        main_box.append(&split_view);
        
        window.set_content(Some(&main_box));
        
        Self {
            window,
            view_stack,
            dashboard_view,
            cpu_view,
            gpu_view,
            memory_view,
            disk_view,
            network_view,
            process_view,
        }
    }
    
    pub fn window(&self) -> &libadwaita::ApplicationWindow {
        &self.window
    }
    
    pub fn update_data(&self, data: &EnhancedSystemData) {
        self.dashboard_view.borrow().update_data(data);
        self.cpu_view.borrow().update_data(data);
        self.gpu_view.borrow().update_data(data);
        self.memory_view.borrow().update_data(data);
        self.disk_view.borrow().update_data(data);
        self.network_view.borrow().update_data(data);
        self.process_view.borrow().update_data(data);
    }
    
    pub fn present(&self) {
        self.window.present();
    }
}
