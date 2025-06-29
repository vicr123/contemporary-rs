pub mod icon;
pub mod linux;
pub mod macos;
pub mod tool_setup;

/// A tuple containing the major, minor, and patch version number components, in that order.
pub type VersionTuple = (u64, u64, u64);
