use libadwaita::prelude::*;
use libadwaita::Application;
use gtk4::CssProvider;

use crate::window::FuseWindow;

const APP_ID: &str = "com.alloy.fuse";

pub struct FuseApp {
    app: Application,
}

impl FuseApp {
    pub fn new() -> Self {
        let app = Application::builder()
            .application_id(APP_ID)
            .build();

        app.connect_startup(|_| {
            load_css();
        });

        app.connect_activate(|app| {
            let window = FuseWindow::new(app);
            window.present();
        });

        Self { app }
    }

    pub fn run(&self) {
        self.app.run();
    }
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("resources/style.css"));

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not connect to display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
