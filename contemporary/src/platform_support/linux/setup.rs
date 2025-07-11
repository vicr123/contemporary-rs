use crate::platform_support::linux::setup_xdg_portal_interface_theme_changed_listener::setup_xdg_portal_interface_theme_changed_listener;
use gpui::App;
use crate::platform_support::linux::desktop_environment::DesktopEnvironment;
use crate::platform_support::linux::gnome::setup::setup_gnome;
use crate::platform_support::linux::kde::setup::setup_kde;

pub fn setup_linux(cx: &mut App) {
    setup_xdg_portal_interface_theme_changed_listener(cx);
    
    match DesktopEnvironment::current() {
        Some(DesktopEnvironment::KDE) => setup_kde(cx),
        Some(DesktopEnvironment::GNOME) => setup_gnome(cx),
        None => {}
    }
}
