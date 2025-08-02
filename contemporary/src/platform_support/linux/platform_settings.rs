use crate::platform_support::platform_settings::PlatformSettings;
use gpui::{Decorations, px};
use std::rc::Rc;

pub fn create_linux_platform_settings() -> PlatformSettings {
    PlatformSettings {
        resize_grip_size: Rc::new(|window| match window.window_decorations() {
            Decorations::Server => px(0.),
            Decorations::Client { .. } => {
                if window.is_maximized() {
                    px(0.)
                } else {
                    px(8.)
                }
            }
        }),
        ..PlatformSettings::new()
    }
}
