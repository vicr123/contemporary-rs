use crate::styling::contemporary::{ContemporaryDark, make_contemporary_base_theme};
use gpui::{Global, Pixels, Rgba};

#[cfg(target_os = "macos")]
use crate::styling::macos::create_macos_theme;
use crate::styling::rgb::rgb_tuple;

pub struct Theme {
    pub background: Rgba,
    pub foreground: Rgba,

    pub system_font_family: &'static str,
    pub system_font_size: Pixels,
    pub heading_font_size: Pixels,

    pub button_background: Rgba,
    pub button_foreground: Rgba,
    pub button_hover_background: Rgba,
    pub button_active_background: Rgba,

    pub layer_background: Rgba,

    pub border_radius: Pixels,
}

impl Default for Theme {
    #[allow(unreachable_code)]
    fn default() -> Self {
        #[cfg(target_os = "macos")]
        {
            return create_macos_theme();
        }

        Self {
            button_background: rgb_tuple(0, 50, 150),
            button_hover_background: rgb_tuple(0, 75, 225),
            button_active_background: rgb_tuple(0, 33, 100),
            ..make_contemporary_base_theme::<ContemporaryDark>()
        }
    }
}

impl Global for Theme {}
