// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use notify_rust::Notification;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    

    ui.global::<Logic>().on_notify(|content| {
        let _ = Notification::new()
    .summary("Tulipe")
    .body(content.as_str())
    .show();
    });

    ui.run()?;
    
    

    Ok(())
}
