use gpui::App;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

pub fn setup_platform(cx: &mut App) {
    #[cfg(target_os = "macos")]
    macos::setup::setup_macos(cx);

    #[cfg(target_os = "windows")]
    windows::setup::setup_windows(cx);
}
