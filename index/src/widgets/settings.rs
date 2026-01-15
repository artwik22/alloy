use gtk4::prelude::*;
use gtk4::{
    ApplicationWindow, Box as GtkBox, Button, Entry, Label, Notebook, Orientation, ScrolledWindow,
    Separator,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::core::{Keybind, KeybindAction, KeybindConfig};

pub struct SettingsWindow {
    window: ApplicationWindow,
    keybinds: Rc<RefCell<HashMap<KeybindAction, Keybind>>>,
}

impl SettingsWindow {
    pub fn new(parent: &gtk4::ApplicationWindow) -> Self {
        let window = if let Some(app) = parent.application() {
            ApplicationWindow::builder()
                .application(&app)
                .title("Settings")
                .default_width(600)
                .default_height(500)
                .resizable(true)
                .modal(true)
                .transient_for(parent)
                .build()
        } else {
            ApplicationWindow::builder()
                .title("Settings")
                .default_width(600)
                .default_height(500)
                .resizable(true)
                .modal(true)
                .transient_for(parent)
                .build()
        };
        
        // Apply Index styling
        window.add_css_class("settings");

        let keybinds = Rc::new(RefCell::new(KeybindConfig::load()));

        let main_box = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .margin_start(20)
            .margin_end(20)
            .margin_top(20)
            .margin_bottom(20)
            .build();

        // Create notebook for tabs
        let notebook = Notebook::new();
        notebook.set_tab_pos(gtk4::PositionType::Top);

        // Keyboard Shortcuts tab
        let shortcuts_page = Self::create_shortcuts_page(keybinds.clone(), &window);
        notebook.append_page(&shortcuts_page, Some(&Label::new(Some("Keyboard Shortcuts"))));

        main_box.append(&notebook);

        window.set_child(Some(&main_box));

        Self { window, keybinds }
    }

    fn create_shortcuts_page(keybinds: Rc<RefCell<HashMap<KeybindAction, Keybind>>>, parent_window: &ApplicationWindow) -> ScrolledWindow {
        let scrolled = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .vexpand(true)
            .hexpand(true)
            .build();

        let content_box = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .margin_start(20)
            .margin_end(20)
            .margin_top(20)
            .margin_bottom(20)
            .build();

        // Section: Navigation
        let nav_label = Label::builder()
            .css_classes(["title-4"])
            .halign(gtk4::Align::Start)
            .build();
        nav_label.set_text("Navigation");
        content_box.append(&nav_label);

        content_box.append(&Self::create_editable_shortcut_row(
            &keybinds,
            KeybindAction::Back,
            parent_window,
        ));
        content_box.append(&Self::create_editable_shortcut_row(
            &keybinds,
            KeybindAction::Forward,
            parent_window,
        ));
        content_box.append(&Self::create_editable_shortcut_row(
            &keybinds,
            KeybindAction::Up,
            parent_window,
        ));
        content_box.append(&Self::create_editable_shortcut_row(
            &keybinds,
            KeybindAction::Home,
            parent_window,
        ));

        content_box.append(&Separator::new(gtk4::Orientation::Horizontal));

        // Section: File Operations
        let file_label = Label::builder()
            .css_classes(["title-4"])
            .halign(gtk4::Align::Start)
            .build();
        file_label.set_text("File Operations");
        content_box.append(&file_label);

        content_box.append(&Self::create_editable_shortcut_row(
            &keybinds,
            KeybindAction::Copy,
            parent_window,
        ));
        content_box.append(&Self::create_editable_shortcut_row(
            &keybinds,
            KeybindAction::Cut,
            parent_window,
        ));
        content_box.append(&Self::create_editable_shortcut_row(
            &keybinds,
            KeybindAction::Paste,
            parent_window,
        ));
        content_box.append(&Self::create_editable_shortcut_row(
            &keybinds,
            KeybindAction::Delete,
            parent_window,
        ));
        content_box.append(&Self::create_editable_shortcut_row(
            &keybinds,
            KeybindAction::Rename,
            parent_window,
        ));
        content_box.append(&Self::create_editable_shortcut_row(
            &keybinds,
            KeybindAction::SelectAll,
            parent_window,
        ));

        content_box.append(&Separator::new(gtk4::Orientation::Horizontal));

        // Section: View
        let view_label = Label::builder()
            .css_classes(["title-4"])
            .halign(gtk4::Align::Start)
            .build();
        view_label.set_text("View");
        content_box.append(&view_label);

        content_box.append(&Self::create_editable_shortcut_row(
            &keybinds,
            KeybindAction::ToggleHidden,
            parent_window,
        ));
        content_box.append(&Self::create_editable_shortcut_row(
            &keybinds,
            KeybindAction::Refresh,
            parent_window,
        ));

        content_box.append(&Separator::new(gtk4::Orientation::Horizontal));

        // Section: Tools
        let tools_label = Label::builder()
            .css_classes(["title-4"])
            .halign(gtk4::Align::Start)
            .build();
        tools_label.set_text("Tools");
        content_box.append(&tools_label);

        content_box.append(&Self::create_editable_shortcut_row(
            &keybinds,
            KeybindAction::OpenWithMicro,
            parent_window,
        ));
        content_box.append(&Self::create_editable_shortcut_row(
            &keybinds,
            KeybindAction::OpenTerminal,
            parent_window,
        ));

        scrolled.set_child(Some(&content_box));
        scrolled
    }

    fn create_editable_shortcut_row(
        keybinds: &Rc<RefCell<HashMap<KeybindAction, Keybind>>>,
        action: KeybindAction,
        parent_window: &ApplicationWindow,
    ) -> GtkBox {
        let row = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(12)
            .halign(gtk4::Align::Fill)
            .build();

        let action_label = Label::builder()
            .halign(gtk4::Align::Start)
            .hexpand(true)
            .build();
        action_label.set_text(KeybindConfig::action_to_display_name(&action));

        let shortcut_entry = Entry::new();
        shortcut_entry.set_width_chars(20);
        shortcut_entry.set_editable(false);
        shortcut_entry.add_css_class("monospace");
        
        // Set initial value
        {
            let keybinds_borrow = keybinds.borrow();
            if let Some(keybind) = keybinds_borrow.get(&action) {
                shortcut_entry.set_text(&KeybindConfig::keybind_to_string(keybind));
            }
        }

        let edit_btn = Button::builder()
            .label("Edit")
            .build();

        let action_clone = action.clone();
        let keybinds_clone = keybinds.clone();
        let shortcut_entry_clone = shortcut_entry.clone();
        let parent_window_clone = parent_window.clone();
        
        edit_btn.connect_clicked(move |_| {
            Self::show_keybind_dialog(&action_clone, &keybinds_clone, &shortcut_entry_clone, Some(&parent_window_clone));
        });

        row.append(&action_label);
        row.append(&shortcut_entry);
        row.append(&edit_btn);

        row
    }

    fn show_keybind_dialog(
        action: &KeybindAction,
        keybinds: &Rc<RefCell<HashMap<KeybindAction, Keybind>>>,
        shortcut_entry: &Entry,
        parent: Option<&gtk4::ApplicationWindow>,
    ) {
        // Create a modal window for keybind editing
        let dialog = if let Some(parent_win) = parent {
            if let Some(app) = parent_win.application() {
                ApplicationWindow::builder()
                    .application(&app)
                    .title("Edit Keybind")
                    .default_width(400)
                    .default_height(200)
                    .resizable(false)
                    .modal(true)
                    .transient_for(parent_win)
                    .build()
            } else {
                ApplicationWindow::builder()
                    .title("Edit Keybind")
                    .default_width(400)
                    .default_height(200)
                    .resizable(false)
                    .modal(true)
                    .transient_for(parent_win)
                    .build()
            }
        } else {
            ApplicationWindow::builder()
                .title("Edit Keybind")
                .default_width(400)
                .default_height(200)
                .resizable(false)
                .modal(true)
                .build()
        };
        
        dialog.add_css_class("settings");

        let vbox = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .margin_start(20)
            .margin_end(20)
            .margin_top(20)
            .margin_bottom(20)
            .build();

        let label = Label::new(Some(&format!("Press keys for: {}", KeybindConfig::action_to_display_name(action))));
        vbox.append(&label);

        let entry = Entry::new();
        entry.set_placeholder_text(Some("Press keys..."));
        entry.set_editable(false);
        entry.add_css_class("monospace");
        
        // Set current value
        {
            let keybinds_borrow = keybinds.borrow();
            if let Some(keybind) = keybinds_borrow.get(action) {
                entry.set_text(&KeybindConfig::keybind_to_string(keybind));
            }
        }

        let keybinds_clone = keybinds.clone();
        let action_clone = action.clone();
        let entry_clone = entry.clone();
        let shortcut_entry_clone = shortcut_entry.clone();
        let dialog_clone = dialog.clone();
        
        let controller = gtk4::EventControllerKey::new();
        controller.connect_key_pressed(move |_, key, _, mods| {
            let mut modifiers = Vec::new();
            if mods.contains(gtk4::gdk::ModifierType::CONTROL_MASK) {
                modifiers.push("Control".to_string());
            }
            if mods.contains(gtk4::gdk::ModifierType::SHIFT_MASK) {
                modifiers.push("Shift".to_string());
            }
            if mods.contains(gtk4::gdk::ModifierType::ALT_MASK) {
                modifiers.push("Alt".to_string());
            }
            if mods.contains(gtk4::gdk::ModifierType::SUPER_MASK) {
                modifiers.push("Super".to_string());
            }

            let key_name = key.name().map(|s| s.to_string()).unwrap_or_default();
            let display = if modifiers.is_empty() {
                key_name.clone()
            } else {
                format!("{}+{}", modifiers.join("+"), key_name)
            };
            
            entry_clone.set_text(&display);
            
            // Save immediately
            let new_keybind = Keybind {
                key: key_name,
                modifiers,
            };
            
            keybinds_clone.borrow_mut().insert(action_clone.clone(), new_keybind.clone());
            shortcut_entry_clone.set_text(&KeybindConfig::keybind_to_string(&new_keybind));
            
            // Save to file
            if let Err(e) = KeybindConfig::save(&keybinds_clone.borrow()) {
                eprintln!("Failed to save keybind: {}", e);
            }
            
            dialog_clone.close();
            gtk4::glib::Propagation::Stop
        });

        entry.add_controller(controller);
        vbox.append(&entry);

        let instruction = Label::new(Some("Click in the field above and press the desired key combination"));
        instruction.add_css_class("dim-label");
        instruction.set_wrap(true);
        vbox.append(&instruction);

        dialog.set_child(Some(&vbox));
        dialog.present();
    }

    pub fn present(&self) {
        self.window.present();
    }
}
