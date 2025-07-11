use crate::hsv::Hsva;
use crate::styling::contemporary::{
    make_contemporary_base_theme, ContemporaryDark, ContemporaryLight,
};
use crate::styling::rgb::rgba_tuple;
use gpui::{Global, Pixels, Rgba};
use std::rc::Rc;

#[derive(PartialEq, Copy, Clone)]
pub enum ThemeType {
    System,
    Light,
    Dark,
}

#[derive(Clone)]
pub struct Theme {
    pub theme_type: ThemeType,

    pub background: Rgba,
    pub foreground: Rgba,

    pub system_font_family: String,
    pub system_font_size: Pixels,
    pub heading_font_size: Pixels,

    pub button_background: Rgba,
    pub button_foreground: Rgba,

    pub layer_background: Rgba,

    pub border_color: Rgba,
    pub border_radius: Pixels,

    pub focus_decoration: Rgba,
    pub destructive_accent_color: Rgba,
}

pub trait VariableColor {
    fn disable_when(self, condition: bool) -> Rgba;
    fn disabled(self) -> Rgba;
    fn hover(self) -> Self;
    fn active(self) -> Self;
}

impl VariableColor for Rgba {
    fn disable_when(self, condition: bool) -> Self {
        if condition { self.disabled() } else { self }
    }

    fn disabled(self) -> Self {
        if self.a == 0.0 {
            return self;
        }

        let hsv: Hsva = self.into();

        if hsv.v < 0.5 {
            Hsva {
                h: hsv.h,
                s: hsv.s / 2.,
                v: (1. - hsv.v) / 2.,
                a: hsv.a,
            }
            .into()
        } else {
            Hsva {
                h: hsv.h,
                s: hsv.s / 2.0,
                v: hsv.v / 2.0,
                a: hsv.a,
            }
            .into()
        }
    }

    fn hover(self) -> Self {
        if self.a == 0. {
            rgba_tuple(255, 255, 255, 75. / 255.)
        } else {
            let hsv: Hsva = self.into();
            hsv.lighter(1.5).into()
        }
    }

    fn active(self) -> Self {
        if self.a == 0. {
            rgba_tuple(0, 0, 0, 75. / 255.)
        } else {
            let hsv: Hsva = self.into();
            hsv.darker(2.).into()
        }
    }
}

pub fn variable_transparent() -> Rgba {
    rgba_tuple(0, 0, 0, 0.)
}

impl Theme {
    pub fn set_theme(&mut self, other: Theme) {
        self.theme_type = other.theme_type;
        self.background = other.background;
        self.foreground = other.foreground;
        self.system_font_family = other.system_font_family;
        self.system_font_size = other.system_font_size;
        self.heading_font_size = other.heading_font_size;
        self.button_background = other.button_background;
        self.button_foreground = other.button_foreground;
        self.layer_background = other.layer_background;
        self.border_color = other.border_color;
        self.border_radius = other.border_radius;
        self.focus_decoration = other.focus_decoration;
        self.destructive_accent_color = other.destructive_accent_color;
    }

    pub fn disabled(self) -> Self {
        Theme {
            background: self.background.disabled(),
            foreground: self.foreground.disabled(),
            button_background: self.button_background.disabled(),
            button_foreground: self.button_foreground.disabled(),
            border_color: self.border_color.disabled(),
            destructive_accent_color: self.destructive_accent_color.disabled(),
            ..self
        }
    }

    pub fn hover(self) -> Self {
        Theme {
            background: self.background.hover(),
            foreground: self.foreground.hover(),
            button_background: self.button_background.hover(),
            button_foreground: self.button_foreground.hover(),
            border_color: self.border_color.hover(),
            destructive_accent_color: self.destructive_accent_color.hover(),
            ..self
        }
    }

    pub fn active(self) -> Self {
        Theme {
            background: self.background.active(),
            foreground: self.foreground.active(),
            button_background: self.button_background.active(),
            button_foreground: self.button_foreground.active(),
            border_color: self.border_color.active(),
            destructive_accent_color: self.destructive_accent_color.active(),
            ..self
        }
    }

    pub fn disable_when(self, condition: bool) -> Self {
        if condition { self.disabled() } else { self }
    }

    #[allow(unreachable_code)]
    pub fn default_of_type(theme_type: ThemeType) -> Theme {
        #[cfg(target_os = "macos")]
        {
            return crate::platform_support::macos::theme::create_macos_theme(theme_type);
        }

        #[cfg(target_os = "windows")]
        {
            return crate::platform_support::windows::theme::create_windows_theme(theme_type);
        }

        #[cfg(target_os = "linux")]
        {
            return crate::platform_support::linux::theme::create_linux_theme(theme_type);
        }

        Self {
            ..match theme_type {
                ThemeType::System => make_contemporary_base_theme::<ContemporaryDark>(),
                ThemeType::Light => make_contemporary_base_theme::<ContemporaryLight>(),
                ThemeType::Dark => make_contemporary_base_theme::<ContemporaryDark>(),
            }
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::default_of_type(ThemeType::System)
    }
}

impl Global for Theme {}
