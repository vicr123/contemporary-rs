use crate::platform_support::macos::apple_interface_theme_changed_listener::setup_apple_interface_theme_changed_listener;
use crate::platform_support::macos::apple_workspace_a11y_options_changed_listener::setup_apple_workspace_a11y_options_changed_listener;
use gpui::App;

pub fn setup_macos(cx: &mut App) {
    setup_apple_interface_theme_changed_listener(cx);
    setup_apple_workspace_a11y_options_changed_listener(cx);
}
