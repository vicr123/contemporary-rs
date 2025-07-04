use crate::styling::contemporary::{
    make_contemporary_base_theme, ContemporaryDark, ContemporaryLight,
};
use crate::styling::theme::{Theme, ThemeType};
use gpui::px;

pub fn create_macos_theme(theme_type: ThemeType) -> Theme {
    let is_dark_mode = match theme_type {
        ThemeType::System => true,
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
