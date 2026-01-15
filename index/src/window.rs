use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Orientation, Paned, ScrolledWindow};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::core::{Clipboard, FileOperations, KeybindAction, KeybindConfig};
use crate::widgets::{FileView, HeaderBar, PathBar, Sidebar, SettingsWindow};

pub struct IndexWindow {
    pub window: ApplicationWindow,
}

impl IndexWindow {
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Index")
            .default_width(1000)
            .default_height(700)
            .resizable(true)
            .decorated(true)
            .build();
        
        // Ensure window is treated as a normal window, not floating
        window.set_modal(false);
        window.set_destroy_with_parent(false);

        // Shared state
        let current_path: Rc<RefCell<PathBuf>> = Rc::new(RefCell::new(
            dirs::home_dir().unwrap_or_else(|| PathBuf::from("/")),
        ));
        let history: Rc<RefCell<Vec<PathBuf>>> = Rc::new(RefCell::new(Vec::new()));
        let history_index: Rc<RefCell<i32>> = Rc::new(RefCell::new(-1));
        let clipboard: Rc<RefCell<Clipboard>> = Rc::new(RefCell::new(Clipboard::new()));

        // Main layout
        let main_box = GtkBox::new(Orientation::Vertical, 0);

        // Path bar
        let path_bar = PathBar::new();

        // Header bar
        let header_bar = HeaderBar::new();
        window.set_titlebar(Some(header_bar.container()));

        // Create paned for sidebar and file view
        let paned = Paned::new(Orientation::Horizontal);
        paned.set_position(220);
        paned.set_shrink_start_child(false);
        paned.set_shrink_end_child(false);
        paned.set_wide_handle(false);
        paned.set_resize_start_child(false);

        // Sidebar
        let sidebar = Sidebar::new();
        paned.set_start_child(Some(sidebar.container()));

        // File view container
        let file_view_box = GtkBox::new(Orientation::Vertical, 0);

        // Add path bar to file view box
        file_view_box.append(path_bar.container());

