#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lock;
mod win_api;

use std::sync::Mutex;

use anyhow::{anyhow, Result};
use lock::{MutexLock, Pointer};
use serde::Serialize;
use tauri::Manager;
use win_api::{Process, SystemSnapshot};
use window_shadows::set_shadow;

#[derive(Serialize)]
struct ProcessInfo {
    pid: u32,
    exe_file_name: String,
}

#[derive(Default)]
struct AppState {
    opened_process: Pointer<Mutex<Option<Process>>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            if let Some(window) = app.get_window("main") {
                set_shadow(window, true).map_err(|e| anyhow!("{}", e))?;
            }
            Ok(())
        })
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            get_system_process_info_list,
            open_process
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}

#[tauri::command]
fn get_system_process_info_list() -> Result<Vec<ProcessInfo>, String> {
    (|| {
        let mut process_info_list = Vec::new();
        for process_entry in SystemSnapshot::new()?.process_entry_iter() {
            if process_entry.open_process().is_ok() {
                process_info_list.push(ProcessInfo {
                    pid: process_entry.get_pid(),
                    exe_file_name: process_entry.get_exe_file_name()?,
                })
            }
        }
        anyhow::Ok(process_info_list)
    })()
    .map_err(|err| err.to_string())
}

#[tauri::command]
fn open_process(app_state: tauri::State<AppState>, pid: u32) -> Result<(), String> {
    (|| {
        *app_state.opened_process.lock() = Some(Process::new(pid)?);
        anyhow::Ok(())
    })()
    .map_err(|err| err.to_string())
}
