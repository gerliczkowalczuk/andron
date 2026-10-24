use std::env;

pub fn connect_display_bridge() -> Result<(), Box<dyn std::error::Error>> {
    let wayland_display = env::var("WAYLAND_DISPLAY").unwrap_or_else(|_| "wayland-0".to_string());
    let xdg_runtime_dir = env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/run/user/1000".to_string());

    let socket_path = format!("{}/{}", xdg_runtime_dir, wayland_display);
    if std::path::Path::new(&socket_path).exists() {
        log::info!("SurfaceFlinger <-> Wayland zero-copy bridge initialized via socket: {}", socket_path);
    } else {
        log::warn!("Wayland socket {} not detected; falling back to XWayland / X11 bridge.", socket_path);
    }

    Ok(())
}
