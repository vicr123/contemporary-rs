use cntp_i18n::tr;
use contemporary::components::anchorer::WithAnchorer;
use contemporary::components::button::button;
use contemporary::components::constrainer::constrainer;
use contemporary::components::flyout::flyout;
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::subtitle::subtitle;
use contemporary::styling::theme::ThemeStorage;
use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
};

pub struct Flyouts {
    bottom_flyout_open: bool,
    right_flyout_open: bool,
}

impl Flyouts {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Flyouts {
            bottom_flyout_open: false,
            right_flyout_open: false,
        })
    }
}

impl Render for Flyouts {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        let bottom_flyout_open = self.bottom_flyout_open;
        let right_flyout_open = self.right_flyout_open;
        let flyout_bottom_close_function = cx.listener(|this, _, _, cx| {
            this.bottom_flyout_open = false;
            cx.notify()
        });
        let flyout_right_close_function = cx.listener(|this, _, _, cx| {
            this.right_flyout_open = false;
            cx.notify()
        });

        div()
            .bg(theme.background)
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(
                grandstand("flyouts-grandstand")
                    .text(tr!("FLYOUTS_TITLE", "Flyouts"))
                    .pt(px(36.)),
            )
            .child(
                constrainer("flyouts")
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
                            .child(subtitle(tr!("FLYOUTS_TITLE")))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(tr!(
                                        "FLYOUTS_DESCRIPTION",
                                        "Click on a button to open a flyout"
                                    ))
                                    .child(
                                        button("bottom-flyout")
                                            .child(tr!("FLYOUT_BOTTOM", "Bottom Flyout"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.bottom_flyout_open = true;
                                                cx.notify()
                                            }))
                                            .with_anchorer(move |david, anchorer, _, _| {
                                                david.child(
                                                    flyout(anchorer)
                                                        .visible(bottom_flyout_open)
                                                        .p(px(4.))
                                                        .child(tr!(
                                                            "FLYOUT_CONTENT",
                                                            "This is a flyout."
                                                        ))
                                                        .on_close(flyout_bottom_close_function),
                                                )
                                            }),
                                    )
                                    .child(
                                        button("right-flyout")
                                            .child(tr!("FLYOUT_RIGHT", "Right Flyout"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.right_flyout_open = true;
                                                cx.notify()
                                            }))
                                            .with_anchorer(move |david, anchorer, _, _| {
                                                david.child(
                                                    flyout(anchorer)
                                                        .visible(right_flyout_open)
                                                        .anchor_top_right()
                                                        .p(px(4.))
                                                        .child(tr!("FLYOUT_CONTENT",))
                                                        .on_close(flyout_right_close_function),
                                                )
                                            }),
                                    ),
                            ),
                    ),
            )
    }
}
