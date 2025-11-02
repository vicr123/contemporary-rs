use crate::styling::contemporary::{
    ContemporaryDark, ContemporaryLight, make_contemporary_base_theme,
};
use crate::styling::theme::{Theme, ThemeType};
use gpui::px;
use objc2_foundation::{NSUserDefaults, ns_string};

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

    let base_theme = if is_dark_mode {
        make_contemporary_base_theme::<ContemporaryDark>()
    } else {
        make_contemporary_base_theme::<ContemporaryLight>()
    };

    Theme {
        theme_type,
        system_font_family: ".AppleSystemUIFont".to_string(),
        system_font_size: px(13.),
        monospaced_font_family: "SF Mono".to_string(),

        ..base_theme
    }
}
