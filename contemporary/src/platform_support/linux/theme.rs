use crate::styling::contemporary::{
    make_contemporary_base_theme, ContemporaryDark, ContemporaryLight,
};
use crate::styling::theme::{Theme, ThemeType};
use ashpd::desktop::settings::{Settings, APPEARANCE_NAMESPACE, COLOR_SCHEME_KEY};

pub fn create_linux_theme(theme_type: ThemeType) -> Theme {
    let is_dark_mode = match theme_type {
        ThemeType::System => {
            let portal_settings = smol::block_on(Settings::new());
            if let Ok(portal_settings) = portal_settings {
                let color_scheme = smol::block_on(
                    portal_settings.read::<u32>(APPEARANCE_NAMESPACE, COLOR_SCHEME_KEY),
                );
                if let Ok(color_scheme) = color_scheme {
                    if color_scheme == 2 {
                        // Light theme requested
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            } else {
                true
            }
        }
        ThemeType::Light => false,
        ThemeType::Dark => true,
    };

    Theme {
        theme_type,
        ..if is_dark_mode {
            make_contemporary_base_theme::<ContemporaryDark>()
        } else {
            make_contemporary_base_theme::<ContemporaryLight>()
        }
    }
}
