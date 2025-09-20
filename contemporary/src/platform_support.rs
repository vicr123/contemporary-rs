use gpui::App;
pub mod platform_settings;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

pub mod cx_platform_extensions;

pub fn setup_platform(cx: &mut App) {
    #[cfg(target_os = "macos")]
    macos::setup::setup_macos(cx);

    #[cfg(target_os = "windows")]
    windows::setup::setup_windows(cx);

    #[cfg(target_os = "linux")]
    linux::setup::setup_linux(cx);
}
