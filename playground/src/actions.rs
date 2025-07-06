use contemporary::styling::theme::Theme;
use contemporary::styling::theme::ThemeType::{Dark, Light, System};
use gpui::{App, actions};

actions!(playground, [SystemTheme, LightTheme, DarkTheme]);

pub fn register_actions(cx: &mut App) {
    cx.on_action(system_theme);
    cx.on_action(light_theme);
    cx.on_action(dark_theme);
}

fn system_theme(_: &SystemTheme, cx: &mut App) {
    let theme = cx.global_mut::<Theme>();
    theme.set_theme(Theme::default_of_type(System));
    cx.refresh_windows();
}

fn light_theme(_: &LightTheme, cx: &mut App) {
    let theme = cx.global_mut::<Theme>();
    theme.set_theme(Theme::default_of_type(Light));
    cx.refresh_windows();
}

fn dark_theme(_: &DarkTheme, cx: &mut App) {
    let theme = cx.global_mut::<Theme>();
    theme.set_theme(Theme::default_of_type(Dark));
    cx.refresh_windows();
}
