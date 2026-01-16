use gtk4::glib::{self, Object};
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::{
    gio, CustomFilter, DragSource, DropTarget, FilterListModel, GestureClick, Label, ListItem, ListView, MultiSelection,
    PopoverMenu, SignalListItemFactory,
};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::core::{FileEntry, Scanner};

mod imp {
    use gtk4::glib;
    use gtk4::glib::Object;
    use gtk4::subclass::prelude::*;
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct FileObject {
        pub name: RefCell<String>,
        pub path: RefCell<String>,
        pub is_directory: RefCell<bool>,
        pub size: RefCell<String>,
        pub modified: RefCell<String>,
        pub icon_name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FileObject {
        const NAME: &'static str = "IndexFileObject";
        type Type = super::FileObject;
        type ParentType = Object;
    }

    impl ObjectImpl for FileObject {}
}

glib::wrapper! {
    pub struct FileObject(ObjectSubclass<imp::FileObject>);
}

impl FileObject {
    pub fn new(entry: &FileEntry) -> Self {
        let obj: Self = Object::builder().build();
        *obj.imp().name.borrow_mut() = entry.name.clone();
        *obj.imp().path.borrow_mut() = entry.path.to_string_lossy().to_string();
        *obj.imp().is_directory.borrow_mut() = entry.is_directory;
        *obj.imp().size.borrow_mut() = entry.size_display();
        *obj.imp().modified.borrow_mut() = entry.modified.clone();
        *obj.imp().icon_name.borrow_mut() = entry.icon_name.clone();
        obj
    }

    pub fn name(&self) -> String {
        self.imp().name.borrow().clone()
    }

    pub fn path(&self) -> PathBuf {
        PathBuf::from(self.imp().path.borrow().clone())
    }

    pub fn is_directory(&self) -> bool {
        *self.imp().is_directory.borrow()
    }

    pub fn size(&self) -> String {
        self.imp().size.borrow().clone()
    }

    pub fn modified(&self) -> String {
        self.imp().modified.borrow().clone()
    }

