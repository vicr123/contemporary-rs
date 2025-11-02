use cntp_i18n::tr;
use contemporary::application::Details;
use contemporary::components::constrainer::constrainer;
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::subtitle::subtitle;
use contemporary::styling::theme::ThemeStorage;
use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
};

pub struct Directories {}

impl Directories {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self {})
    }
}

impl Render for Directories {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let details = cx.global::<Details>();

        let directories = details.standard_dirs().unwrap();

        div()
            .bg(theme.background)
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(
                grandstand("directories-grandstand")
                    .text(tr!("DIRECTORIES_TITLE", "Directories"))
                    .pt(px(36.)),
            )
            .child(
                constrainer("i18n")
                    .flex()
                    .flex_col()
                    .w_full()
                    .p(px(8.))
                    .gap(px(8.))
                    .child(
                        layer()
                            .flex()
                            .flex_col()
                            .p(px(8.))
                            .w_full()
                            .child(subtitle(tr!("DIRECTORIES_TITLE")))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(tr!(
                                        "DIRECTORIES_DESCRIPTION",
                                        "On this platform, the standard directories are as follows."
                                    ))
                                    .child(tr!(
                                        "DIRECTORIES_CACHE_DIR",
                                        "Cache directory: {{cache_dir}}",
                                        cache_dir = directories.cache_dir().to_str().unwrap()
                                    ))
                                    .child(tr!(
                                        "DIRECTORIES_CONFIG_DIR",
                                        "Config directory: {{config_dir}}",
                                        config_dir = directories.config_dir().to_str().unwrap()
                                    ))
                                    .child(tr!(
                                        "DIRECTORIES_DATA_DIR",
                                        "Data directory: {{data_dir}}",
                                        data_dir = directories.data_dir().to_str().unwrap()
                                    ))
                                    .child(tr!(
                                        "DIRECTORIES_PREFERENCES_DIR",
                                        "Preferences directory: {{preferences_dir}}",
                                        preferences_dir =
                                            directories.preference_dir().to_str().unwrap()
                                    )),
                            ),
                    ),
            )
    }
}
