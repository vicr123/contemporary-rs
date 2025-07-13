use crate::platform_support::platform_settings::PlatformSettings;
use objc2_app_kit::NSWorkspace;
use std::time::Duration;

pub fn create_macos_platform_settings() -> PlatformSettings {
    let reduced_motion = unsafe {
        let workspace = NSWorkspace::sharedWorkspace();
        workspace.accessibilityDisplayShouldReduceMotion()
    };

    let base_platform_settings = PlatformSettings::new();

    PlatformSettings {
        animation_duration: if reduced_motion {
            Duration::ZERO
        } else {
            base_platform_settings.animation_duration
        },

        ..base_platform_settings
    }
}
