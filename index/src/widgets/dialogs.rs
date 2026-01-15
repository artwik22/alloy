use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Box as GtkBox, Button, Entry, Label, Orientation, Window};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::core::FileOperations;
use crate::widgets::FileView;

pub fn show_new_folder_dialog(window: &ApplicationWindow, current_path: &Path, file_view: &FileView) {
    let dialog = Window::builder()
        .title("New Folder")
        .transient_for(window)
        .modal(true)
        .default_width(300)
        .default_height(100)
        .resizable(false)
        .build();

    let vbox = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_start(16)
        .margin_end(16)
        .margin_top(16)
        .margin_bottom(16)
        .build();

    let label = Label::new(Some("Folder name:"));
    label.set_halign(gtk4::Align::Start);

    let entry = Entry::builder()
        .placeholder_text("New Folder")
        .activates_default(true)
        .build();

    let button_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .halign(gtk4::Align::End)
        .build();

    let cancel_btn = Button::builder().label("Cancel").build();
    let create_btn = Button::builder()
        .label("Create")
        .css_classes(["suggested-action"])
        .build();

    button_box.append(&cancel_btn);
    button_box.append(&create_btn);

    vbox.append(&label);
    vbox.append(&entry);
    vbox.append(&button_box);

    dialog.set_child(Some(&vbox));

    // Connect signals
    let dialog_weak = dialog.downgrade();
    cancel_btn.connect_clicked(move |_| {
        if let Some(d) = dialog_weak.upgrade() {
            d.close();
        }
    });

    let dialog_weak = dialog.downgrade();
    let current_path = current_path.to_path_buf();
    let file_view = file_view.clone();
    create_btn.connect_clicked(move |_| {
        let name = entry.text().to_string();
        if !name.is_empty() {
            let new_path = current_path.join(&name);
            if let Err(e) = FileOperations::create_directory(&new_path) {
                eprintln!("Failed to create folder: {}", e);
            } else {
                file_view.load_directory(&current_path);
            }
        }
        if let Some(d) = dialog_weak.upgrade() {
            d.close();
        }
    });

    dialog.present();
}

pub fn show_new_file_dialog(window: &ApplicationWindow, current_path: &Path, file_view: &FileView) {
    let dialog = Window::builder()
        .title("New File")
        .transient_for(window)
        .modal(true)
        .default_width(300)
        .default_height(100)
        .resizable(false)
        .build();

    let vbox = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_start(16)
        .margin_end(16)
        .margin_top(16)
        .margin_bottom(16)
        .build();

    let label = Label::new(Some("File name:"));
    label.set_halign(gtk4::Align::Start);

    let entry = Entry::builder()
        .placeholder_text("new_file.txt")
        .activates_default(true)
        .build();

    let button_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .halign(gtk4::Align::End)
        .build();

    let cancel_btn = Button::builder().label("Cancel").build();
    let create_btn = Button::builder()
        .label("Create")
        .css_classes(["suggested-action"])
        .build();

    button_box.append(&cancel_btn);
    button_box.append(&create_btn);

    vbox.append(&label);
    vbox.append(&entry);
    vbox.append(&button_box);

    dialog.set_child(Some(&vbox));

    // Connect signals
    let dialog_weak = dialog.downgrade();
    cancel_btn.connect_clicked(move |_| {
        if let Some(d) = dialog_weak.upgrade() {
            d.close();
        }
    });

    let dialog_weak = dialog.downgrade();
    let current_path = current_path.to_path_buf();
    let file_view = file_view.clone();
    create_btn.connect_clicked(move |_| {
        let name = entry.text().to_string();
        if !name.is_empty() {
            let new_path = current_path.join(&name);
            if let Err(e) = FileOperations::create_file(&new_path) {
                eprintln!("Failed to create file: {}", e);
            } else {
                file_view.load_directory(&current_path);
            }
        }
        if let Some(d) = dialog_weak.upgrade() {
            d.close();
        }
    });

    dialog.present();
}

pub fn show_rename_dialog(
    window: &ApplicationWindow,
    path: &Path,
    current_path: &Rc<RefCell<PathBuf>>,
    file_view: &FileView,
) {
    let dialog = Window::builder()
        .title("Rename")
        .transient_for(window)
        .modal(true)
        .default_width(300)
        .default_height(100)
        .resizable(false)
        .build();

    let vbox = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_start(16)
        .margin_end(16)
        .margin_top(16)
        .margin_bottom(16)
        .build();

    let label = Label::new(Some("New name:"));
    label.set_halign(gtk4::Align::Start);

    let current_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let entry = Entry::builder()
        .text(&current_name)
        .activates_default(true)
        .build();

    let button_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .halign(gtk4::Align::End)
        .build();

    let cancel_btn = Button::builder().label("Cancel").build();
    let rename_btn = Button::builder()
        .label("Rename")
        .css_classes(["suggested-action"])
        .build();

    button_box.append(&cancel_btn);
    button_box.append(&rename_btn);

    vbox.append(&label);
    vbox.append(&entry);
    vbox.append(&button_box);

    dialog.set_child(Some(&vbox));

    // Connect signals
    let dialog_weak = dialog.downgrade();
    cancel_btn.connect_clicked(move |_| {
        if let Some(d) = dialog_weak.upgrade() {
            d.close();
        }
    });

    let dialog_weak = dialog.downgrade();
    let path = path.to_path_buf();
    let current_path = current_path.clone();
    let file_view = file_view.clone();
    rename_btn.connect_clicked(move |_| {
        let new_name = entry.text().to_string();
        if !new_name.is_empty() {
            if let Err(e) = FileOperations::rename(&path, &new_name) {
                eprintln!("Failed to rename: {}", e);
            } else {
                let current = current_path.borrow().clone();
                file_view.load_directory(&current);
            }
        }
        if let Some(d) = dialog_weak.upgrade() {
            d.close();
        }
    });

    dialog.present();
}
