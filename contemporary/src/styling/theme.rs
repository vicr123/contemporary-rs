use crate::styling::contemporary::{make_contemporary_base_theme, ContemporaryDark};
use gpui::{Global, Pixels, Rgba};

#[cfg(target_os = "macos")]
use crate::styling::macos::create_macos_theme;
use crate::styling::rgb::{rgb_tuple, rgba_tuple};

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

fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    // Calculate hue
    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h } / 360.0; // Normalize to [0, 1]

    // Calculate saturation
    let s = if max == 0.0 { 0.0 } else { delta / max };

    // Value is the maximum component
    (h, s, max)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Rgba {
    let h = h * 360.0; // Scale back to [0, 360]
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h as i32 {
        h if h < 60 => (c, x, 0.0),
        h if h < 120 => (x, c, 0.0),
        h if h < 180 => (0.0, c, x),
        h if h < 240 => (0.0, x, c),
        h if h < 300 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    rgba_tuple(
        ((r + m) * 255.) as u8,
        ((g + m) * 255.) as u8,
        ((b + m) * 255.) as u8,
        1.0,
    )
}

impl VariableColor for Rgba {
    fn disable_when(self, condition: bool) -> Self {
        if condition { self.disabled() } else { self }
    }

    fn disabled(self) -> Self {
        if self.a == 0.0 {
            return self;
        }

        let (h, s, v) = rgb_to_hsv(self.r, self.g, self.b);

        if v < 0.5 {
            let new_v = (1.0 - v) / 2.0;
            let new_s = s / 2.0;
            hsv_to_rgb(h, new_s, new_v)
        } else {
            let new_v = v / 2.0;
            let new_s = s / 2.0;
            hsv_to_rgb(h, new_s, new_v)
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
