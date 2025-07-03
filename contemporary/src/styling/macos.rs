use crate::styling::contemporary::{
    make_contemporary_base_theme, ContemporaryDark, ContemporaryLight,
};
use crate::styling::rgb::rgb_tuple;
use crate::styling::theme::Theme;
use gpui::px;

pub fn create_macos_theme() -> Theme {
    let is_dark_mode = true;

    Theme {
        system_font_family: ".AppleSystemUIFont",
        system_font_size: px(13.),
        button_background: rgb_tuple(0, 50, 150),
        button_hover_background: rgb_tuple(0, 75, 225),
        button_active_background: rgb_tuple(0, 33, 100),

        ..{
            if is_dark_mode {
                make_contemporary_base_theme::<ContemporaryDark>()
            } else {
                make_contemporary_base_theme::<ContemporaryLight>()
            }
        }
    }
}
