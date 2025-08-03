use anyhow::Result;
use gtk4::prelude::*;
use gtk4::{
    gdk, glib, Application, ApplicationWindow, Box, Entry, EventControllerKey, Label, ListBox,
    ListBoxRow, Orientation, ScrolledWindow, SelectionMode,
};
use log::{debug, error, info, warn};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use tokio::runtime::Runtime;

use crate::database::{ClipboardEntry, Database};

pub struct ClipboardPopup {
    window: ApplicationWindow,
    list_box: ListBox,
    search_entry: Entry,
    entries: Rc<RefCell<Vec<ClipboardEntry>>>,
    filtered_entries: Rc<RefCell<Vec<ClipboardEntry>>>,
    db: Arc<tokio::sync::Mutex<Database>>,
    runtime: Arc<Runtime>,
}

impl ClipboardPopup {
    pub fn new(app: &Application) -> Result<Self> {
        let runtime = Arc::new(Runtime::new()?);
        let db = runtime.block_on(async {
            Database::new().await
        })?;
        let db = Arc::new(tokio::sync::Mutex::new(db));
        
        // Create window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Clippoo")
            .default_width(600)
            .default_height(600)
            .decorated(false)
            .modal(true)
            .resizable(false)
            .build();
        
        // Add CSS class for styling
        window.add_css_class("clipboard-popup");
        
        // Center the window on present
        
        // Create main container
        let main_box = Box::new(Orientation::Vertical, 8);
        main_box.add_css_class("popup-container");
        main_box.set_size_request(580, 580);  // Slightly smaller than window for padding
        
        // Create search entry
        let search_entry = Entry::builder()
            .placeholder_text("Type to search...")
            .visible(false)
            .build();
        search_entry.add_css_class("search-entry");
        main_box.append(&search_entry);
        
        // Create scrolled window
        let scrolled = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .vexpand(true)
            .build();
        
        // Create list box
        let list_box = ListBox::new();
        list_box.set_selection_mode(SelectionMode::Single);
        list_box.add_css_class("clipboard-list");
        
        scrolled.set_child(Some(&list_box));
        main_box.append(&scrolled);
        
        window.set_child(Some(&main_box));
        
        let popup = Self {
            window,
            list_box,
            search_entry,
            entries: Rc::new(RefCell::new(Vec::new())),
            filtered_entries: Rc::new(RefCell::new(Vec::new())),
            db,
            runtime,
        };
        
        popup.setup_keyboard_navigation();
        popup.setup_search_handler();
        popup.load_entries()?;
        
        Ok(popup)
    }
    
    pub fn show(&self) -> Result<()> {
        // Make sure window has proper size
        self.window.set_default_size(600, 600);
        self.window.set_size_request(600, 600);
        
        // Present the window
        self.window.present();
        
        // Select first item after presenting
        if let Some(first_row) = self.list_box.row_at_index(0) {
            self.list_box.select_row(Some(&first_row));
            first_row.grab_focus();
        }
        
        // Ensure window has focus
        self.window.grab_focus();
        
        Ok(())
    }
    
    fn load_entries(&self) -> Result<()> {
        let db = self.db.clone();
        let runtime = self.runtime.clone();
        
        let entries = runtime.block_on(async {
            db.lock().await.get_recent_entries(60).await
        })?;
        
        self.entries.replace(entries.clone());
        self.filtered_entries.replace(entries.clone());
        
        self.update_list_display();
        
        Ok(())
    }
    
    fn update_list_display(&self) {
        // Clear existing rows
        while let Some(child) = self.list_box.first_child() {
            self.list_box.remove(&child);
        }
        
        let entries = self.filtered_entries.borrow();
        
        // Add new rows
        for (index, entry) in entries.iter().enumerate() {
            let row = self.create_row(entry, index);
            self.list_box.append(&row);
        }
    }
    
