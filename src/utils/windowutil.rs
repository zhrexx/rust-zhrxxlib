use x_win::{get_open_windows, WindowInfo};

/// Uses x-win internally to get open windows
/// WARNING: Works only on systems with: Windows 10-11, Linux (On Gnome =< 45 & X server), macOS 10.6+
pub fn get_windows() -> Option<Vec<WindowInfo>> {
    #[cfg(target_os = "linux")]
    {
        let kde_full_session = std::env::var("KDE_FULL_SESSION").unwrap_or_default();
        let xdg_current_desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
        let desktop_session = std::env::var("DESKTOP_SESSION").unwrap_or_default();

        if kde_full_session == "true"
            || xdg_current_desktop.contains("KDE")
            || desktop_session.contains("plasma")
        {
            println!("WARNING: Running on Linux with KDE Plasma. Cannot get active window. Returning empty list.");
            return Some(Vec::new());
        }
    }

    match get_open_windows() {
        Ok(active_window) => {
            Some(active_window)
        }
        _ => {
            None
        }
    }
}

