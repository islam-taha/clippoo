use anyhow::Result;
use gtk4::prelude::*;
use gtk4::{Application, CssProvider};
use log::info;

mod popup;
use popup::ClipboardPopup;

#[path = "../../src/database/mod.rs"]
mod database;

const APP_ID: &str = "com.clippoo.ClipboardManager";

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(|app| {
        if let Err(e) = build_ui(app) {
            eprintln!("Failed to build UI: {}", e);
        }
    });

    app.run();
    Ok(())
}

fn build_ui(app: &Application) -> Result<()> {
    info!("Building Clippoo UI");

    // Load CSS for styling
    let css_provider = CssProvider::new();
    css_provider.load_from_data(include_str!("style.css"));

    gtk4::style_context_add_provider_for_display(
        &gdk4::Display::default().expect("Could not connect to display"),
        &css_provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Create and show the popup
    let popup = ClipboardPopup::new(app)?;
    popup.show()?;

    Ok(())
}
