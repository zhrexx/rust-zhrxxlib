use std::ptr::null;
use x_win::{get_active_window, WindowInfo, XWinError};

pub fn get_windows() {
    match get_active_window() {
        Ok(active_window) => {
            println!("{:?}", active_window);
        }
        _ => {}
    }
}

