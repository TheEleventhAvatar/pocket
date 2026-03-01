fn main() {
    // Fix for EGL/Webview issues on certain Linux distros
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    anchor_lib::run()
}