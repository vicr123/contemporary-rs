use crate::styling::contemporary::{
    make_contemporary_base_theme, ContemporaryDark, ContemporaryLight,
};
use crate::styling::theme::Theme;

pub fn create_gnome_theme(is_dark_mode: bool) -> Theme {
    // TODO
    if is_dark_mode {
        make_contemporary_base_theme::<ContemporaryDark>()
    } else {
        make_contemporary_base_theme::<ContemporaryLight>()
    }
}
