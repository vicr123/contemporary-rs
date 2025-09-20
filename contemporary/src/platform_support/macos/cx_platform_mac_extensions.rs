use crate::platform_support::cx_platform_extensions::CxPlatformExtensions;
use gpui::App;
use objc2_app_kit::NSBeep;

impl CxPlatformExtensions for App {
    fn beep(&self) {
        unsafe { NSBeep() }
    }
}
