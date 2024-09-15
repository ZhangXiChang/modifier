#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lock;
mod win_api;

use anyhow::{anyhow, Result};
use serde::Serialize;
use tauri::Manager;
use win_api::SystemSnapshot;
use window_shadows::set_shadow;

#[tokio::main]
async fn main() -> Result<()> {
    let qwdqwd = get_system_process_info_list().unwrap();
    for qwdqwdqwdwqd in qwdqwd {
        println!("{}", qwdqwdqwdwqd.exe_file_name);
    }
    // tauri::Builder::default()
    //     .setup(|app| {
    //         if let Some(window) = app.get_window("main") {
    //             set_shadow(window, true).map_err(|e| anyhow!("{}", e))?;
    //         }
    //         Ok(())
    //     })
    //     .invoke_handler(tauri::generate_handler![get_system_process_info_list])
    //     .run(tauri::generate_context!())?;
    Ok(())
}

#[derive(Serialize)]
struct ProcessInfo {
    pid: u32,
    exe_file_name: String,
}
#[tauri::command]
fn get_system_process_info_list() -> Result<Vec<ProcessInfo>, String> {
    (|| {
        let mut process_info_list = Vec::new();
        for process_entry in SystemSnapshot::new()?.process_entry_iter() {
            if process_entry.open_process().is_ok() {
                process_info_list.push(ProcessInfo {
                    pid: process_entry.get_process_id(),
                    exe_file_name: process_entry.get_exe_file_name()?,
                })
            }
        }
        anyhow::Ok(process_info_list)
    })()
    .map_err(|err| err.to_string())
}
