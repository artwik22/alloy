use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Entry, Label, Orientation, Revealer};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

#[derive(Clone)]
pub struct PathBar {
    container: GtkBox,
    entry: Entry,
    breadcrumbs_box: GtkBox,
    revealer: Revealer,
    current_path: Rc<RefCell<PathBuf>>,
    showing_entry: Rc<RefCell<bool>>,
    on_path_changed: Rc<RefCell<Option<Box<dyn Fn(std::path::PathBuf)>>>>,
    on_segment_clicked: Rc<RefCell<Option<Box<dyn Fn(std::path::PathBuf)>>>>,
}

use std::path::PathBuf;

impl PathBar {
    pub fn new() -> Self {
        let container = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(0)
            .margin_start(16)
            .margin_end(16)
            .margin_top(8)
            .margin_bottom(8)
            .css_classes(["path-bar"])
            .valign(gtk4::Align::Center)
            .build();

        // Entry for editing path
        let entry = Entry::builder()
            .placeholder_text("Enter path...")
            .css_classes(["path-entry"])
            .hexpand(true)
            .build();

        // Breadcrumbs container
        let breadcrumbs_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(0)
            .hexpand(true)
            .css_classes(["path-breadcrumbs"])
            .build();

        // Revealer to switch between entry and breadcrumbs
        let revealer = Revealer::builder()
            .reveal_child(true)
            .transition_type(gtk4::RevealerTransitionType::SlideLeft)
            .transition_duration(150)
            .child(&breadcrumbs_box)
            .build();

        container.append(&entry);
        container.append(&revealer);

        let current_path: Rc<RefCell<PathBuf>> = Rc::new(RefCell::new(PathBuf::from("/")));
        let showing_entry: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));

        // Entry handlers will be set up in connect_path_changed

        // Initially hide entry
        entry.set_visible(false);

        Self {
            container,
            entry,
            breadcrumbs_box,
            revealer,
            current_path,
            showing_entry,
            on_path_changed: Rc::new(RefCell::new(None)),
            on_segment_clicked: Rc::new(RefCell::new(None)),
        }
    }

    pub fn container(&self) -> &GtkBox {
        &self.container
    }

    pub fn set_path(&self, path: &Path) {
        self.current_path.replace(path.to_path_buf());
        
        // Clear existing breadcrumbs
        while let Some(child) = self.breadcrumbs_box.first_child() {
            self.breadcrumbs_box.remove(&child);
        }

        // Build path segments
        let path_str = path.to_string_lossy();
        let segments: Vec<&str> = path_str.split('/').filter(|s| !s.is_empty()).collect();

        // Root separator
        let root_btn = Button::builder()
            .label("/")
            .css_classes(["path-segment", "flat"])
            .build();
        
        {
            let callback = self.on_segment_clicked.clone();
            root_btn.connect_clicked(move |_| {
                if let Some(ref cb) = *callback.borrow() {
                    cb(PathBuf::from("/"));
                }
            });
        }
        self.breadcrumbs_box.append(&root_btn);

        // Path segments
        let mut accumulated_path = String::from("/");
        
        for (i, segment) in segments.iter().enumerate() {
            // Separator before each segment (but not before first, root button already has "/")
            if i > 0 {
                let sep = Label::builder()
                    .label("/")
                    .css_classes(["path-separator"])
                    .build();
                self.breadcrumbs_box.append(&sep);
            }

            accumulated_path.push_str(segment);
            
            let is_last = i == segments.len() - 1;
            
            let btn = Button::builder()
                .label(*segment)
                .css_classes(if is_last { 
                    vec!["path-segment", "path-current", "flat"] 
                } else { 
                    vec!["path-segment", "flat"] 
                })
                .build();

            let path_for_click = accumulated_path.clone();
            let callback = self.on_segment_clicked.clone();
            btn.connect_clicked(move |_| {
                if let Some(ref cb) = *callback.borrow() {
                    cb(PathBuf::from(&path_for_click));
                }
            });

            self.breadcrumbs_box.append(&btn);
            accumulated_path.push('/');
        }

        // Make breadcrumbs clickable to show entry for editing
        // Single click anywhere on breadcrumbs (except on segment buttons) will show entry
        let entry_clone = self.entry.clone();
        let revealer_clone = self.revealer.clone();
        let showing_entry_clone = self.showing_entry.clone();
        let current_path_clone = self.current_path.clone();
        
        // Add gesture to breadcrumbs box - check if we clicked on a button
        let gesture = gtk4::GestureClick::new();
        gesture.set_button(1);
        gesture.set_propagation_phase(gtk4::PropagationPhase::Bubble);
        
        gesture.connect_pressed(move |gest, n_press, x, y| {
            if n_press == 1 {
                // Check if we clicked on a button - if so, let button handle it
                if let Some(widget) = gest.widget() {
                    let clicked_widget: Option<gtk4::Widget> = widget.pick(x, y, gtk4::PickFlags::all());
                    
                    let clicked_on_button = clicked_widget.as_ref().map_or(false, |w| {
                        w.is::<gtk4::Button>() || w.ancestor(gtk4::Button::static_type()).is_some()
                    });
                    
                    // Only show entry if we didn't click on a button
                    if !clicked_on_button && !*showing_entry_clone.borrow() {
                        let path_str = current_path_clone.borrow().to_string_lossy().to_string();
                        *showing_entry_clone.borrow_mut() = true;
                        revealer_clone.set_reveal_child(false);
                        entry_clone.set_visible(true);
                        entry_clone.grab_focus();
                        entry_clone.set_text(&path_str);
                        entry_clone.select_region(0, -1);
                    }
                }
            }
        });
        
        self.breadcrumbs_box.add_controller(gesture);
    }

    pub fn connect_segment_clicked<F: Fn(PathBuf) + 'static>(&self, callback: F) {
        *self.on_segment_clicked.borrow_mut() = Some(Box::new(callback));
    }

    pub fn connect_path_changed<F: Fn(PathBuf) + 'static>(&self, callback: F) {
        *self.on_path_changed.borrow_mut() = Some(Box::new(callback));
        
        let entry_clone = self.entry.clone();
        let callback_clone = self.on_path_changed.clone();
        let revealer_clone = self.revealer.clone();
        let showing_entry_clone = self.showing_entry.clone();

        // Connect activate (Enter key)
        entry_clone.connect_activate(move |e| {
            let path_str = e.text().to_string();
            let path = PathBuf::from(path_str.trim());
            
            if path.exists() {
                if let Some(ref cb) = *callback_clone.borrow() {
                    cb(path);
                }
                *showing_entry_clone.borrow_mut() = false;
                revealer_clone.set_reveal_child(true);
                e.set_visible(false);
            }
        });

        // Handle Escape key
        let entry_esc = self.entry.clone();
        let revealer_esc = self.revealer.clone();
        let showing_entry_esc = self.showing_entry.clone();
        let controller = gtk4::EventControllerKey::new();
        controller.connect_key_pressed(move |_, key, _, _| {
            if key == gtk4::gdk::Key::Escape {
                if *showing_entry_esc.borrow() {
                    *showing_entry_esc.borrow_mut() = false;
                    revealer_esc.set_reveal_child(true);
                    entry_esc.set_visible(false);
                }
                gtk4::glib::Propagation::Stop
            } else {
                gtk4::glib::Propagation::Proceed
            }
        });
        self.entry.add_controller(controller);
    }
}
