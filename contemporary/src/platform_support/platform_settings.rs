use gpui::Global;
use std::time::Duration;

pub struct PlatformSettings {
    pub animation_duration: Duration,
}

impl PlatformSettings {
    pub fn new() -> Self {
        Self {
            animation_duration: Duration::from_millis(250),
        }
    }

    pub fn reload(&mut self) {
        let default = Self::default();
        self.animation_duration = default.animation_duration;
    }
}

impl Global for PlatformSettings {}

impl Default for PlatformSettings {
    #[allow(unreachable_code)]
    fn default() -> Self {
        #[cfg(target_os = "macos")]
        {
            return crate::platform_support::macos::platform_settings::create_macos_platform_settings();
        }

        #[cfg(target_os = "windows")]
        {
            return crate::platform_support::windows::platform_settings::create_windows_platform_settings();
        }

        #[cfg(target_os = "linux")]
        {
            return crate::platform_support::linux::platform_settings::create_linux_platform_settings();
        }

        Self::new()
    }
}
