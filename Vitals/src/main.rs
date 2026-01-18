mod app;
mod window;
mod ui;
mod data;
mod core;

use app::VitalsApp;
use gtk4::prelude::*;
use libadwaita::Application;

fn main() {
    let application = Application::builder()
        .application_id("com.alloy.Vitals")
        .build();
    
    let app = std::rc::Rc::new(std::cell::RefCell::new(VitalsApp::new(application.clone())));
    let app_clone = app.clone();
    
    application.connect_activate(move |_| {
        app_clone.borrow_mut().activate();
    });
    
    application.run();
}