    fn create_row(&self, entry: &ClipboardEntry, index: usize) -> ListBoxRow {
        let row = ListBoxRow::new();
        row.add_css_class("clipboard-row");
        
        let hbox = Box::new(Orientation::Horizontal, 12);
        hbox.set_margin_start(12);
        hbox.set_margin_end(12);
        hbox.set_margin_top(8);
        hbox.set_margin_bottom(8);
        
        // Index label
        let index_label = Label::new(Some(&format!("{}.", index + 1)));
        index_label.add_css_class("index-label");
        index_label.set_width_request(30);
        hbox.append(&index_label);
        
        // Content label (truncated)
        let content = if entry.content.len() > 80 {
            format!("{}...", &entry.content[..80])
        } else {
            entry.content.clone()
        };
        
        let content_label = Label::new(Some(&content));
        content_label.add_css_class("content-label");
        content_label.set_xalign(0.0);
        content_label.set_hexpand(true);
        content_label.set_ellipsize(pango::EllipsizeMode::End);
        hbox.append(&content_label);
        
        // Default indicator
        if entry.is_default {
            let default_label = Label::new(Some("●"));
            default_label.add_css_class("default-indicator");
            hbox.append(&default_label);
        }
        
        row.set_child(Some(&hbox));
        row
    }
    
    fn setup_keyboard_navigation(&self) {
        let key_controller = EventControllerKey::new();
        
        let window = self.window.clone();
        let list_box = self.list_box.clone();
        let entries = self.entries.clone();
        let filtered_entries = self.filtered_entries.clone();
        let search_entry = self.search_entry.clone();
        let db = self.db.clone();
        let runtime = self.runtime.clone();
        
        key_controller.connect_key_pressed(move |_, keyval, _, modifiers| {
            match keyval {
                gdk::Key::Escape => {
                    window.close();
                    glib::Propagation::Stop
                }
                gdk::Key::v if modifiers.contains(gdk::ModifierType::SUPER_MASK) && modifiers.contains(gdk::ModifierType::SHIFT_MASK) => {
                    // Meta+Shift+V cycles through list items
                    navigate_list(&list_box, 1);
                    glib::Propagation::Stop
                }
                gdk::Key::Return | gdk::Key::KP_Enter => {
                    if let Some(selected_row) = list_box.selected_row() {
                        let index = selected_row.index() as usize;
                        let entries_ref = filtered_entries.borrow();
                        
                        if let Some(entry) = entries_ref.get(index) {
                            let content = entry.content.clone();
                            let entry_id = entry.id;
                            
                            // Set as default in database
                            let db_clone = db.clone();
                            let runtime_clone = runtime.clone();
                            runtime_clone.block_on(async {
                                if let Err(e) = db_clone.lock().await.set_default_entry(entry_id).await {
                                    error!("Failed to set default entry: {}", e);
                                }
                            });
                            
                            // Copy to clipboard
                            if let Err(e) = copy_to_clipboard(&content) {
                                error!("Failed to copy to clipboard: {}", e);
                            }
                            
                            // Spawn auto-paste as a detached process
                            spawn_auto_paste();
                            
                            // Close window
                            window.close();
                        }
                    }
                    glib::Propagation::Stop
                }
                gdk::Key::n if modifiers.contains(gdk::ModifierType::CONTROL_MASK) => {
                    navigate_list(&list_box, 1);
                    glib::Propagation::Stop
                }
                gdk::Key::p if modifiers.contains(gdk::ModifierType::CONTROL_MASK) => {
                    navigate_list(&list_box, -1);
                    glib::Propagation::Stop
                }
                gdk::Key::Down => {
                    navigate_list(&list_box, 1);
                    glib::Propagation::Stop
                }
                gdk::Key::Up => {
                    navigate_list(&list_box, -1);
                    glib::Propagation::Stop
                }
                k if k.to_unicode().map(|c| c.is_ascii_digit()).unwrap_or(false) => {
                    // Quick select by number
                    if let Some(digit) = keyval.to_unicode() {
                        if let Some(num) = digit.to_digit(10) {
                            if num >= 1 && num <= 9 {
                                let index = (num - 1) as i32;
                                if let Some(row) = list_box.row_at_index(index) {
                                    list_box.select_row(Some(&row));
                                    // Trigger the same action as Enter key
                                    if let Some(selected_row) = list_box.selected_row() {
                                        let index = selected_row.index() as usize;
                                        let entries_ref = filtered_entries.borrow();
                                        
                                        if let Some(entry) = entries_ref.get(index) {
                                            let content = entry.content.clone();
                                            let entry_id = entry.id;
                                            
                                            // Set as default in database
                                            let db_clone = db.clone();
                                            let runtime_clone = runtime.clone();
                                            runtime_clone.block_on(async {
                                                if let Err(e) = db_clone.lock().await.set_default_entry(entry_id).await {
                                                    error!("Failed to set default entry: {}", e);
                                                }
                                            });
                                            
                                            // Copy to clipboard
                                            if let Err(e) = copy_to_clipboard(&content) {
                                                error!("Failed to copy to clipboard: {}", e);
                                            }
                                            
                                            // Spawn auto-paste as a detached process
                                            spawn_auto_paste();
                                            
                                            // Close window
                                            window.close();
                                        }
                                    }
                                }
                            }
                        }
                    }
                    glib::Propagation::Stop
                }
                gdk::Key::slash => {
                    // Show search entry
                    search_entry.set_visible(true);
                    search_entry.grab_focus();
                    glib::Propagation::Stop
                }
                _ => glib::Propagation::Proceed,
            }
        });
        
        self.window.add_controller(key_controller);
    }
    
