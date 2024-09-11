#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod sys_api;

use anyhow::{anyhow, Result};
use sys_api::ProcessInfo;
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
        .invoke_handler(tauri::generate_handler![get_all_process_info, test])
        .run(tauri::generate_context!())?;
    Ok(())
}

#[derive(Default)]
struct System {}

#[tauri::command]
fn get_all_process_info() -> Result<Vec<ProcessInfo>, String> {
    sys_api::get_all_process_info().map_err(|err| err.to_string())
}

#[tauri::command]
fn test(process_id: u32) -> Result<(), String> {
    sys_api::test(process_id).map_err(|err| err.to_string())
}
