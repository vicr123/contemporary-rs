use gpui::px;
use windows::UI::ViewManagement::{UIColorType, UISettings};
use crate::styling::contemporary::{make_contemporary_base_theme, ContemporaryDark, ContemporaryLight};
use crate::styling::theme::{Theme, ThemeType};

pub fn create_windows_theme(theme_type: ThemeType) -> Theme {
    let is_dark_mode = match theme_type {
        ThemeType::System => {
            let ui_settings = UISettings::new().unwrap();
            let foreground = ui_settings.GetColorValue(UIColorType::Foreground).unwrap();

            // https://learn.microsoft.com/en-us/windows/apps/desktop/modernize/ui/apply-windows-themes
            
            // "modernize"
            ((5 * foreground.G as u32) + (2 * foreground.R as u32) + foreground.B as u32) > (8 * 128)
        },
        ThemeType::Light => false,
        ThemeType::Dark => true,
    };

    Theme {
        theme_type,
        system_font_family: "Segoe UI",
        system_font_size: px(13.),

        ..{
            if is_dark_mode {
                make_contemporary_base_theme::<ContemporaryDark>()
            } else {
                make_contemporary_base_theme::<ContemporaryLight>()
            }
        }
    }
}