use cntp_i18n::tr;
use contemporary::components::constrainer::constrainer;
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::progress_bar::progress_bar;
use contemporary::components::subtitle::subtitle;
use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
};

pub struct ProgressBars;

impl ProgressBars {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| ProgressBars {})
    }
}

impl Render for ProgressBars {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(
                grandstand("progress-bars-grandstand")
                    .text(tr!("PROGRESS_BARS_TITLE", "Progress Bars"))
                    .pt(px(36.)),
            )
            .child(
                constrainer("progress-bars")
                    .flex()
                    .flex_col()
                    .w_full()
                    .p(px(8.))
                    .gap(px(8.))
                    .child(
                        layer("normal-progress-bars")
                            .flex()
                            .flex_col()
                            .p(px(8.))
                            .w_full()
                            .child(subtitle(tr!("PROGRESS_BARS_TITLE")))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(progress_bar().value(0.4))
                                    .child(
                                        progress_bar().indeterminate("indeterminate-progress-bar"),
                                    ),
                            ),
                    ),
            )
    }
}
