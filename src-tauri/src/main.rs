fn main() {
    // Set this BEFORE initializing the Tauri app
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}