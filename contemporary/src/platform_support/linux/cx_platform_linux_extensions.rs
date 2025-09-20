use crate::platform_support::cx_platform_extensions::CxPlatformExtensions;
use gpui::App;

impl CxPlatformExtensions for App {
    fn beep(&self) {
        // noop
    }
}
