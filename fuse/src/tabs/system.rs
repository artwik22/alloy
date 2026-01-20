use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, Label, ScrolledWindow};
use std::sync::{Arc, Mutex};

use crate::core::config::ColorConfig;

pub struct SystemTab {
    widget: ScrolledWindow,
    _config: Arc<Mutex<ColorConfig>>,
}

impl SystemTab {
    pub fn new(config: Arc<Mutex<ColorConfig>>) -> Self {
        let scrolled = ScrolledWindow::new();
        let content = GtkBox::new(Orientation::Vertical, 24);
        content.set_margin_start(20);
        content.set_margin_end(20);
        content.set_margin_top(20);
        content.set_margin_bottom(20);

        let title = Label::new(Some("System Settings"));
        title.add_css_class("title");
        title.set_xalign(0.0);
        content.append(&title);

        let placeholder = Label::new(Some("System-related settings will appear here."));
        placeholder.add_css_class("placeholder");
        placeholder.set_xalign(0.0);
        content.append(&placeholder);

        scrolled.set_child(Some(&content));

        Self {
            widget: scrolled,
            _config: config,
        }
    }

    pub fn widget(&self) -> &ScrolledWindow {
        &self.widget
    }
}
