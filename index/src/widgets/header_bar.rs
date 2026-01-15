use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, HeaderBar as GtkHeaderBar, Orientation, SearchEntry};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
#[allow(dead_code)]
pub struct HeaderBar {
    container: GtkHeaderBar,
    back_btn: Button,
    forward_btn: Button,
    up_btn: Button,
    home_btn: Button,
    search_entry: SearchEntry,
    new_folder_btn: Button,
    new_file_btn: Button,
    settings_btn: Button,

    on_back: Rc<RefCell<Option<Box<dyn Fn()>>>>,
    on_forward: Rc<RefCell<Option<Box<dyn Fn()>>>>,
    on_up: Rc<RefCell<Option<Box<dyn Fn()>>>>,
    on_home: Rc<RefCell<Option<Box<dyn Fn()>>>>,
    on_search: Rc<RefCell<Option<Box<dyn Fn(String)>>>>,
    on_new_folder: Rc<RefCell<Option<Box<dyn Fn()>>>>,
    on_new_file: Rc<RefCell<Option<Box<dyn Fn()>>>>,
    on_settings: Rc<RefCell<Option<Box<dyn Fn()>>>>,
}

impl HeaderBar {
    pub fn new() -> Self {
        let container = GtkHeaderBar::new();
        container.set_show_title_buttons(false);

        // Navigation buttons - linked group
        let nav_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(0)
            .css_classes(["linked"])
            .build();

        let back_btn = Button::builder()
            .icon_name("go-previous-symbolic")
            .tooltip_text("Back")
            .build();

        let forward_btn = Button::builder()
            .icon_name("go-next-symbolic")
            .tooltip_text("Forward")
            .build();

        nav_box.append(&back_btn);
        nav_box.append(&forward_btn);

        container.pack_start(&nav_box);

        // Path navigation buttons
        let path_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(4)
            .margin_start(8)
            .build();

        let up_btn = Button::builder()
            .icon_name("go-up-symbolic")
            .tooltip_text("Go to parent folder")
            .build();

        let home_btn = Button::builder()
            .icon_name("go-home-symbolic")
            .tooltip_text("Go to home folder")
            .build();

        path_box.append(&up_btn);
        path_box.append(&home_btn);

        container.pack_start(&path_box);

        // Search entry - centered
        let search_entry = SearchEntry::builder()
            .placeholder_text("Search files...")
            .width_chars(30)
            .max_width_chars(50)
            .build();

        container.set_title_widget(Some(&search_entry));

        // Action buttons - right side
        let action_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(0)
            .css_classes(["linked"])
            .build();

        let new_folder_btn = Button::builder()
            .icon_name("folder-new-symbolic")
            .tooltip_text("New Folder")
            .build();

        let new_file_btn = Button::builder()
            .icon_name("document-new-symbolic")
            .tooltip_text("New File")
            .build();

        action_box.append(&new_folder_btn);
        action_box.append(&new_file_btn);

        // Settings button - separate from action box
        let settings_btn = Button::builder()
            .icon_name("emblem-system-symbolic")
            .tooltip_text("Settings")
            .build();

        container.pack_end(&action_box);
        container.pack_end(&settings_btn);

        // Callbacks
        let on_back: Rc<RefCell<Option<Box<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let on_forward: Rc<RefCell<Option<Box<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let on_up: Rc<RefCell<Option<Box<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let on_home: Rc<RefCell<Option<Box<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let on_search: Rc<RefCell<Option<Box<dyn Fn(String)>>>> = Rc::new(RefCell::new(None));
        let on_new_folder: Rc<RefCell<Option<Box<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let on_new_file: Rc<RefCell<Option<Box<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let on_settings: Rc<RefCell<Option<Box<dyn Fn()>>>> = Rc::new(RefCell::new(None));

        // Connect signals
        {
            let on_back_clone = on_back.clone();
            back_btn.connect_clicked(move |_| {
                if let Some(ref callback) = *on_back_clone.borrow() {
                    callback();
                }
            });
        }

        {
            let on_forward_clone = on_forward.clone();
            forward_btn.connect_clicked(move |_| {
                if let Some(ref callback) = *on_forward_clone.borrow() {
                    callback();
                }
            });
        }

        {
            let on_up_clone = on_up.clone();
            up_btn.connect_clicked(move |_| {
                if let Some(ref callback) = *on_up_clone.borrow() {
                    callback();
                }
            });
        }

        {
            let on_home_clone = on_home.clone();
            home_btn.connect_clicked(move |_| {
                if let Some(ref callback) = *on_home_clone.borrow() {
                    callback();
                }
            });
        }

        {
            let on_search_clone = on_search.clone();
            search_entry.connect_search_changed(move |entry| {
                if let Some(ref callback) = *on_search_clone.borrow() {
                    callback(entry.text().to_string());
                }
            });
        }

        {
            let on_new_folder_clone = on_new_folder.clone();
            new_folder_btn.connect_clicked(move |_| {
                if let Some(ref callback) = *on_new_folder_clone.borrow() {
                    callback();
                }
            });
        }

        {
            let on_new_file_clone = on_new_file.clone();
            new_file_btn.connect_clicked(move |_| {
                if let Some(ref callback) = *on_new_file_clone.borrow() {
                    callback();
                }
            });
        }

        {
            let on_settings_clone = on_settings.clone();
            settings_btn.connect_clicked(move |_| {
                if let Some(ref callback) = *on_settings_clone.borrow() {
                    callback();
                }
            });
        }

        Self {
            container,
            back_btn,
            forward_btn,
            up_btn,
            home_btn,
            search_entry,
            new_folder_btn,
            new_file_btn,
            settings_btn,
            on_back,
            on_forward,
            on_up,
            on_home,
            on_search,
            on_new_folder,
            on_new_file,
            on_settings,
        }
    }

    pub fn container(&self) -> &GtkHeaderBar {
        &self.container
    }

    pub fn connect_back<F: Fn() + 'static>(&self, callback: F) {
        *self.on_back.borrow_mut() = Some(Box::new(callback));
    }

    pub fn connect_forward<F: Fn() + 'static>(&self, callback: F) {
        *self.on_forward.borrow_mut() = Some(Box::new(callback));
    }

    pub fn connect_up<F: Fn() + 'static>(&self, callback: F) {
        *self.on_up.borrow_mut() = Some(Box::new(callback));
    }

    pub fn connect_home<F: Fn() + 'static>(&self, callback: F) {
        *self.on_home.borrow_mut() = Some(Box::new(callback));
    }

    pub fn connect_search<F: Fn(String) + 'static>(&self, callback: F) {
        *self.on_search.borrow_mut() = Some(Box::new(callback));
    }

    pub fn connect_new_folder<F: Fn() + 'static>(&self, callback: F) {
        *self.on_new_folder.borrow_mut() = Some(Box::new(callback));
    }

    pub fn connect_new_file<F: Fn() + 'static>(&self, callback: F) {
        *self.on_new_file.borrow_mut() = Some(Box::new(callback));
    }

    pub fn connect_settings<F: Fn() + 'static>(&self, callback: F) {
        *self.on_settings.borrow_mut() = Some(Box::new(callback));
    }

    pub fn clear_search(&self) {
        self.search_entry.set_text("");
    }
}
