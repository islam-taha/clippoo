use anyhow::Result;
use log::info;
use std::sync::Arc;
use tokio::sync::Mutex;

mod clipboard_watcher;
use clipboard_watcher::ClipboardWatcher;

#[path = "../../src/database/mod.rs"]
mod database;
use database::Database;

#[path = "../../src/shortcut_manager.rs"]
mod shortcut_manager;
use shortcut_manager::ShortcutManager;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    info!("Starting Clippoo daemon");
    
    // Check and register shortcut if needed (only on GNOME for now)
    if !ShortcutManager::is_shortcut_registered()? {
        info!("Registering keyboard shortcut...");
        if let Err(e) = ShortcutManager::register_shortcut() {
            log::warn!("Failed to register shortcut automatically: {}", e);
            log::info!("Please run: ~/.local/share/clippoo/scripts/setup-shortcuts.sh");
        } else {
            // Notify user about the shortcut
            let _ = ShortcutManager::notify_user_about_shortcut();
        }
    }
    
    // Initialize database
    let db = Arc::new(Mutex::new(Database::new().await?));
    info!("Database initialized");
    
    // Create clipboard watcher
    let watcher = ClipboardWatcher::new(db.clone());
    
    // Start monitoring clipboard
    info!("Starting clipboard monitoring");
    watcher.start_monitoring().await?;
    
    Ok(())
}