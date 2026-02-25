mod db;
mod commands;

use commands::{AppState, start_background_tasks};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};

#[tauri::command]
async fn show_window(app_handle: AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.show().map_err(|e: tauri::Error| e.to_string())?;
        window.set_focus().map_err(|e: tauri::Error| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn hide_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.hide().map_err(|e: tauri::Error| e.to_string())?;
    }
    Ok(())
}

fn main() {
    let db = db::Database::new().expect("Failed to initialize database");
    let device_status = Arc::new(Mutex::new(commands::DeviceStatus::default()));
    
    let app_state = AppState {
        db: Arc::new(db),
        device_status,
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(app_state.clone())
        .invoke_handler(tauri::generate_handler![
            commands::add_transcript,
            commands::get_transcripts,
            commands::mark_synced,
            commands::simulate_sync,
            commands::get_device_status,
            commands::get_unsynced_count,
            commands::toggle_device_connection,
            show_window,
            hide_window,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            let state = app.state::<AppState>();
            let state_arc = AppState {
                db: state.db.clone(),
                device_status: state.device_status.clone(),
            };
            
            start_background_tasks(app_handle, Arc::new(state_arc));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}