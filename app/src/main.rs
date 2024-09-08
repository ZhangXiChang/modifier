#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{anyhow, Result};
use tauri::Manager;
use window_shadows::set_shadow;

#[tokio::main]
async fn main() -> Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            if let Some(window) = app.get_window("main") {
                set_shadow(window, true).map_err(|e| anyhow!("{}", e))?;
            }
            Ok(())
        })
        .manage(System::default())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())?;
    Ok(())
}

#[derive(Default)]
struct System {}
