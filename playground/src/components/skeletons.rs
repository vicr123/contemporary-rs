use cntp_i18n::tr;
use contemporary::components::constrainer::constrainer;
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::skeleton::{skeleton, skeleton_row};
use contemporary::components::subtitle::subtitle;
use contemporary::styling::theme::Theme;
use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
};

pub struct Skeletons;

impl Skeletons {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Skeletons {})
    }
}

impl Render for Skeletons {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .bg(theme.background)
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(
                grandstand("skeletons-grandstand")
                    .text(tr!("SKELETONS_TITLE", "Skeletons"))
                    .pt(px(36.)),
            )
            .child(
                constrainer("skeletons-constrainer")
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
                            .child(subtitle(tr!("SKELETONS_TITLE")))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .items_start()
                                    .gap(px(8.))
                                    .child(skeleton("skeleton-1"))
                                    .child(skeleton("skeleton-big").text_size(px(24.)))
                                    .child(
                                        skeleton("skeleton-2")
                                            .child("This is a really long continuous skeleton"),
                                    )
                                    .child(
                                        skeleton_row("skeleton-3")
                                            .chunk("This")
                                            .chunk("is")
                                            .chunk("a")
                                            .chunk("skeleton")
                                            .chunk("row"),
                                    )
                                    .child(skeleton("skeleton-4").size(px(100.))),
                            ),
                    ),
            )
    }
}
