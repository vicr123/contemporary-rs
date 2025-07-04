pub mod contemporary;
pub mod theme;
mod rgb;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;
