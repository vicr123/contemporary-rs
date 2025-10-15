use crate::styling::rgb::{rgb_tuple, rgba_tuple};
use crate::styling::theme::{Theme, ThemeType};
use gpui::{Pixels, Rgba, px};

pub trait ContemporaryTheme {
    const TYPE: ThemeType;
    const BACKGROUND: Rgba;
    const FOREGROUND: Rgba;
    const SKELETON: Rgba;
    const LAYER: Rgba;
    const BUTTON_BACKGROUND: Rgba;
    const BORDER: Rgba;
    const SYSTEM_FONT_FAMILY: &'static str = "Commissioner Thin";
    const SYSTEM_FONT_SIZE: Pixels = px(14.0);
    const HEADING_FONT_SIZE: Pixels = px(16.0);
    const BORDER_RADIUS: Pixels;
    const FOCUS_DECORATION: Rgba;
    const DESTRUCTIVE_ACCENT: Rgba;
    const INFO_ACCENT: Rgba;
    const WARNING_ACCENT: Rgba;
    const ERROR_ACCENT: Rgba;
}

pub struct ContemporaryDark;

impl ContemporaryTheme for ContemporaryDark {
    const TYPE: ThemeType = ThemeType::Dark;
    const BACKGROUND: Rgba = rgb_tuple(40, 40, 40);
    const FOREGROUND: Rgba = rgb_tuple(255, 255, 255);
    const SKELETON: Rgba = rgb_tuple(230, 230, 230);
    const BUTTON_BACKGROUND: Rgba = rgb_tuple(0, 50, 150);
    const LAYER: Rgba = rgba_tuple(255, 255, 255, 5. / 255.);
    const BORDER: Rgba = rgba_tuple(255, 255, 255, 0.4);
    const BORDER_RADIUS: Pixels = px(4.0);
    const FOCUS_DECORATION: Rgba = rgb_tuple(20, 125, 200);
    const DESTRUCTIVE_ACCENT: Rgba = rgb_tuple(200, 0, 0);
    const INFO_ACCENT: Rgba = rgba_tuple(0, 200, 255, 0.1);
    const WARNING_ACCENT: Rgba = rgba_tuple(255, 200, 0, 0.1);
    const ERROR_ACCENT: Rgba = rgba_tuple(200, 0, 0, 0.1);
}

pub struct ContemporaryLight;

impl ContemporaryTheme for ContemporaryLight {
    const TYPE: ThemeType = ThemeType::Light;
    const BACKGROUND: Rgba = rgb_tuple(255, 255, 255);
    const FOREGROUND: Rgba = rgb_tuple(0, 0, 0);
    const SKELETON: Rgba = rgb_tuple(20, 20, 20);
    const BUTTON_BACKGROUND: Rgba = rgb_tuple(0, 150, 255);
    const LAYER: Rgba = rgba_tuple(0, 0, 0, 10. / 255.);
    const BORDER: Rgba = rgba_tuple(0, 0, 0, 0.4);
    const BORDER_RADIUS: Pixels = px(4.0);
    const FOCUS_DECORATION: Rgba = rgb_tuple(20, 125, 200);
    const DESTRUCTIVE_ACCENT: Rgba = rgb_tuple(255, 0, 0);
    const INFO_ACCENT: Rgba = rgba_tuple(0, 200, 255, 0.2);
    const WARNING_ACCENT: Rgba = rgba_tuple(255, 200, 0, 0.2);
    const ERROR_ACCENT: Rgba = rgba_tuple(200, 0, 0, 0.2);
}

pub fn make_contemporary_base_theme<T>() -> Theme
where
    T: ContemporaryTheme,
{
    Theme {
        theme_type: T::TYPE,
        background: T::BACKGROUND,
        foreground: T::FOREGROUND,
        skeleton: T::SKELETON,
        system_font_family: T::SYSTEM_FONT_FAMILY.to_string(),
        system_font_size: T::SYSTEM_FONT_SIZE,
        heading_font_size: T::HEADING_FONT_SIZE,
        button_background: T::BUTTON_BACKGROUND,
        button_foreground: T::FOREGROUND,
        border_color: T::BORDER,
        border_radius: T::BORDER_RADIUS,
        layer_background: T::LAYER,
        focus_decoration: T::FOCUS_DECORATION,
        destructive_accent_color: T::DESTRUCTIVE_ACCENT,
        info_accent_color: T::INFO_ACCENT,
        warning_accent_color: T::WARNING_ACCENT,
        error_accent_color: T::ERROR_ACCENT,
    }
}
