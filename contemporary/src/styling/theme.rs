use crate::hsv::Hsva;
use crate::styling::contemporary::{make_contemporary_base_theme, ContemporaryDark};
#[cfg(target_os = "macos")]
use crate::styling::macos::create_macos_theme;
use crate::styling::rgb::rgb_tuple;
use gpui::{Global, Pixels, Rgba};

#[derive(Copy, Clone)]
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

    pub border_color: Rgba,
    pub border_radius: Pixels,

    pub focus_decoration: Rgba,
    pub destructive_accent_color: Rgba,
}

pub trait VariableColor {
    fn disable_when(self, condition: bool) -> Rgba;
    fn disabled(self) -> Rgba;
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
            let new_v = hsv.v / 2.0;
            let new_s = hsv.s / 2.0;
            Hsva {
                h: hsv.h,
                s: hsv.s / 2.0,
                v: hsv.v / 2.0,
                a: hsv.a,
            }
            .into()
        }
    }
}

impl Theme {
    pub fn disabled(self) -> Self {
        Theme {
            background: self.background.disabled(),
            foreground: self.foreground.disabled(),
            button_background: self.button_background.disabled(),
            button_foreground: self.button_foreground.disabled(),
            button_hover_background: self.button_hover_background.disabled(),
            button_active_background: self.button_active_background.disabled(),
            border_color: self.border_color.disabled(),
            destructive_accent_color: self.destructive_accent_color.disabled(),
            ..self
        }
    }

    pub fn disable_when(self, condition: bool) -> Self {
        if condition { self.disabled() } else { self }
    }
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
