pub mod contemporary;
mod rgb;
pub mod theme;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;
