// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use slint::include_modules;

include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
   let main_window = MainWindow::new()?;
   main_window.run()?;
   Ok(())
}
