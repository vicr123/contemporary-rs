use cntp_i18n::tr;
use contemporary::components::constrainer::constrainer;
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::skeleton::{SkeletonExt, skeleton, skeleton_row};
use contemporary::components::spinner::spinner;
use contemporary::components::subtitle::subtitle;
use contemporary::styling::theme::ThemeStorage;
use gpui::{
    App, AppContext, AsyncApp, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
    div, px,
};
use std::time::Duration;

pub struct Skeletons {
    skeleton_5_visible: bool,
}

impl Skeletons {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let skeletons = cx.new(|_| Skeletons {
            skeleton_5_visible: false,
        });

        let skeletons_clone = skeletons.clone();
        cx.spawn(async move |cx: &mut AsyncApp| {
            loop {
                cx.background_executor().timer(Duration::from_secs(1)).await;
                skeletons_clone
                    .update(cx, |skeletons, cx| {
                        skeletons.skeleton_5_visible = !skeletons.skeleton_5_visible;
                        cx.notify();
                    })
                    .unwrap()
            }
        })
        .detach();

        skeletons
    }
}

impl Render for Skeletons {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
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
                                    .child(skeleton("skeleton-4").size(px(100.)))
                                    .child(
                                        spinner().into_skeleton_when(
                                            self.skeleton_5_visible,
                                            "skeleton-5",
                                        ),
                                    ),
                            ),
                    ),
            )
    }
}
