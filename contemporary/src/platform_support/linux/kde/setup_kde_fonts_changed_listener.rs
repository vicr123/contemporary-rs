use crate::styling::theme::Theme;
use ashpd::desktop::settings::Settings;
use gpui::{App, AsyncApp};
use smol::stream::StreamExt;

pub fn setup_kde_fonts_changed_listener(cx: &mut App) {
    cx.spawn(async move |cx: &mut AsyncApp| {
        let Ok(portal_settings) = Settings::new().await else {
            return;
        };

        let Ok(mut setting_changed) = portal_settings.receive_setting_changed().await else {
            return;
        };

        loop {
            let setting = setting_changed.next().await;
            if let Some(setting) = setting
                && setting.namespace() == "org.kde.kdeglobals.General" && setting.key() == "font" {
                    _ = cx.update_global::<Theme, ()>(|theme, _cx| {
                        theme.set_theme(Theme::default_of_type(theme.theme_type));
                    });

                    _ = cx.refresh();
                }
        }
    })
    .detach()
}
