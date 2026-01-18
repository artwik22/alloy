mod app;
mod core;
mod widgets;
mod window;

use app::IndexApp;
use libadwaita as adw;

fn main() {
    // Initialize libadwaita
    adw::init().expect("Failed to initialize libadwaita");
    
    let app = IndexApp::new();
    let _ = app.run();
}
