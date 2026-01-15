use gtk4::prelude::*;
use gtk4::{glib, Application, CssProvider};

use crate::window::IndexWindow;

const APP_ID: &str = "com.index.fileexplorer";

pub struct IndexApp {
    app: Application,
}

impl IndexApp {
    pub fn new() -> Self {
        let app = Application::builder().application_id(APP_ID).build();

        app.connect_startup(|_| {
            load_css();
        });

        app.connect_activate(|app| {
            let window = IndexWindow::new(app);
            window.present();
        });

        Self { app }
    }

    pub fn run(&self) -> glib::ExitCode {
        self.app.run()
    }
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not connect to display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
