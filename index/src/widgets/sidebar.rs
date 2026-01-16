use gtk4::prelude::*;
use gtk4::{Box as GtkBox, GestureClick, Image, Label, ListBox, ListBoxRow, Orientation, PopoverMenu, ScrolledWindow, SelectionMode};
use gtk4::gio;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::core::{DriveScanner, PinnedManager, PinnedFolder};

#[derive(Clone)]
#[allow(dead_code)]
pub struct Sidebar {
    container: GtkBox,
    list_box: ListBox,
    on_location_selected: Rc<RefCell<Option<Box<dyn Fn(PathBuf)>>>>,
    paths: Rc<RefCell<Vec<PathBuf>>>,
    pinned_start_index: Rc<RefCell<usize>>,
    pinned_end_index: Rc<RefCell<usize>>,
}

impl Sidebar {
    pub fn new() -> Self {
        let container = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .css_classes(["sidebar"])
            .vexpand(true)
            .width_request(220)
            .build();

        let scrolled = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .vexpand(true)
            .build();

        let list_box = ListBox::builder()
            .selection_mode(SelectionMode::Single)
            .css_classes(["navigation-sidebar"])
            .build();

        let paths: Rc<RefCell<Vec<PathBuf>>> = Rc::new(RefCell::new(Vec::new()));
        let pinned_start_index: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));
        let pinned_end_index: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));

        // === Pinned Section ===
        let pinned_header = Self::create_section_header("Pinned");
        list_box.append(&pinned_header);
        let pinned_start = paths.borrow().len();

        let pinned = PinnedManager::load();
        for item in &pinned {
            if item.path.exists() {
                let path = item.path.clone();
                let row = Self::create_pinned_row(&item.name, "folder-symbolic", &path, &list_box);
                list_box.append(&row);
                paths.borrow_mut().push(path);
            }
        }
        let pinned_end = paths.borrow().len();

        *pinned_start_index.borrow_mut() = pinned_start;
        *pinned_end_index.borrow_mut() = pinned_end;

        // === Devices Section ===
        let devices_header = Self::create_section_header("Devices");
        list_box.append(&devices_header);

        let drives = DriveScanner::scan();
        for drive in drives {
            let subtitle = if drive.total_size > 0 {
                Some(drive.size_display())
            } else {
                None
            };

            let row = Self::create_item_row(&drive.name, &drive.icon_name, subtitle.as_deref());
            row.add_css_class("drive-item");
            list_box.append(&row);
            paths.borrow_mut().push(drive.mount_point);
        }

        // === System Section ===
        let system_header = Self::create_section_header("System");
        list_box.append(&system_header);

        // Trash
        if let Some(trash_path) = dirs::data_local_dir().map(|p| p.join("Trash/files")) {
            let row = Self::create_item_row("Trash", "user-trash-symbolic", None);
            list_box.append(&row);
            paths.borrow_mut().push(trash_path);
        }

        // Root
        let row = Self::create_item_row("File System", "drive-harddisk-symbolic", None);
        list_box.append(&row);
        paths.borrow_mut().push(PathBuf::from("/"));

        scrolled.set_child(Some(&list_box));
        container.append(&scrolled);

        let on_location_selected: Rc<RefCell<Option<Box<dyn Fn(PathBuf)>>>> =
            Rc::new(RefCell::new(None));

        // Connect row activation - use direct index lookup
        {
            let on_location_selected_clone = on_location_selected.clone();
            let paths_clone = paths.clone();

            list_box.connect_row_activated(move |_listbox, activated_row| {
                // Skip if it's a header
                if activated_row
                    .css_classes()
                    .iter()
                    .any(|c| c == "sidebar-section-header")
                {
                    return;
                }

                // Get row index and map to paths array
                let row_index = activated_row.index();
                
                // Build index map (skip headers)
                let mut item_indices = Vec::new();
                let mut current = _listbox.first_child();
                
                while let Some(child) = current {
                    if let Some(r) = child.downcast_ref::<ListBoxRow>() {
                        if !r.css_classes().iter().any(|c| c == "sidebar-section-header") {
                            item_indices.push(r.index());
                        }
                    }
                    current = child.next_sibling();
                }

                // Find position in item_indices that matches row_index
                if let Some(array_index) = item_indices.iter().position(|&idx| idx == row_index) {
                    let paths = paths_clone.borrow();
                    if let Some(path) = paths.get(array_index) {
                        if let Some(ref callback) = *on_location_selected_clone.borrow() {
                            callback(path.clone());
                        }
                    }
                }
            });
        }

        Self {
            container,
            list_box,
            on_location_selected,
            paths,
            pinned_start_index,
            pinned_end_index,
        }
    }

    fn create_section_header(title: &str) -> ListBoxRow {
        let row = ListBoxRow::builder()
            .selectable(false)
            .activatable(false)
            .css_classes(["sidebar-section-header"])
            .build();

        let label = Label::builder()
            .label(title)
            .halign(gtk4::Align::Start)
            .margin_start(16)
            .margin_top(10)
            .margin_bottom(6)
            .css_classes(["sidebar-section-header"])
            .build();

        row.set_child(Some(&label));
        row
    }

    fn create_item_row(name: &str, icon_name: &str, subtitle: Option<&str>) -> ListBoxRow {
        let row = ListBoxRow::new();

        let hbox = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(10)
            .margin_start(12)
            .margin_end(12)
            .margin_top(4)
            .margin_bottom(4)
            .build();

        let icon = Image::builder()
            .icon_name(icon_name)
            .pixel_size(18)
            .build();

        let vbox = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(2)
            .hexpand(true)
            .build();

        let name_label = Label::builder()
            .label(name)
            .halign(gtk4::Align::Start)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .build();

        vbox.append(&name_label);

        if let Some(sub) = subtitle {
            let sub_label = Label::builder()
                .label(sub)
                .halign(gtk4::Align::Start)
                .css_classes(["dim-label"])
                .build();
            vbox.append(&sub_label);
        }

        hbox.append(&icon);
        hbox.append(&vbox);
        row.set_child(Some(&hbox));
        row
    }

    fn get_folder_icon(path: &PathBuf) -> String {
        // Check if it's a standard directory
        if let Some(home) = dirs::home_dir() {
            if path == &home {
                return "user-home-symbolic".to_string();
            }
            if let Some(doc_dir) = dirs::document_dir() {
                if path == &doc_dir {
                    return "folder-documents-symbolic".to_string();
                }
            }
            if let Some(download_dir) = dirs::download_dir() {
                if path == &download_dir {
                    return "folder-download-symbolic".to_string();
                }
            }
            if let Some(pic_dir) = dirs::picture_dir() {
                if path == &pic_dir {
                    return "folder-pictures-symbolic".to_string();
                }
            }
            if let Some(music_dir) = dirs::audio_dir() {
                if path == &music_dir {
                    return "folder-music-symbolic".to_string();
                }
            }
            if let Some(video_dir) = dirs::video_dir() {
                if path == &video_dir {
                    return "folder-videos-symbolic".to_string();
                }
            }
        }
        "folder-symbolic".to_string()
    }

    fn create_pinned_row(name: &str, icon_name: &str, path: &PathBuf, list_box: &ListBox) -> ListBoxRow {
        let row = ListBoxRow::new();

        let hbox = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(10)
            .margin_start(12)
            .margin_end(12)
            .margin_top(4)
            .margin_bottom(4)
            .build();

        let icon = Image::builder()
            .icon_name(icon_name)
            .pixel_size(18)
            .build();

        let vbox = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(2)
            .hexpand(true)
            .build();

        let name_label = Label::builder()
            .label(name)
            .halign(gtk4::Align::Start)
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .build();

        vbox.append(&name_label);

        hbox.append(&icon);
        hbox.append(&vbox);

        row.set_child(Some(&hbox));
        row.add_css_class("pinned-item");

        // Context menu for unpin
        {
            let path_for_unpin = path.clone();
            let row_clone = row.clone();
            let list_box_clone = list_box.clone();
            
            let gesture = GestureClick::builder().button(3).build();
            gesture.connect_pressed(move |_gesture, _, x, y| {
                let menu = gio::Menu::new();
                menu.append(Some("Unpin"), Some("pinned.unpin"));

                let popover = PopoverMenu::from_model(Some(&menu));
                popover.set_parent(&list_box_clone);
                popover.set_pointing_to(Some(&gtk4::gdk::Rectangle::new(
                    x as i32,
                    y as i32,
                    1,
                    1,
                )));

                // Create action group
                let action_group = gio::SimpleActionGroup::new();

                // Unpin action
                {
                    let path_to_unpin = path_for_unpin.clone();
                    let action = gio::SimpleAction::new("unpin", None);
                    action.connect_activate(move |_, _| {
                        if let Err(e) = PinnedManager::remove(&path_to_unpin) {
                            eprintln!("Failed to unpin: {}", e);
                        }
                        // Note: Sidebar refresh would need to be implemented
                        // For now, requires restart to see changes
                    });
                    action_group.add_action(&action);
                }

                popover.insert_action_group("pinned", Some(&action_group));
                popover.popup();
            });

            row.add_controller(gesture);
        }

        row
    }

    pub fn container(&self) -> &GtkBox {
        &self.container
    }

    pub fn connect_location_selected<F: Fn(PathBuf) + 'static>(&self, callback: F) {
        *self.on_location_selected.borrow_mut() = Some(Box::new(callback));
    }

    pub fn refresh(&self) {
        // TODO: Implement refresh to reload pinned items
        // For now, requires restart
    }

    #[allow(dead_code)]
    pub fn refresh_drives(&self) {
        // Could be implemented to refresh drive list
        // For now, requires restart to see new drives
    }
}
