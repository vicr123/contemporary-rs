use gpui::App;

#[cfg(target_os = "macos")]
pub mod macos;

pub fn setup_platform(cx: &mut App) {
    #[cfg(target_os = "macos")]
    macos::setup::setup_macos(cx)
}
