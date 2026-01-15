mod app;
mod core;
mod widgets;
mod window;

use app::IndexApp;

fn main() {
    let app = IndexApp::new();
    let _ = app.run();
}
