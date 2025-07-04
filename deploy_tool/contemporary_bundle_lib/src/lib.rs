pub mod icon;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;
pub mod tool_setup;

/// A tuple containing the major, minor, and patch version number components, in that order.
pub type VersionTuple = (u64, u64, u64);
