use cntp_i18n::tr;
use contemporary::components::button::button;
use contemporary::components::constrainer::constrainer;
use contemporary::components::grandstand::grandstand;
use contemporary::components::icon_text::icon_text;
use contemporary::components::layer::layer;
use contemporary::components::popover::popover;
use contemporary::components::subtitle::subtitle;
use contemporary::styling::theme::Theme;
use gpui::prelude::FluentBuilder;
use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
};

pub struct Popovers {
    informational_popover_open: bool,
    informational_popover_side: u8,
}

impl Popovers {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Popovers {
            informational_popover_open: false,
            informational_popover_side: 0,
        })
    }
}

impl Render for Popovers {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .bg(theme.background)
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(
                grandstand("popovers-grandstand")
                    .text(tr!("POPOVERS_TITLE", "Popovers"))
                    .pt(px(36.)),
            )
            .child(
                constrainer("popovers")
                    .flex()
                    .flex_col()
                    .w_full()
                    .p(px(8.))
                    .gap(px(8.))
                    .child(
                        layer("normal-popovers")
                            .flex()
                            .flex_col()
                            .p(px(8.))
                            .w_full()
                            .child(subtitle(tr!("POPOVERS_TITLE")))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(tr!(
                                        "POPOVERS_DESCRIPTION",
                                        "Click on a button to open a popover"
                                    ))
                                    .child(
                                        button("bottom-popover")
                                            .child(tr!("POPOVER_BOTTOM", "Bottom Popover"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.informational_popover_open = true;
                                                this.informational_popover_side = 1;
                                                cx.notify()
                                            })),
                                    )
                                    .child(
                                        button("trailing-popover")
                                            .child(tr!("POPOVER_TRAILING", "Trailing Popover"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.informational_popover_open = true;
                                                this.informational_popover_side = 3;
                                                cx.notify()
                                            })),
                                    )
                                    .child(
                                        button("top-popover")
                                            .child(tr!("POPOVER_TOP", "Top Popover"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.informational_popover_open = true;
                                                this.informational_popover_side = 0;
                                                cx.notify()
                                            })),
                                    )
                                    .child(
                                        button("leading-popover")
                                            .child(tr!("POPOVER_LEADING", "Leading Popover"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.informational_popover_open = true;
                                                this.informational_popover_side = 2;
                                                cx.notify()
                                            })),
                                    ),
                            ),
                    ),
            )
            .child(
                popover("informational-popover")
                    .visible(self.informational_popover_open)
                    .size_neg(100.)
                    .when(self.informational_popover_side == 0, |popover| {
                        popover.anchor_top()
                    })
                    .when(self.informational_popover_side == 1, |popover| {
                        popover.anchor_bottom()
                    })
                    .when(self.informational_popover_side == 2, |popover| {
                        popover.anchor_leading()
                    })
                    .when(self.informational_popover_side == 3, |popover| {
                        popover.anchor_trailing()
                    })
                    .content(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(9.))
                            .child(
                                grandstand("informational-popover-grandstand")
                                    .text(tr!("POPOVER_INFORMATIONAL", "Informational Popover"))
                                    .on_back_click(cx.listener(|this, _, _, cx| {
                                        this.informational_popover_open = false;
                                        cx.notify()
                                    })),
                            )
                            .child(
                                constrainer("informational-dialog-box-constrainer").child(
                                    layer("normal-popovers")
                                        .flex()
                                        .flex_col()
                                        .p(px(8.))
                                        .w_full()
                                        .child(subtitle(tr!("POPOVER_INFORMATIONAL")))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.))
                                                .child(tr!(
                                                    "INFORMATIONAL_POPOVER_DESCRIPTION",
                                                    "This is an informational popover."
                                                ))
                                                .child(
                                                    button("informational-dialog-box")
                                                        .child(icon_text(
                                                            "dialog-ok".into(),
                                                            tr!("CLOSE", "Close").into(),
                                                        ))
                                                        .on_click(cx.listener(|this, _, _, cx| {
                                                            this.informational_popover_open = false;
                                                            cx.notify()
                                                        })),
                                                ),
                                        ),
                                ),
                            ),
                    ),
            )
    }
}
