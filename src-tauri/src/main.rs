fn main() {
    #[cfg(target_os = "linux")]
    {
        // Fix for EGL_BAD_PARAMETER and general WebKitGTK crashes on Linux
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
        
        // This specifically helps with NVIDIA drivers and Wayland/X11 DMABUF issues
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}