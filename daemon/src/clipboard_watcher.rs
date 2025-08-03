use anyhow::Result;
use arboard::Clipboard;
use log::{debug, error, info, warn};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use crate::database::Database;

pub struct ClipboardWatcher {
    db: Arc<Mutex<Database>>,
    last_content: Arc<Mutex<Option<String>>>,
}

impl ClipboardWatcher {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        Self {
            db,
            last_content: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn start_monitoring(&self) -> Result<()> {
        info!("Starting efficient clipboard monitoring with arboard");

        // Create clipboard instance
        let mut clipboard = match Clipboard::new() {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to create clipboard instance: {}", e);
                return Err(anyhow::anyhow!("Failed to initialize clipboard: {}", e));
            }
        };

        // Get initial clipboard content
        match clipboard.get_text() {
            Ok(content) => {
                if !content.trim().is_empty() {
                    self.process_clipboard_content(content).await;
                }
            }
            Err(e) => {
                debug!("No initial clipboard content: {}", e);
            }
        }

        // Main monitoring loop with adaptive polling
        let mut poll_interval = Duration::from_millis(100); // Start with fast polling
        let mut last_change = Instant::now();
        
        loop {
            match clipboard.get_text() {
                Ok(content) => {
                    if !content.trim().is_empty() {
                        let mut last = self.last_content.lock().await;
                        
                        if last.as_ref() != Some(&content) {
                            // Content changed
                            debug!("New clipboard content detected: {} bytes", content.len());
                            
                            // Update database
                            if let Err(e) = self.db.lock().await.add_entry(content.clone()).await {
                                error!("Failed to add clipboard entry: {}", e);
                            } else {
                                info!("Added new clipboard entry to database");
                            }
                            
                            *last = Some(content);
                            last_change = Instant::now();
                            
                            // Reset to fast polling after a change
                            poll_interval = Duration::from_millis(100);
                        } else {
                            // No change, gradually slow down polling
                            let time_since_change = last_change.elapsed();
                            
                            if time_since_change > Duration::from_secs(10) {
                                // After 10 seconds of no changes, poll every second
                                poll_interval = Duration::from_secs(1);
                            } else if time_since_change > Duration::from_secs(5) {
                                // After 5 seconds, poll every 500ms
                                poll_interval = Duration::from_millis(500);
                            } else if time_since_change > Duration::from_secs(2) {
                                // After 2 seconds, poll every 250ms
                                poll_interval = Duration::from_millis(250);
                            } else {
                                // Otherwise keep fast polling
                                poll_interval = Duration::from_millis(100);
                            }
                        }
                    }
                }
                Err(e) => {
                    // Handle errors gracefully
                    let error_str = e.to_string();
                    if error_str.contains("empty") || error_str.contains("Empty") {
                        debug!("Clipboard is empty");
                        poll_interval = Duration::from_millis(500);
                    } else if error_str.contains("not available") || error_str.contains("format") {
                        debug!("Clipboard content not available (non-text data)");
                        poll_interval = Duration::from_millis(500);
                    } else {
                        warn!("Failed to read clipboard: {}", e);
                        // Re-create clipboard instance on other errors
                        match Clipboard::new() {
                            Ok(c) => {
                                clipboard = c;
                                info!("Recreated clipboard instance after error");
                            }
                            Err(e) => {
                                error!("Failed to recreate clipboard: {}", e);
                                // Wait longer before retrying
                                poll_interval = Duration::from_secs(2);
                            }
                        }
                    }
                }
            }
            
            sleep(poll_interval).await;
        }
    }

    async fn process_clipboard_content(&self, content: String) {
        let mut last = self.last_content.lock().await;
        
        if last.as_ref() != Some(&content) {
            debug!("Processing initial clipboard content: {} bytes", content.len());
            
            // Update database
            if let Err(e) = self.db.lock().await.add_entry(content.clone()).await {
                error!("Failed to add clipboard entry: {}", e);
            } else {
                info!("Added initial clipboard entry to database");
            }
            
            *last = Some(content);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_available() {
        // Test if we can create a clipboard instance
        match Clipboard::new() {
            Ok(_) => println!("Clipboard available"),
            Err(e) => println!("Clipboard not available: {}", e),
        }
        
        // This test always passes - it's just informational
        assert!(true);
    }
}