use gpui::Rgba;

pub const fn rgb(hex: u32) -> Rgba {
    let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
    let b = (hex & 0xFF) as f32 / 255.0;
    Rgba { r, g, b, a: 1.0 }
}

pub const fn rgb_tuple(r: u8, g: u8, b: u8) -> Rgba {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    Rgba { r, g, b, a: 1.0 }
}

pub const fn rgba_tuple(r: u8, g: u8, b: u8, a: f32) -> Rgba {
    Rgba {
        a,
        ..rgb_tuple(r, g, b)
    }
}