    fn setup_search_handler(&self) {
        let entries = self.entries.clone();
        let filtered_entries = self.filtered_entries.clone();
        
        // Setup change handler for search
        let entries_clone = entries.clone();
        let filtered_clone = filtered_entries.clone();
        let search_entry_clone = self.search_entry.clone();
        
        let popup_weak = Rc::downgrade(&Rc::new(RefCell::new(())));
        let list_box_clone = self.list_box.clone();
        
        search_entry_clone.connect_changed(move |entry| {
            let query = entry.text().to_string().to_lowercase();
            let all_entries = entries_clone.borrow();
            
            if query.is_empty() {
                // Show all entries if search is empty
                filtered_clone.replace(all_entries.clone());
            } else {
                // Filter entries based on search query (limit to top 15 matches)
                let filtered: Vec<ClipboardEntry> = all_entries
                    .iter()
                    .filter(|entry| entry.content.to_lowercase().contains(&query))
                    .take(15)
                    .cloned()
                    .collect();
                filtered_clone.replace(filtered);
            }
            
            // Update the list display
            while let Some(child) = list_box_clone.first_child() {
                list_box_clone.remove(&child);
            }
            
            let entries_to_show = filtered_clone.borrow();
            for (index, entry) in entries_to_show.iter().enumerate() {
                let row = ListBoxRow::new();
                row.add_css_class("clipboard-row");
                
                let hbox = Box::new(Orientation::Horizontal, 12);
                hbox.set_margin_start(12);
                hbox.set_margin_end(12);
                hbox.set_margin_top(8);
                hbox.set_margin_bottom(8);
                
                // Index label
                let index_label = Label::new(Some(&format!("{}.", index + 1)));
                index_label.add_css_class("index-label");
                index_label.set_width_request(30);
                hbox.append(&index_label);
                
                // Content label (truncated)
                let content = if entry.content.len() > 80 {
                    format!("{}...", &entry.content[..80])
                } else {
                    entry.content.clone()
                };
                
                let content_label = Label::new(Some(&content));
                content_label.add_css_class("content-label");
                content_label.set_xalign(0.0);
                content_label.set_hexpand(true);
                content_label.set_ellipsize(pango::EllipsizeMode::End);
                hbox.append(&content_label);
                
                // Default indicator
                if entry.is_default {
                    let default_label = Label::new(Some("●"));
                    default_label.add_css_class("default-indicator");
                    hbox.append(&default_label);
                }
                
                row.set_child(Some(&hbox));
                list_box_clone.append(&row);
            }
            
            // Select first row
            if let Some(first_row) = list_box_clone.row_at_index(0) {
                list_box_clone.select_row(Some(&first_row));
            }
        });
        
        // Handle escape key in search entry
        let search_key_controller = EventControllerKey::new();
        let search_entry_clone = self.search_entry.clone();
        let entries_clone = self.entries.clone();
        let filtered_clone = self.filtered_entries.clone();
        
        search_key_controller.connect_key_pressed(move |_, keyval, _, _| {
            if keyval == gdk::Key::Escape {
                // Clear search and hide entry
                search_entry_clone.set_text("");
                search_entry_clone.set_visible(false);
                // Restore all entries
                filtered_clone.replace(entries_clone.borrow().clone());
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        });
        
        self.search_entry.add_controller(search_key_controller);
    }
}

fn navigate_list(list_box: &ListBox, direction: i32) {
    let current_index = list_box
        .selected_row()
        .map(|row| row.index())
        .unwrap_or(-1);
    
    let new_index = current_index + direction;
    
    if let Some(new_row) = list_box.row_at_index(new_index) {
        list_box.select_row(Some(&new_row));
        new_row.grab_focus();
    }
}

fn copy_to_clipboard(content: &str) -> Result<()> {
    use arboard::Clipboard;
    
    let mut clipboard = Clipboard::new()
        .map_err(|e| anyhow::anyhow!("Failed to access clipboard: {}", e))?;
    
    clipboard.set_text(content)
        .map_err(|e| anyhow::anyhow!("Failed to set clipboard content: {}", e))?;
    
    info!("Copied to clipboard using arboard");
    Ok(())
}

fn spawn_auto_paste() {
    use std::process::Command;
    
    // Simpler approach: try to detect if we need terminal paste by checking focused window
    let paste_script = r#"
sleep 0.2

# Try to get the focused window name using GNOME's tools
WINDOW_NAME=$(gdbus call --session --dest org.gnome.Shell --object-path /org/gnome/Shell --method org.gnome.Shell.Eval 'global.display.focus_window.get_title()' 2>/dev/null | grep -o '"[^"]*"' | head -1 | tr -d '"' | tr '[:upper:]' '[:lower:]')

# If that fails, try getting active window process
if [ -z "$WINDOW_NAME" ]; then
    # Try to get the active process name
    ACTIVE_PID=$(gdbus call --session --dest org.gnome.Shell --object-path /org/gnome/Shell --method org.gnome.Shell.Eval 'global.display.focus_window.get_pid()' 2>/dev/null | grep -o '[0-9]*' | head -1)
    if [ -n "$ACTIVE_PID" ]; then
        WINDOW_NAME=$(cat /proc/$ACTIVE_PID/comm 2>/dev/null | tr '[:upper:]' '[:lower:]')
    fi
fi

# Check if it's likely a terminal based on window title or process name
case "$WINDOW_NAME" in
    *terminal*|*term*|*konsole*|*alacritty*|*kitty*|*tilix*|gnome-terminal-server|foot|wezterm)
        # Terminal - use Ctrl+Shift+V
        ydotool key ctrl+shift+v
        ;;
    *)
        # Regular application - use Ctrl+V
        ydotool key ctrl+v
        ;;
