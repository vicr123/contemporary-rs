use crate::platform_support::linux::desktop_environment::DesktopEnvironment;
use crate::platform_support::linux::gnome::theme::create_gnome_theme;
use crate::platform_support::linux::kde::theme::create_kde_theme;
use crate::styling::contemporary::{
    ContemporaryDark, ContemporaryLight, make_contemporary_base_theme,
};
use crate::styling::theme::{Theme, ThemeType};
use ashpd::desktop::settings::{APPEARANCE_NAMESPACE, COLOR_SCHEME_KEY, Settings};

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
        ..match DesktopEnvironment::current() {
            Some(DesktopEnvironment::KDE) => create_kde_theme(is_dark_mode),
            Some(DesktopEnvironment::GNOME) => create_gnome_theme(is_dark_mode),
            None => {
                if is_dark_mode {
                    make_contemporary_base_theme::<ContemporaryDark>()
                } else {
                    make_contemporary_base_theme::<ContemporaryLight>()
                }
            }
        }
    }
}
