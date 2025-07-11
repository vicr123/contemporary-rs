use crate::styling::contemporary::{
    make_contemporary_base_theme, ContemporaryDark, ContemporaryLight,
};
use crate::styling::theme::Theme;
use ashpd::desktop::settings::Settings;
use gpui::px;

pub fn create_kde_theme(is_dark_mode: bool) -> Theme {
    let mut base_theme = if is_dark_mode {
        make_contemporary_base_theme::<ContemporaryDark>()
    } else {
        make_contemporary_base_theme::<ContemporaryLight>()
    };

    // KDE's default font is Noto Sans, 10pt
    base_theme.system_font_family = "Noto Sans".to_string();
    base_theme.system_font_size = px(10. * 4. / 3.);

    let portal_settings = smol::block_on(Settings::new());
    if let Ok(portal_settings) = portal_settings {
        if let Ok(font_settings) =
            smol::block_on(portal_settings.read::<String>("org.kde.kdeglobals.General", "font"))
        {
            // This gives us a QFont description, normally ingested by QFont::fromString
            // https://doc.qt.io/qt-6/qfont.html#fromString
            let font_parts: Vec<&str> = font_settings.split(',').collect();
            if let Some(font_family) = font_parts.first() {
                base_theme.system_font_family = font_family.to_string();
            }
            if let Some(font_size_pt_str) = font_parts.get(1) {
                if let Ok(font_size) = font_size_pt_str.parse::<f32>() {
                    base_theme.system_font_size = px(font_size * 4. / 3.);
                }
            }
        }
    }

    base_theme
}
