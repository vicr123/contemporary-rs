use crate::styling::contemporary::{
    make_contemporary_base_theme, ContemporaryDark, ContemporaryLight,
};
use crate::styling::theme::{Theme, ThemeType};
use gpui::{px, Rgba};
use windows::UI::ViewManagement::{UIColorType, UISettings};

pub fn create_windows_theme(theme_type: ThemeType) -> Theme {
    let ui_settings = UISettings::new().unwrap();

    let is_dark_mode = match theme_type {
        ThemeType::System => {
            let foreground = ui_settings.GetColorValue(UIColorType::Foreground).unwrap();

            // https://learn.microsoft.com/en-us/windows/apps/desktop/modernize/ui/apply-windows-themes

            // "modernize"
            ((5 * foreground.G as u32) + (2 * foreground.R as u32) + foreground.B as u32)
                > (8 * 128)
        }
        ThemeType::Light => false,
        ThemeType::Dark => true,
    };

    Theme {
        theme_type,
        system_font_family: "Segoe UI",
        system_font_size: px(13.),
        button_background: {
            let color = if is_dark_mode {
                ui_settings.GetColorValue(UIColorType::Accent)
            } else {
                ui_settings.GetColorValue(UIColorType::AccentLight1)
            }
            .unwrap();

            Rgba {
                r: color.R as f32 / 255.0,
                g: color.G as f32 / 255.0,
                b: color.B as f32 / 255.0,
                a: color.A as f32 / 255.0,
            }
        },

        ..{
            if is_dark_mode {
                make_contemporary_base_theme::<ContemporaryDark>()
            } else {
                make_contemporary_base_theme::<ContemporaryLight>()
            }
        }
    }
}