        // File view
        let file_view = FileView::new();
        let scrolled = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Automatic)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .vexpand(true)
            .hexpand(true)
            .child(file_view.container())
            .build();
        file_view_box.append(&scrolled);

        paned.set_end_child(Some(&file_view_box));

        main_box.append(&paned);
        window.set_child(Some(&main_box));
        
        // Add DropTarget at window level for cross-window drag and drop
        {
            let current_path_clone = current_path.clone();
            let file_view_clone = file_view.clone();
            
            let window_drop_target = gtk4::DropTarget::new(
                gtk4::gdk::FileList::static_type(),
                gtk4::gdk::DragAction::COPY | gtk4::gdk::DragAction::MOVE
            );
            
            window_drop_target.connect_drop(move |_, value, _, _| {
                // Extract files from FileList
                if let Ok(file_list) = value.get::<gtk4::gdk::FileList>() {
                    let files: Vec<PathBuf> = file_list.files()
                        .iter()
                        .filter_map(|f| f.path())
                        .collect();
                    
                    if !files.is_empty() {
                        // Get current directory from file view
                        let dest_dir = current_path_clone.borrow().clone();
                        
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
                        let should_move = !same_dir;
                        
                        // Move/copy files
                        for source_file in &files {
                            if let Some(file_name) = source_file.file_name() {
                                let dest_path = dest_dir.join(file_name);
                                
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
                                
                                let result = if should_move {
                                    crate::core::FileOperations::move_file(source_file, &final_dest)
                                } else {
                                    crate::core::FileOperations::copy_file(source_file, &final_dest)
                                };
                                
                                if let Err(e) = result {
                                    eprintln!("Failed to {} file: {}", if should_move { "move" } else { "copy" }, e);
                                }
                            }
                        }
                        
                        // Refresh file view
                        file_view_clone.refresh();
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            });
            
            window.add_controller(window_drop_target);
        }

        // Load keybinds configuration
        let keybinds = KeybindConfig::load();
        
        // Helper function to check if keybind matches
        let matches_keybind = |key: gtk4::gdk::Key, mods: gtk4::gdk::ModifierType, keybind: &crate::core::Keybind| -> bool {
            let key_name = key.name().map(|s| s.to_string()).unwrap_or_default();
            if key_name != keybind.key {
                return false;
            }
            
            let mut has_control = mods.contains(gtk4::gdk::ModifierType::CONTROL_MASK);
            let mut has_shift = mods.contains(gtk4::gdk::ModifierType::SHIFT_MASK);
            let mut has_alt = mods.contains(gtk4::gdk::ModifierType::ALT_MASK);
            let mut has_super = mods.contains(gtk4::gdk::ModifierType::SUPER_MASK);
            
            // Check if modifiers match
            let mut required_control = false;
            let mut required_shift = false;
            let mut required_alt = false;
            let mut required_super = false;
            
            for mod_str in &keybind.modifiers {
                match mod_str.as_str() {
                    "Control" => required_control = true,
                    "Shift" => required_shift = true,
                    "Alt" => required_alt = true,
                    "Super" => required_super = true,
                    _ => {}
                }
            }
            
            // Check that required modifiers are present and no extra modifiers
            if required_control != has_control || required_shift != has_shift || 
               required_alt != has_alt || required_super != has_super {
                return false;
            }
            
            // Check that no other modifiers are present
            let other_mods = mods & !(gtk4::gdk::ModifierType::CONTROL_MASK | 
                                     gtk4::gdk::ModifierType::SHIFT_MASK |
                                     gtk4::gdk::ModifierType::ALT_MASK |
                                     gtk4::gdk::ModifierType::SUPER_MASK);
            if !other_mods.is_empty() {
                return false;
            }
            
            true
        };

        // Keyboard shortcuts
        let file_view_clone = file_view.clone();
        let current_path_clone = current_path.clone();
        let keybinds_clone = keybinds.clone();
        let controller = gtk4::EventControllerKey::new();
        controller.connect_key_pressed(move |_, key, _, mods| {
            // Toggle hidden files
            if let Some(keybind) = keybinds_clone.get(&KeybindAction::ToggleHidden) {
                if matches_keybind(key, mods, keybind) {
                    file_view_clone.toggle_hidden();
                    return gtk4::glib::Propagation::Stop;
                }
            }
            
            // Open terminal
            if let Some(keybind) = keybinds_clone.get(&KeybindAction::OpenTerminal) {
                if matches_keybind(key, mods, keybind) {
                    let current = current_path_clone.borrow().clone();
                    let path_str = current.display().to_string();
                    
                    // Try different terminals in order
                    let terminals: Vec<(&str, Vec<String>)> = vec![
                        ("alacritty", vec!["--working-directory".to_string(), path_str.clone()]),
                        ("xterm", vec!["-e".to_string(), "sh".to_string(), "-c".to_string(), format!("cd '{}' && exec $SHELL", path_str.clone())]),
                        ("gnome-terminal", vec!["--working-directory".to_string(), path_str.clone()]),
                        ("konsole", vec!["--workdir".to_string(), path_str.clone()]),
                        ("kitty", vec!["-d".to_string(), path_str.clone()]),
                        ("terminator", vec!["--working-directory".to_string(), path_str.clone()]),
                        ("tilix", vec!["-w".to_string(), path_str.clone()]),
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
                        eprintln!("Failed to open terminal in directory: {}", path_str);
                    }
                    return gtk4::glib::Propagation::Stop;
                }
            }
            
            // Select all
            if let Some(keybind) = keybinds_clone.get(&KeybindAction::SelectAll) {
                if matches_keybind(key, mods, keybind) {
                    file_view_clone.select_all();
                    return gtk4::glib::Propagation::Stop;
                }
            }
            
            // Refresh
            if let Some(keybind) = keybinds_clone.get(&KeybindAction::Refresh) {
                if matches_keybind(key, mods, keybind) {
                    file_view_clone.refresh();
                    return gtk4::glib::Propagation::Stop;
                }
            }
            
            // Open with micro
            if let Some(keybind) = keybinds_clone.get(&KeybindAction::OpenWithMicro) {
                if matches_keybind(key, mods, keybind) {
                    file_view_clone.open_with_micro();
                    return gtk4::glib::Propagation::Stop;
                }
            }
            
            gtk4::glib::Propagation::Proceed
        });
        window.add_controller(controller);
        
        // Mouse side buttons for back/forward navigation
        {
            // Back button
            if let Some(keybind) = keybinds.get(&KeybindAction::Back) {
                if keybind.key == "Mouse8" {
                    let file_view_clone = file_view.clone();
                    let path_bar_clone = path_bar.clone();
                    let current_path_clone = current_path.clone();
                    let history_clone = history.clone();
                    let history_index_clone = history_index.clone();
                    
                    let back_gesture = gtk4::GestureClick::new();
                    back_gesture.set_button(8);
                    back_gesture.connect_pressed(move |_, _, _, _| {
                        let mut idx = history_index_clone.borrow_mut();
                        if *idx > 0 {
                            *idx -= 1;
                            let hist = history_clone.borrow();
                            if let Some(path) = hist.get(*idx as usize) {
                                current_path_clone.replace(path.clone());
                                file_view_clone.load_directory(path);
                                path_bar_clone.set_path(path);
                            }
                        }
                    });
                    window.add_controller(back_gesture);
                }
            }
            
            // Forward button
            if let Some(keybind) = keybinds.get(&KeybindAction::Forward) {
                if keybind.key == "Mouse9" {
                    let file_view_clone = file_view.clone();
                    let path_bar_clone = path_bar.clone();
                    let current_path_clone = current_path.clone();
                    let history_clone = history.clone();
                    let history_index_clone = history_index.clone();
                    
                    let forward_gesture = gtk4::GestureClick::new();
                    forward_gesture.set_button(9);
                    forward_gesture.connect_pressed(move |_, _, _, _| {
                        let hist = history_clone.borrow();
                        let mut idx = history_index_clone.borrow_mut();
                        if (*idx as usize) < hist.len() - 1 {
                            *idx += 1;
                            if let Some(path) = hist.get(*idx as usize) {
                                current_path_clone.replace(path.clone());
                                file_view_clone.load_directory(path);
                                path_bar_clone.set_path(path);
                            }
                        }
                    });
                    window.add_controller(forward_gesture);
                }
            }
        }

        // Initial load
        let initial_path = current_path.borrow().clone();
        file_view.load_directory(&initial_path);
        path_bar.set_path(&initial_path);

        // Push initial path to history
        {
            let mut hist = history.borrow_mut();
            hist.push(initial_path);
            *history_index.borrow_mut() = 0;
        }

        // Connect path bar segment clicks
        {
            let file_view_clone = file_view.clone();
            let path_bar_clone = path_bar.clone();
            let current_path_clone = current_path.clone();
            let history_clone = history.clone();
            let history_index_clone = history_index.clone();
            let header_bar_clone = header_bar.clone();

            path_bar.connect_segment_clicked(move |path| {
                current_path_clone.replace(path.clone());
                file_view_clone.load_directory(&path);
                path_bar_clone.set_path(&path);
                header_bar_clone.clear_search();

                // Add to history
                let mut hist = history_clone.borrow_mut();
                let mut idx = history_index_clone.borrow_mut();
                *idx += 1;
                hist.truncate(*idx as usize);
                hist.push(path);
            });
        }

        // Connect path bar path changed (when user types path)
        {
            let file_view_clone = file_view.clone();
            let path_bar_clone = path_bar.clone();
            let current_path_clone = current_path.clone();
            let history_clone = history.clone();
            let history_index_clone = history_index.clone();
            let header_bar_clone = header_bar.clone();

            path_bar.connect_path_changed(move |path| {
                current_path_clone.replace(path.clone());
                file_view_clone.load_directory(&path);
                path_bar_clone.set_path(&path);
                header_bar_clone.clear_search();

                // Add to history
                let mut hist = history_clone.borrow_mut();
                let mut idx = history_index_clone.borrow_mut();
                *idx += 1;
                hist.truncate(*idx as usize);
                hist.push(path);
            });
        }

        // Connect sidebar navigation
        {
            let file_view_clone = file_view.clone();
            let path_bar_clone = path_bar.clone();
            let current_path_clone = current_path.clone();
            let history_clone = history.clone();
            let history_index_clone = history_index.clone();

            sidebar.connect_location_selected(move |path| {
                current_path_clone.replace(path.clone());
                file_view_clone.load_directory(&path);
                path_bar_clone.set_path(&path);

                // Add to history
                let mut hist = history_clone.borrow_mut();
                let mut idx = history_index_clone.borrow_mut();
                *idx += 1;
                hist.truncate(*idx as usize);
                hist.push(path);
            });
        }

        // Connect file view navigation (double-click on folder)
        {
            let file_view_clone = file_view.clone();
            let path_bar_clone = path_bar.clone();
            let current_path_clone = current_path.clone();
            let history_clone = history.clone();
            let history_index_clone = history_index.clone();

            file_view.connect_directory_activated(move |path| {
                current_path_clone.replace(path.clone());
                file_view_clone.load_directory(&path);
                path_bar_clone.set_path(&path);

                // Add to history
                let mut hist = history_clone.borrow_mut();
                let mut idx = history_index_clone.borrow_mut();
                *idx += 1;
                hist.truncate(*idx as usize);
                hist.push(path);
            });
        }

        // Connect header bar actions
        {
            // Back button
            let file_view_clone = file_view.clone();
            let path_bar_clone = path_bar.clone();
            let current_path_clone = current_path.clone();
            let history_clone = history.clone();
            let history_index_clone = history_index.clone();

            header_bar.connect_back(move || {
                let mut idx = history_index_clone.borrow_mut();
                if *idx > 0 {
                    *idx -= 1;
                    let hist = history_clone.borrow();
                    if let Some(path) = hist.get(*idx as usize) {
                        current_path_clone.replace(path.clone());
                        file_view_clone.load_directory(path);
                        path_bar_clone.set_path(path);
                    }
                }
            });
        }

        {
            // Forward button
            let file_view_clone = file_view.clone();
            let path_bar_clone = path_bar.clone();
            let current_path_clone = current_path.clone();
            let history_clone = history.clone();
            let history_index_clone = history_index.clone();

            header_bar.connect_forward(move || {
                let hist = history_clone.borrow();
                let mut idx = history_index_clone.borrow_mut();
                if (*idx as usize) < hist.len() - 1 {
                    *idx += 1;
                    if let Some(path) = hist.get(*idx as usize) {
                        current_path_clone.replace(path.clone());
                        file_view_clone.load_directory(path);
                        path_bar_clone.set_path(path);
                    }
                }
            });
        }

        {
            // Up button
            let file_view_clone = file_view.clone();
            let path_bar_clone = path_bar.clone();
            let current_path_clone = current_path.clone();
            let history_clone = history.clone();
            let history_index_clone = history_index.clone();

            header_bar.connect_up(move || {
                let current = current_path_clone.borrow().clone();
                if let Some(parent) = current.parent() {
                    let parent_path = parent.to_path_buf();
                    current_path_clone.replace(parent_path.clone());
                    file_view_clone.load_directory(&parent_path);
                    path_bar_clone.set_path(&parent_path);

                    // Add to history
                    let mut hist = history_clone.borrow_mut();
                    let mut idx = history_index_clone.borrow_mut();
                    *idx += 1;
                    hist.truncate(*idx as usize);
                    hist.push(parent_path);
                }
            });
        }

        {
            // Home button
            let file_view_clone = file_view.clone();
            let path_bar_clone = path_bar.clone();
            let current_path_clone = current_path.clone();
            let history_clone = history.clone();
            let history_index_clone = history_index.clone();

            header_bar.connect_home(move || {
                if let Some(home) = dirs::home_dir() {
                    current_path_clone.replace(home.clone());
                    file_view_clone.load_directory(&home);
                    path_bar_clone.set_path(&home);

                    // Add to history
                    let mut hist = history_clone.borrow_mut();
                    let mut idx = history_index_clone.borrow_mut();
                    *idx += 1;
                    hist.truncate(*idx as usize);
                    hist.push(home);
                }
            });
        }

        {
            // Search
            let file_view_clone = file_view.clone();

            header_bar.connect_search(move |query| {
                file_view_clone.filter(&query);
            });
        }

        {
            // New folder
            let file_view_clone = file_view.clone();
            let current_path_clone = current_path.clone();
            let window_clone = window.clone();

            header_bar.connect_new_folder(move || {
                let current = current_path_clone.borrow().clone();
                crate::widgets::show_new_folder_dialog(&window_clone, &current, &file_view_clone);
            });
        }

        {
            // New file
            let file_view_clone = file_view.clone();
            let current_path_clone = current_path.clone();
            let window_clone = window.clone();

            header_bar.connect_new_file(move || {
                let current = current_path_clone.borrow().clone();
                crate::widgets::show_new_file_dialog(&window_clone, &current, &file_view_clone);
            });
        }

        // Connect file view context menu actions
        {
            let clipboard_clone = clipboard.clone();
            file_view.connect_copy(move |paths| {
                clipboard_clone.borrow_mut().copy(paths);
            });
        }

        {
            let clipboard_clone = clipboard.clone();
            file_view.connect_cut(move |paths| {
                clipboard_clone.borrow_mut().cut(paths);
            });
        }

        {
            let clipboard_clone = clipboard.clone();
            let current_path_clone = current_path.clone();
            let file_view_clone = file_view.clone();

            file_view.connect_paste(move || {
                let current = current_path_clone.borrow().clone();
                let mut clip = clipboard_clone.borrow_mut();
                if let Err(e) = clip.paste(&current) {
                    eprintln!("Paste error: {}", e);
                }
                drop(clip);
                file_view_clone.load_directory(&current);
            });
        }

        {
            let current_path_clone = current_path.clone();
            let file_view_clone = file_view.clone();

            file_view.connect_delete(move |paths| {
                for path in paths {
                    if let Err(e) = FileOperations::delete(&path) {
                        eprintln!("Delete error: {}", e);
                    }
                }
                let current = current_path_clone.borrow().clone();
                file_view_clone.load_directory(&current);
            });
        }

        {
            let current_path_clone = current_path.clone();
            let file_view_clone = file_view.clone();
            let window_clone = window.clone();

            file_view.connect_rename(move |path| {
                crate::widgets::show_rename_dialog(
                    &window_clone,
                    &path,
                    &current_path_clone,
                    &file_view_clone,
                );
            });
        }

        {
            // Pin to sidebar
            let sidebar_clone = sidebar.clone();
            file_view.connect_pin(move |path| {
                if let Err(e) = crate::core::PinnedManager::add(&path) {
                    eprintln!("Failed to pin folder: {}", e);
                } else {
                    // Refresh sidebar to show newly pinned folder
                    sidebar_clone.refresh();
                }
            });
        }

        {
            // Settings button
            let window_clone = window.clone();
            header_bar.connect_settings(move || {
                let settings = SettingsWindow::new(&window_clone);
                settings.present();
            });
        }

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }
}
