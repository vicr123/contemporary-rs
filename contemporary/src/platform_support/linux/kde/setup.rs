use gpui::App;
use crate::platform_support::linux::kde::setup_kde_fonts_changed_listener::setup_kde_fonts_changed_listener;

pub fn setup_kde(cx: &mut App) {
    setup_kde_fonts_changed_listener(cx);
}
