use crate::styling::contemporary::{
    make_contemporary_base_theme, ContemporaryDark, ContemporaryLight,
};
use crate::styling::theme::{Theme, ThemeType};
use gpui::px;
use objc2_foundation::{ns_string, NSUserDefaults};

pub fn create_macos_theme(theme_type: ThemeType) -> Theme {
    let apple_interface_style = unsafe {
        let user_defaults = NSUserDefaults::standardUserDefaults();
        user_defaults
            .stringForKey(ns_string!("AppleInterfaceStyle"))
            .map(|ns_string| ns_string.to_string())
            .unwrap_or("Light".into())
    };

    let is_dark_mode = match theme_type {
        ThemeType::System => apple_interface_style == "Dark",
        ThemeType::Light => false,
        ThemeType::Dark => true,
    };

    Theme {
        theme_type,
        system_font_family: ".AppleSystemUIFont",
        system_font_size: px(13.),

        ..{
            if is_dark_mode {
                make_contemporary_base_theme::<ContemporaryDark>()
            } else {
                make_contemporary_base_theme::<ContemporaryLight>()
            }
        }
    }
}
