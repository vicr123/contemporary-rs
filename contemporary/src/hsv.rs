use crate::lerp::Lerpable;
use gpui::{Hsla, Rgba};

#[derive(PartialEq, Clone, Copy, Default)]
pub struct Hsva {
    pub h: f32,
    pub s: f32,
    pub v: f32,
    pub a: f32,
}

impl Hsva {
    pub fn lighter(self, factor: f32) -> Self {
        Hsva {
            h: self.h,
            s: self.s,
            v: self.v * factor,
            a: self.a,
        }
    }

    pub fn darker(self, factor: f32) -> Self {
        Hsva {
            h: self.h,
            s: self.s,
            v: self.v / factor,
            a: self.a,
        }
    }
}

impl From<Rgba> for Hsva {
    fn from(value: Rgba) -> Self {
        let r = value.r;
        let g = value.g;
        let b = value.b;
        let a = value.a;

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
        Hsva { h, s, v: max, a }
    }
}

impl From<Hsva> for Rgba {
    fn from(value: Hsva) -> Self {
        let h = value.h;
        let s = value.s;
        let v = value.v;

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

        Rgba {
            r: r + m,
            g: g + m,
            b: b + m,
            a: value.a,
        }
    }
}

impl From<Hsva> for Hsla {
    fn from(value: Hsva) -> Self {
        let rgb_value: Rgba = value.into();
        rgb_value.into()
    }
}

impl From<Hsla> for Hsva {
    fn from(value: Hsla) -> Self {
        let rgb_value: Rgba = value.into();
        rgb_value.into()
    }
}

impl Lerpable for Hsva {
    fn lerp(&self, with: &Self, t: f32) -> Self {
        let rgb: Rgba = self.clone().into();
        let other_rgb: Rgba = with.clone().into();
        rgb.lerp(&other_rgb, t).into()
    }
}