esac
"#;
    
    // Create a detached process that will survive after the UI closes
    match Command::new("sh")
        .arg("-c")
        .arg(paste_script)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
    {
        Ok(_) => {
            info!("Spawned auto-paste process");
        }
        Err(e) => {
            error!("Failed to spawn auto-paste: {}", e);
        }
    }
}

fn simulate_paste() -> Result<()> {
    use std::process::Command;
    use std::thread;
    use std::time::Duration;
    
    // Small delay to ensure window closes first
    thread::sleep(Duration::from_millis(200));
    
    // Try ydotool first with regular Ctrl+V
    info!("Attempting to auto-paste with ydotool...");
    let ydotool_result = Command::new("ydotool")
        .args(&["key", "ctrl+v"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output();
    
    match ydotool_result {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            info!("ydotool exit status: {}, stdout: {}, stderr: {}", output.status.success(), stdout, stderr);
            
            if output.status.success() {
                info!("Pasted using ydotool");
                return Ok(());
            } else {
                warn!("ydotool failed with stderr: {}", stderr);
            }
        }
        Err(e) => {
            warn!("ydotool not found or failed to run: {}", e);
        }
    }
    
    // Fallback to wtype
    let wtype_result = Command::new("wtype")
        .args(&["-M", "ctrl", "-P", "v", "-m", "ctrl"])
        .stderr(std::process::Stdio::piped())
        .output();
    
    match wtype_result {
        Ok(output) => {
            if output.status.success() {
                info!("Pasted using wtype");
                return Ok(());
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                debug!("wtype failed: {}", stderr);
            }
        }
        Err(e) => {
            debug!("wtype not found or failed to run: {}", e);
        }
    }
    
    // If both fail, just log a warning but don't error out
    // The content is already in the clipboard, user can paste manually
    warn!("Auto-paste failed. Content is in clipboard - press Ctrl+V to paste manually.");
    Ok(())
}