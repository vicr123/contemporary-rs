use contemporary::button::button;
use contemporary::constrainer::constrainer;
use contemporary::grandstand::grandstand;
use contemporary::layer::layer;
use contemporary::subtitle::subtitle;
use contemporary_i18n::tr;
use gpui::{div, px, App, IntoElement, ParentElement, RenderOnce, Styled, Window};

#[derive(IntoElement)]
pub struct Buttons;

pub fn buttons() -> Buttons {
    Buttons {}
}

impl RenderOnce for Buttons {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(
                grandstand("buttons-grandstand")
                    .text(tr!("BUTTONS_TITLE", "Buttons"))
                    .pt(px(36.)),
            )
            .child(
                constrainer("buttons")
                    .flex()
                    .flex_col()
                    .w_full()
                    .p(px(8.))
                    .gap(px(8.))
                    .child(
                        layer("normal-buttons")
                            .flex()
                            .flex_col()
                            .p(px(8.))
                            .w_full()
                            .child(subtitle(tr!("BUTTONS_NORMAL_TITLE", "Buttons").into()))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(8.))
                                    .child(
                                        div()
                                            .flex_grow()
                                            .child(button("button-1").child("Default Button")),
                                    )
                                    .child(div().flex_grow().child(
                                        button("button-2").disabled().child("Disabled Button"),
                                    ))
                                    .child(
                                        div()
                                            .flex_grow()
                                            .child(button("button-3").child("Checkable Button")),
                                    ),
                            ),
                    )
                    .child(
                        layer("flat-buttons")
                            .flex()
                            .flex_col()
                            .p(px(8.))
                            .w_full()
                            .child(subtitle(tr!("BUTTONS_FLAT_TITLE", "Flat Buttons").into()))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(8.))
                                    .child(
                                        button("button-1").flat().flex_grow().child("Flat Button"),
                                    )
                                    .child(
                                        button("button-2")
                                            .flat()
                                            .disabled()
                                            .flex_grow()
                                            .child("Flat Disabled Button"),
                                    )
                                    .child(
                                        button("button-3")
                                            .flat()
                                            .flex_grow()
                                            .child("Flat Checkable Button"),
                                    ),
                            ),
                    ),
            )
    }
}
