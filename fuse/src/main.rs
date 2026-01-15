mod app;
mod core;
mod tabs;
mod widgets;
mod window;

use app::FuseApp;

fn main() {
    let app = FuseApp::new();
    app.run();
}
