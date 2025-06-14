use crate::styling::contemporary::{
    ContemporaryDark, ContemporaryLight, ContemporaryTheme, make_contemporary_base_theme,
};
use crate::styling::rgb::rgb_tuple;
use crate::styling::theme::Theme;

pub fn create_macos_theme() -> Theme {
    let is_dark_mode = true;

    Theme {
        system_font_family: "San Francisco",
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
