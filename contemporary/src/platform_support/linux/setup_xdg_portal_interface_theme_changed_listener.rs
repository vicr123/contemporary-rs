use crate::styling::theme::Theme;
use crate::styling::theme::ThemeType::System;
use ashpd::desktop::settings::Settings;
use gpui::{App, AsyncApp};
use smol::stream::StreamExt;

pub fn setup_xdg_portal_interface_theme_changed_listener(cx: &mut App) {
    cx.spawn(async move |cx: &mut AsyncApp| {
        let Ok(portal_settings) = Settings::new().await else {
            return;
        };

        let Ok(mut color_scheme_changed) = portal_settings.receive_color_scheme_changed().await
        else {
            return;
        };

        loop {
            _ = color_scheme_changed.next().await;

            _ = cx.update_global::<Theme, ()>(|theme, _cx| {
                if theme.theme_type == System {
                    theme.set_theme(Theme::default());
                }
            });

            _ = cx.refresh();
        }
    })
    .detach()
}
