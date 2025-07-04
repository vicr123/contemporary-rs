use gpui::App;
use crate::platform_support::windows::setup_windows_color_values_changed_listener::setup_windows_color_values_changed_listener;

pub fn setup_windows(cx: &mut App) {
    setup_windows_color_values_changed_listener(cx)
}
