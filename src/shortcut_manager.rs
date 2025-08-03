use anyhow::Result;
use std::process::Command;

pub struct ShortcutManager;

impl ShortcutManager {
    /// Check if the Clippoo shortcut is already registered
    pub fn is_shortcut_registered() -> Result<bool> {
        let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
        
        if desktop.contains("GNOME") {
                let output = Command::new("gsettings")
                    .args(&["get", "org.gnome.settings-daemon.plugins.media-keys", "custom-keybindings"])
                    .output()?;
                
            let result = String::from_utf8_lossy(&output.stdout);
            Ok(result.contains("clippoo"))
        } else {
            Ok(false) // For other desktops, assume not registered
        }
    }
    
    /// Register the global shortcut for Clippoo
    pub fn register_shortcut() -> Result<()> {
        let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
        
        if desktop.contains("GNOME") {
            Self::register_gnome_shortcut()
        } else {
            log::warn!("Automatic shortcut registration not supported for desktop: {}", desktop);
            Ok(())
        }
    }
    
    fn register_gnome_shortcut() -> Result<()> {
        // Get current custom keybindings
        let output = Command::new("gsettings")
            .args(&["get", "org.gnome.settings-daemon.plugins.media-keys", "custom-keybindings"])
            .output()?;
        
        let current = String::from_utf8_lossy(&output.stdout);
        let current = current.trim();
        
        // Add clippoo to the list if not already present
        if !current.contains("clippoo") {
            let new_bindings = if current == "@as []" || current == "[]" {
                "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clippoo/']".to_string()
            } else {
                format!("{}, '/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clippoo/']", 
                        &current[..current.len()-1])
            };
            
            Command::new("gsettings")
                .args(&["set", "org.gnome.settings-daemon.plugins.media-keys", 
                       "custom-keybindings", &new_bindings])
                .output()?;
        }
        
        // Set the shortcut properties
        let schema = "org.gnome.settings-daemon.plugins.media-keys.custom-keybinding";
        let path = "/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/clippoo/";
        
        // Get home directory
        let home = std::env::var("HOME")?;
        let command = format!("{}/.local/bin/clippoo-ui.sh", home);
        
        // Set name
        Command::new("gsettings")
            .args(&["set", &format!("{}:{}", schema, path), "name", "Clippoo Clipboard Manager"])
            .output()?;
            
        // Set command
        Command::new("gsettings")
            .args(&["set", &format!("{}:{}", schema, path), "command", &command])
            .output()?;
            
        // Set binding
        Command::new("gsettings")
            .args(&["set", &format!("{}:{}", schema, path), "binding", "<Super><Shift>v"])
            .output()?;
        
        log::info!("Successfully registered Meta+Shift+V shortcut for Clippoo");
        Ok(())
    }
    
    /// Show a notification to the user about shortcut registration
    pub fn notify_user_about_shortcut() -> Result<()> {
        Command::new("notify-send")
            .args(&[
                "Clippoo", 
                "Press Meta+Shift+V to open clipboard history",
                "--icon=edit-paste"
            ])
            .spawn()?;
        Ok(())
    }
}