    pub fn icon_name(&self) -> String {
        self.imp().icon_name.borrow().clone()
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct FileView {
    container: gtk4::Box,
    list_view: ListView,
    store: gio::ListStore,
    filter: CustomFilter,
    selection: MultiSelection,
    current_path: Rc<RefCell<PathBuf>>,
    all_entries: Rc<RefCell<Vec<FileEntry>>>,
    selected_paths: Rc<RefCell<Vec<PathBuf>>>,
    show_hidden: Rc<RefCell<bool>>,

    on_directory_activated: Rc<RefCell<Option<Box<dyn Fn(PathBuf)>>>>,
    on_copy: Rc<RefCell<Option<Box<dyn Fn(Vec<PathBuf>)>>>>,
    on_cut: Rc<RefCell<Option<Box<dyn Fn(Vec<PathBuf>)>>>>,
    on_paste: Rc<RefCell<Option<Box<dyn Fn()>>>>,
    on_delete: Rc<RefCell<Option<Box<dyn Fn(Vec<PathBuf>)>>>>,
    on_rename: Rc<RefCell<Option<Box<dyn Fn(PathBuf)>>>>,
    on_pin: Rc<RefCell<Option<Box<dyn Fn(PathBuf)>>>>,
}

impl FileView {
    pub fn new() -> Self {
        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .build();

        // Create store
        let store = gio::ListStore::new::<FileObject>();

        // Create filter
        let filter = CustomFilter::new(|_| true);
        let filter_model = FilterListModel::new(Some(store.clone()), Some(filter.clone()));

        // Create selection model
        let selection = MultiSelection::new(Some(filter_model));
        let selection_clone = selection.clone();

        // Create factory
        let factory = SignalListItemFactory::new();

        factory.connect_setup(|_, item| {
            let item = item.downcast_ref::<ListItem>().unwrap();

            let hbox = gtk4::Box::builder()
                .orientation(gtk4::Orientation::Horizontal)
                .spacing(12)
                .margin_start(16)
                .margin_end(16)
                .margin_top(0)
                .margin_bottom(0)
                .build();

            let icon = gtk4::Image::builder().pixel_size(24).build();

            let name_label = Label::builder()
                .halign(gtk4::Align::Start)
                .hexpand(true)
                .ellipsize(gtk4::pango::EllipsizeMode::End)
                .build();

            let size_label = Label::builder()
                .halign(gtk4::Align::End)
                .width_chars(10)
                .css_classes(["dim-label"])
                .build();

            let date_label = Label::builder()
                .halign(gtk4::Align::End)
                .width_chars(16)
                .css_classes(["dim-label"])
                .build();

            hbox.append(&icon);
            hbox.append(&name_label);
            hbox.append(&size_label);
            hbox.append(&date_label);

            item.set_child(Some(&hbox));
        });

        factory.connect_bind(|_, item| {
            let item = item.downcast_ref::<ListItem>().unwrap();
            let file_obj = item.item().and_downcast::<FileObject>().unwrap();

            let hbox = item.child().and_downcast::<gtk4::Box>().unwrap();

            let icon = hbox.first_child().and_downcast::<gtk4::Image>().unwrap();
            let name_label = icon.next_sibling().and_downcast::<Label>().unwrap();
            let size_label = name_label.next_sibling().and_downcast::<Label>().unwrap();
            let date_label = size_label.next_sibling().and_downcast::<Label>().unwrap();

            icon.set_icon_name(Some(&file_obj.icon_name()));
            name_label.set_text(&file_obj.name());
            size_label.set_text(&file_obj.size());
            date_label.set_text(&file_obj.modified());
        });

        // Create list view
        let list_view = ListView::builder()
            .model(&selection)
            .factory(&factory)
            .css_classes(["file-list"])
            .build();

        container.append(&list_view);

        let current_path = Rc::new(RefCell::new(PathBuf::new()));
        let all_entries = Rc::new(RefCell::new(Vec::new()));
        let selected_paths = Rc::new(RefCell::new(Vec::new()));
        let show_hidden = Rc::new(RefCell::new(false));

        let on_directory_activated: Rc<RefCell<Option<Box<dyn Fn(PathBuf)>>>> =
            Rc::new(RefCell::new(None));
        let on_copy: Rc<RefCell<Option<Box<dyn Fn(Vec<PathBuf>)>>>> = Rc::new(RefCell::new(None));
        let on_cut: Rc<RefCell<Option<Box<dyn Fn(Vec<PathBuf>)>>>> = Rc::new(RefCell::new(None));
        let on_paste: Rc<RefCell<Option<Box<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let on_delete: Rc<RefCell<Option<Box<dyn Fn(Vec<PathBuf>)>>>> = Rc::new(RefCell::new(None));
        let on_rename: Rc<RefCell<Option<Box<dyn Fn(PathBuf)>>>> = Rc::new(RefCell::new(None));
        let on_pin: Rc<RefCell<Option<Box<dyn Fn(PathBuf)>>>> = Rc::new(RefCell::new(None));

        // Double-click to activate
        {
            let on_directory_activated_clone = on_directory_activated.clone();
            let selection_clone = selection.clone();

            list_view.connect_activate(move |_, position| {
                if let Some(item) = selection_clone.item(position) {
                    let file_obj = item.downcast::<FileObject>().unwrap();
                    if file_obj.is_directory() {
                        if let Some(ref callback) = *on_directory_activated_clone.borrow() {
                            callback(file_obj.path());
                        }
                    } else {
                        // Open file with default application
                        let path = file_obj.path();
                        if let Err(e) = open::that(&path) {
                            eprintln!("Failed to open file: {}", e);
                        }
                    }
                }
            });
        }

        // Context menu
        {
            let on_copy_clone = on_copy.clone();
            let on_cut_clone = on_cut.clone();
            let on_paste_clone = on_paste.clone();
            let on_delete_clone = on_delete.clone();
            let on_rename_clone = on_rename.clone();
            let on_pin_clone = on_pin.clone();
            let selection_clone = selection.clone();
            let list_view_clone = list_view.clone();

            let gesture = GestureClick::builder().button(3).build();

            gesture.connect_pressed(move |_gesture, _, x, y| {
                // Get all selected items
                let mut selected_paths = Vec::new();
                let n_items = selection_clone.n_items();
                for i in 0..n_items {
                    if selection_clone.is_selected(i) {
                        if let Some(item) = selection_clone.item(i) {
                            if let Ok(file_obj) = item.downcast::<FileObject>() {
                                selected_paths.push(file_obj.path());
                            }
                        }
                    }
                }

                let menu = gio::Menu::new();
                menu.append(Some("Copy"), Some("file.copy"));
                menu.append(Some("Cut"), Some("file.cut"));
                menu.append(Some("Paste"), Some("file.paste"));
                menu.append(Some("Delete"), Some("file.delete"));
                menu.append(Some("Rename"), Some("file.rename"));
                
                // Add Pin option for directories (only if single directory selected)
                let is_dir = if selected_paths.len() == 1 {
                    selected_paths.first()
                        .and_then(|p| std::fs::metadata(p).ok())
                        .map(|m| m.is_dir())
                        .unwrap_or(false)
                } else {
                    false
                };
                
                if is_dir {
                    menu.append(Some("Pin to Sidebar"), Some("file.pin"));
                }

                let popover = PopoverMenu::from_model(Some(&menu));
                popover.set_parent(&list_view_clone);
                popover.set_pointing_to(Some(&gtk4::gdk::Rectangle::new(
                    x as i32,
                    y as i32,
                    1,
                    1,
                )));

                // Create action group
                let action_group = gio::SimpleActionGroup::new();

                // Copy action
                {
                    let paths = selected_paths.clone();
                    let on_copy = on_copy_clone.clone();
                    let action = gio::SimpleAction::new("copy", None);
                    action.connect_activate(move |_, _| {
                        if let Some(ref callback) = *on_copy.borrow() {
                            callback(paths.clone());
                        }
                    });
                    action_group.add_action(&action);
                }

                // Cut action
                {
                    let paths = selected_paths.clone();
                    let on_cut = on_cut_clone.clone();
                    let action = gio::SimpleAction::new("cut", None);
                    action.connect_activate(move |_, _| {
                        if let Some(ref callback) = *on_cut.borrow() {
                            callback(paths.clone());
                        }
                    });
                    action_group.add_action(&action);
                }

                // Paste action
                {
                    let on_paste = on_paste_clone.clone();
                    let action = gio::SimpleAction::new("paste", None);
                    action.connect_activate(move |_, _| {
                        if let Some(ref callback) = *on_paste.borrow() {
                            callback();
                        }
                    });
                    action_group.add_action(&action);
                }

                // Delete action
                {
                    let paths = selected_paths.clone();
                    let on_delete = on_delete_clone.clone();
                    let action = gio::SimpleAction::new("delete", None);
                    action.connect_activate(move |_, _| {
                        if let Some(ref callback) = *on_delete.borrow() {
                            callback(paths.clone());
                        }
                    });
                    action_group.add_action(&action);
                }

                // Rename action
                {
                    let paths = selected_paths.clone();
                    let on_rename = on_rename_clone.clone();
                    let action = gio::SimpleAction::new("rename", None);
                    action.connect_activate(move |_, _| {
                        if let Some(path) = paths.first() {
                            if let Some(ref callback) = *on_rename.borrow() {
                                callback(path.clone());
                            }
                        }
                    });
                    action_group.add_action(&action);
                }

                // Pin action (only for directories)
                if is_dir {
                    let paths = selected_paths.clone();
                    let on_pin = on_pin_clone.clone();
                    let action = gio::SimpleAction::new("pin", None);
                    action.connect_activate(move |_, _| {
                        if let Some(path) = paths.first() {
                            if let Some(ref callback) = *on_pin.borrow() {
                                callback(path.clone());
                            }
                        }
                    });
                    action_group.add_action(&action);
                }

                popover.insert_action_group("file", Some(&action_group));
                popover.popup();
            });

            list_view.add_controller(gesture);
        }

        // Drag & Drop support
        {
            // DragSource - allow dragging files from the list (works between windows)
            let drag_source = DragSource::new();
            drag_source.set_actions(gtk4::gdk::DragAction::COPY | gtk4::gdk::DragAction::MOVE);
            
            let selection_clone = selection.clone();
            drag_source.connect_prepare(move |_, _x, _y| {
                // Get all selected items
                let mut selected_items = Vec::new();
                let n_items = selection_clone.n_items();
                for i in 0..n_items {
                    if selection_clone.is_selected(i) {
                        if let Some(item) = selection_clone.item(i) {
                            if let Ok(file_obj) = item.downcast::<FileObject>() {
                                selected_items.push(file_obj);
                            }
                        }
                    }
                }
                
                if !selected_items.is_empty() {
                    // Create GFiles for all selected paths
                    let gfiles: Vec<gio::File> = selected_items
                        .iter()
                        .map(|obj| gio::File::for_path(&obj.path()))
                        .collect();
                    
                    // Create FileList for GTK4 DND (primary format for GTK4 apps)
                    let file_list = gtk4::gdk::FileList::from_array(&gfiles);
                    
                    // Create content provider with FileList
                    // GTK4 automatically provides text/uri-list format for cross-window compatibility
                    let content = gtk4::gdk::ContentProvider::for_value(&file_list.to_value());
                    Some(content)
                } else {
                    None
                }
            });
            
            // Add visual feedback during drag
            {
                let selection_clone = selection.clone();
                drag_source.connect_drag_begin(move |source, drag| {
                    let mut count = 0;
                    let n_items = selection_clone.n_items();
                    for i in 0..n_items {
                        if selection_clone.is_selected(i) {
                            count += 1;
                        }
                    }
                    if count > 0 {
                        // Create a simple icon showing the number of files
                        let icon = gtk4::Image::builder()
                            .icon_name("document")
                            .pixel_size(48)
                            .build();
                        
                        // Create a label with count if multiple files
                        let drag_icon = gtk4::DragIcon::for_drag(drag);
                        if count > 1 {
                            let box_widget = gtk4::Box::builder()
                                .orientation(gtk4::Orientation::Vertical)
                                .spacing(4)
                                .build();
                            
                            box_widget.append(&icon);
                            
                            let label = gtk4::Label::builder()
                                .css_classes(["drag-count-label"])
                                .build();
                            label.set_text(&format!("{}", count));
                            box_widget.append(&label);
                            
                            drag_icon.set_child(Some(&box_widget));
                        } else {
                            drag_icon.set_child(Some(&icon));
                        }
                    }
                });
            }
            
            list_view.add_controller(drag_source);
            
            // DropTarget - allow dropping files onto the list (works between windows and applications)
            let current_path_clone = current_path.clone();
            let on_paste_clone = on_paste.clone();
            let store_clone = store.clone();
            let show_hidden_clone = show_hidden.clone();
            
            // Helper function to extract files from different formats
            let extract_files = |value: &glib::Value| -> Vec<PathBuf> {
                // Try FileList first (GTK4 native format)
                if let Ok(file_list) = value.get::<gtk4::gdk::FileList>() {
                    file_list.files()
                        .iter()
                        .filter_map(|f| f.path())
                        .collect()
                }
                // Try text/uri-list format (for cross-application compatibility)
                else if let Ok(uri_list) = value.get::<String>() {
                    uri_list
                        .lines()
                        .filter_map(|line| {
                            let line = line.trim();
                            if line.is_empty() || line.starts_with('#') {
                                return None;
                            }
                            // Convert URI to path
                            gio::File::for_uri(line).path()
                        })
                        .collect()
                } else {
                    vec![]
                }
            };
            
            // Support FileList for cross-application compatibility
            let drop_target = DropTarget::new(gtk4::gdk::FileList::static_type(), gtk4::gdk::DragAction::COPY | gtk4::gdk::DragAction::MOVE);
            
            // Also support text/uri-list directly for better cross-window compatibility
            let drop_target_uri = DropTarget::new(glib::types::Type::STRING, gtk4::gdk::DragAction::COPY | gtk4::gdk::DragAction::MOVE);
            
            // Common drop handler
            let handle_drop = |files: Vec<PathBuf>, 
                              current_path: &Rc<RefCell<PathBuf>>,
                              store: &gio::ListStore, 
                              show_hidden: &Rc<RefCell<bool>>,
                              on_paste: &Rc<RefCell<Option<Box<dyn Fn()>>>>| -> bool {
                if files.is_empty() {
                    return false;
                }
                
                // Get current directory
                let dest_dir = current_path.borrow().clone();
                
                // Determine action: move if different directory, copy if same directory
                let mut same_dir = false;
                for source_file in &files {
                    if let Some(parent) = source_file.parent() {
                        if parent == dest_dir {
                            same_dir = true;
                            break;
                        }
                    }
                }
                let should_move = !same_dir; // Move unless same directory
                
                let mut errors = Vec::new();
                
                // Move/copy files to current directory
                for source_file in &files {
                    if let Some(file_name) = source_file.file_name() {
                        let dest_path = dest_dir.join(file_name);
                        
                        // Skip if source and dest are the same
                        if source_file == &dest_path {
                            continue;
                        }
                        
                        // Handle duplicates
                        let mut final_dest = dest_path.clone();
                        let mut counter = 1;
                        while final_dest.exists() {
                            let stem = source_file
                                .file_stem()
                                .map(|s| s.to_string_lossy().to_string())
                                .unwrap_or_default();
                            let extension = source_file
                                .extension()
                                .map(|e| format!(".{}", e.to_string_lossy()))
                                .unwrap_or_default();
                            
                            let new_name = format!("{} ({}){}", stem, counter, extension);
                            final_dest = dest_dir.join(new_name);
                            counter += 1;
                        }
                        
                        // Move or copy file
                        let result = if should_move {
                            crate::core::FileOperations::move_file(source_file, &final_dest)
                        } else {
                            crate::core::FileOperations::copy_file(source_file, &final_dest)
                        };
                        
                        if let Err(e) = result {
                            let op = if should_move { "move" } else { "copy" };
                            errors.push(format!("Failed to {} '{}': {}", op, source_file.display(), e));
                        }
                    }
                }
                
                // Report errors
                if !errors.is_empty() {
                    eprintln!("Drag and drop errors:");
                    for error in &errors {
                        eprintln!("  {}", error);
                    }
                }
                
                // Refresh the view by reloading directory
                let show_hidden_val = *show_hidden.borrow();
                match crate::core::Scanner::scan_with_hidden(&dest_dir, show_hidden_val) {
                    Ok(entries) => {
                        store.remove_all();
                        for entry in &entries {
                            store.append(&FileObject::new(entry));
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to refresh directory: {}", e);
                    }
                }
                
                // Also trigger paste callback if available
                if let Some(ref callback) = *on_paste.borrow() {
                    callback();
                }
                
                true
            };
            
            // Connect drop handler for FileList
            {
                let current_path_clone2 = current_path_clone.clone();
                let on_paste_clone2 = on_paste_clone.clone();
                let store_clone2 = store_clone.clone();
                let show_hidden_clone2 = show_hidden_clone.clone();
                
                drop_target.connect_drop(move |_, value, _, _| {
                    let files = extract_files(value);
                    handle_drop(files, &current_path_clone2, &store_clone2, &show_hidden_clone2, &on_paste_clone2)
                });
            }
            
            // Connect drop handler for text/uri-list
            {
                let current_path_clone2 = current_path_clone.clone();
                let on_paste_clone2 = on_paste_clone.clone();
                let store_clone2 = store_clone.clone();
                let show_hidden_clone2 = show_hidden_clone.clone();
                
                drop_target_uri.connect_drop(move |_, value, _, _| {
                    let files = extract_files(value);
                    handle_drop(files, &current_path_clone2, &store_clone2, &show_hidden_clone2, &on_paste_clone2)
                });
            }
            
            // Note: Modifier keys (Ctrl/Shift) are handled automatically by GTK4's drag system
            // We support both FileList and text/uri-list for maximum compatibility
            
            list_view.add_controller(drop_target);
            list_view.add_controller(drop_target_uri);
        }

        Self {
            container,
            list_view,
            store,
            filter,
            selection: selection_clone,
            current_path,
            all_entries,
            selected_paths,
            show_hidden,
            on_directory_activated,
            on_copy,
            on_cut,
            on_paste,
            on_delete,
            on_rename,
            on_pin,
        }
    }

    pub fn container(&self) -> &gtk4::Box {
        &self.container
    }

    pub fn load_directory(&self, path: &Path) {
        // Clear selection first to avoid issues when modifying store
        self.selection.unselect_all();
        
        self.current_path.replace(path.to_path_buf());
        self.store.remove_all();

        let show_hidden = *self.show_hidden.borrow();
        match Scanner::scan_with_hidden(path, show_hidden) {
            Ok(entries) => {
                self.all_entries.replace(entries.clone());
                for entry in &entries {
                    self.store.append(&FileObject::new(entry));
                }
            }
            Err(e) => {
                eprintln!("Failed to scan directory: {}", e);
            }
        }
    }

    pub fn toggle_hidden(&self) {
        // Use replace to avoid borrow conflicts
        let old_value = self.show_hidden.replace(false);
        self.show_hidden.replace(!old_value);
        
        let current = self.current_path.borrow().clone();
        self.load_directory(&current);
    }

    pub fn refresh(&self) {
        let current = self.current_path.borrow().clone();
        self.load_directory(&current);
    }

    pub fn select_all(&self) {
        // Select all items
        let n_items = self.store.n_items();
        for i in 0..n_items {
            self.selection.select_item(i, false);
        }
    }

    pub fn open_with_micro(&self) {
        // Open first selected item with micro
        let n_items = self.selection.n_items();
        for i in 0..n_items {
            if self.selection.is_selected(i) {
                if let Some(item) = self.selection.item(i) {
                    if let Ok(file_obj) = item.downcast::<FileObject>() {
                        let path = file_obj.path();
                        let path_str = path.display().to_string();
                        
                        // Open terminal with micro editor
                        // Try different terminals in order
                        let terminals: Vec<(&str, Vec<&str>)> = vec![
                            ("alacritty", vec!["-e", "micro", &path_str]),
                            ("xterm", vec!["-e", "micro", &path_str]),
                            ("gnome-terminal", vec!["--", "micro", &path_str]),
                            ("konsole", vec!["-e", "micro", &path_str]),
                            ("kitty", vec!["-e", "micro", &path_str]),
                        ];
                        
                        let mut opened = false;
                        for (term, args) in terminals.iter() {
                            if std::process::Command::new(term)
                                .args(args)
                                .spawn()
                                .is_ok()
                            {
                                opened = true;
                                break;
                            }
                        }
                        
                        if !opened {
                            eprintln!("Failed to open terminal with micro editor");
                        }
                        break; // Only open first selected item
                    }
                }
            }
        }
    }

    pub fn filter(&self, query: &str) {
        let query = query.to_lowercase();
        let query_owned = query.clone();

        self.filter.set_filter_func(move |obj| {
            if query_owned.is_empty() {
                return true;
            }
            let file_obj = obj.downcast_ref::<FileObject>().unwrap();
            file_obj.name().to_lowercase().contains(&query_owned)
        });
    }

    pub fn connect_directory_activated<F: Fn(PathBuf) + 'static>(&self, callback: F) {
        *self.on_directory_activated.borrow_mut() = Some(Box::new(callback));
    }

    pub fn connect_copy<F: Fn(Vec<PathBuf>) + 'static>(&self, callback: F) {
        *self.on_copy.borrow_mut() = Some(Box::new(callback));
    }

    pub fn connect_cut<F: Fn(Vec<PathBuf>) + 'static>(&self, callback: F) {
        *self.on_cut.borrow_mut() = Some(Box::new(callback));
    }

    pub fn connect_paste<F: Fn() + 'static>(&self, callback: F) {
        *self.on_paste.borrow_mut() = Some(Box::new(callback));
    }

    pub fn connect_delete<F: Fn(Vec<PathBuf>) + 'static>(&self, callback: F) {
        *self.on_delete.borrow_mut() = Some(Box::new(callback));
    }

    pub fn connect_rename<F: Fn(PathBuf) + 'static>(&self, callback: F) {
        *self.on_rename.borrow_mut() = Some(Box::new(callback));
    }

    pub fn connect_pin<F: Fn(PathBuf) + 'static>(&self, callback: F) {
        *self.on_pin.borrow_mut() = Some(Box::new(callback));
    }
}
