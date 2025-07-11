use gpui::App;
use crate::platform_support::linux::xdg_portal_interface_theme_changed_listener::setup_xdg_portal_interface_theme_changed_listener;

pub fn setup_linux(cx: &mut App) {
    setup_xdg_portal_interface_theme_changed_listener(cx);
}
