mod app;
mod system;
mod ui;
mod graph;
mod update_checker;

use app::App;
use libadwaita::prelude::*;
use libadwaita::Application;

fn main() {
    let application = Application::builder()
        .application_id("com.core.SystemMonitor")
        .build();
    
    let app = std::rc::Rc::new(std::cell::RefCell::new(App::new(application.clone())));
    let app_clone = app.clone();
    
    application.connect_activate(move |_| {
        app_clone.borrow_mut().activate();
    });
    
    application.run();
}
