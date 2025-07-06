use crate::styling::theme::Theme;
use crate::styling::theme::ThemeType::System;
use gpui::{App, AsyncApp, Global};
use windows::Foundation::TypedEventHandler;
use windows::UI::ViewManagement::UISettings;
use windows::core::IInspectable;

struct UISettingsGlobalableWrapperThing {
    globalable_item: UISettings,
}

impl Global for UISettingsGlobalableWrapperThing {}

pub fn setup_windows_color_values_changed_listener(cx: &mut App) {
    let (tx, rx) = async_channel::bounded(3);

    let ui_settings = UISettings::new().unwrap();
    _ = ui_settings.ColorValuesChanged(&TypedEventHandler::<UISettings, IInspectable>::new(
        move |_, _| {
            _ = smol::block_on(tx.send(()));
            Ok(())
        },
    ));

    cx.set_global(UISettingsGlobalableWrapperThing {
        globalable_item: ui_settings,
    });

    cx.spawn(async move |cx: &mut AsyncApp| {
        loop {
            _ = rx.recv().await;

            _ = cx.update_global::<Theme, ()>(|theme, cx| {
                if theme.theme_type == System {
                    theme.set_theme(Theme::default());
                }
            });

            _ = cx.refresh();
        }
    })
    .detach()
}
