use cntp_i18n::tr;
use contemporary::components::admonition::{AdmonitionSeverity, admonition};
use contemporary::components::constrainer::constrainer;
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::subtitle::subtitle;
use contemporary::styling::theme::Theme;
use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
};

pub struct Admonitions;

impl Admonitions {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Admonitions {})
    }
}

impl Render for Admonitions {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .bg(theme.background)
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(
                grandstand("admonitions-grandstand")
                    .text(tr!("ADMONITIONS_TITLE", "Admonitions"))
                    .pt(px(36.)),
            )
            .child(
                constrainer("admonitions")
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
                            .child(subtitle(tr!("ADMONITIONS_TITLE")))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(
                                        admonition()
                                            .severity(AdmonitionSeverity::Info)
                                            .title(tr!(
                                                "ADMONITION_INFO_TITLE",
                                                "Informational Admonition"
                                            ))
                                            .child(tr!(
                                                "ADMONITION_INFO_CONTENT",
                                                "This is a string with information."
                                            )),
                                    )
                                    .child(
                                        admonition()
                                            .severity(AdmonitionSeverity::Warning)
                                            .title(tr!(
                                                "ADMONITION_WARNING_TITLE",
                                                "Warning Admonition"
                                            ))
                                            .child(tr!(
                                                "ADMONITION_WARNING_CONTENT",
                                                "This is a warning."
                                            )),
                                    )
                                    .child(
                                        admonition()
                                            .severity(AdmonitionSeverity::Error)
                                            .title(tr!(
                                                "ADMONITION_ERROR_TITLE",
                                                "Error Admonition"
                                            ))
                                            .child(tr!(
                                                "ADMONITION_ERROR_CONTENT",
                                                "This is an error."
                                            )),
                                    ),
                            ),
                    ),
            )
    }
}
