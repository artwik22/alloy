use gtk4::prelude::*;
use libadwaita::Application;
use crate::system::SystemMonitor;
use crate::ui::Ui;
use crate::update_checker::UpdateChecker;
use std::sync::{Arc, mpsc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;

pub struct App {
    application: Application,
    ui: Rc<RefCell<Option<Rc<RefCell<Ui>>>>>,
    update_checker: Arc<UpdateChecker>,
}

impl App {
    pub fn new(application: Application) -> Self {
        Self {
            application,
            ui: Rc::new(RefCell::new(None)),
            update_checker: Arc::new(UpdateChecker::new()),
        }
    }
    
    pub fn activate(&mut self) {
        if self.ui.borrow().is_some() {
            return;
        }
        
        let ui = Ui::new();
        let window = ui.window().clone();
        window.set_application(Some(&self.application));
        
        // Setup data channel using mpsc
        let (tx, rx) = mpsc::channel();
        
        // Start system monitor in background thread
        let monitor = SystemMonitor::new(tx);
        monitor.start();
        
        // Store UI in Rc for polling
        let ui_rc = Rc::new(RefCell::new(ui));
        let ui_for_poll = Rc::clone(&ui_rc);
        let rx_arc = Arc::new(Mutex::new(rx));
        let rx_for_poll = Arc::clone(&rx_arc);
        
        glib::timeout_add_local(Duration::from_millis(100), move || {
            let ui_ref = ui_for_poll.borrow();
            let rx_guard = rx_for_poll.lock().unwrap();
            // Try to receive data non-blocking
            while let Ok(data) = rx_guard.try_recv() {
                ui_ref.update_data(&data);
            }
            glib::ControlFlow::Continue
        });
        
        // Check for updates asynchronously (simplified - no async for now)
        // Updates can be checked later if needed
        
        window.present();
        // Store the Rc reference
        *self.ui.borrow_mut() = Some(ui_rc);
    }
}
