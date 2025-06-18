use crate::styling::rgb::{rgb_tuple, rgba_tuple};
use crate::styling::theme::Theme;
use gpui::{px, Pixels, Rgba};

pub trait ContemporaryTheme {
    const BACKGROUND: Rgba;
    const FOREGROUND: Rgba;
    const LAYER: Rgba;
    const SYSTEM_FONT_FAMILY: &'static str = "Sofia Sans";
    const SYSTEM_FONT_SIZE: Pixels = px(14.0);
    const HEADING_FONT_SIZE: Pixels = px(16.0);
    const BORDER_RADIUS: Pixels;
}

pub struct ContemporaryDark;

impl ContemporaryTheme for ContemporaryDark {
    const BACKGROUND: Rgba = rgb_tuple(40, 40, 40);
    const FOREGROUND: Rgba = rgb_tuple(255, 255, 255);
    const LAYER: Rgba = rgba_tuple(255, 255, 255, 0.06);
    const BORDER_RADIUS: Pixels = px(4.0);
}

pub struct ContemporaryLight;

impl ContemporaryTheme for ContemporaryLight {
    const BACKGROUND: Rgba = rgb_tuple(255, 255, 255);
    const FOREGROUND: Rgba = rgb_tuple(0, 0, 0);
    const LAYER: Rgba = rgba_tuple(0, 0, 0, 0.1);
    const BORDER_RADIUS: Pixels = px(4.0);
}

pub fn make_contemporary_base_theme<T>() -> Theme
where
    T: ContemporaryTheme,
{
    Theme {
        background: T::BACKGROUND,
        foreground: T::FOREGROUND,
        system_font_family: T::SYSTEM_FONT_FAMILY,
        system_font_size: T::SYSTEM_FONT_SIZE,
        heading_font_size: T::HEADING_FONT_SIZE,
        button_background: rgb_tuple(0, 50, 150),
        button_foreground: T::FOREGROUND,
        button_hover_background: rgb_tuple(0, 75, 225),
        button_active_background: rgb_tuple(0, 33, 100),
        border_radius: T::BORDER_RADIUS,
        layer_background: T::LAYER,
    }
}
