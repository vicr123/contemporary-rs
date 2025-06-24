use contemporary::components::button::button;
use contemporary::components::constrainer::constrainer;
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::subtitle::subtitle;
use contemporary_i18n::tr;
use gpui::{App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};

#[derive(IntoElement)]
pub struct Buttons;

pub fn buttons() -> Buttons {
    Buttons {}
}

impl RenderOnce for Buttons {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
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
                            .child(subtitle(tr!("BUTTONS_NORMAL_TITLE", "Buttons")))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(8.))
                                    .child(
                                        div().flex_grow().child(button("button-1").child(tr!(
                                            "BUTTONS_DEFAULT_BUTTON",
                                            "Default Button"
                                        ))),
                                    )
                                    .child(div().flex_grow().child(
                                        button("button-2").disabled().child(tr!(
                                            "BUTTONS_DISABLED_BUTTON",
                                            "Disabled Button"
                                        )),
                                    ))
                                    .child(div().flex_grow().child(button("button-3").child(tr!(
                                        "BUTTONS_CHECKABLE_BUTTON",
                                        "Checkable Button"
                                    )))),
                            ),
                    )
                    .child(
                        layer("flat-buttons")
                            .flex()
                            .flex_col()
                            .p(px(8.))
                            .w_full()
                            .child(subtitle(tr!("BUTTONS_FLAT_TITLE", "Flat Buttons")))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(8.))
                                    .child(
                                        button("button-1")
                                            .flat()
                                            .flex_grow()
                                            .child(tr!("BUTTONS_FLAT_BUTTON", "Flat Button")),
                                    )
                                    .child(button("button-2").flat().disabled().flex_grow().child(
                                        tr!("BUTTONS_FLAT_DISABLED_BUTTON", "Flat Disabled Button"),
                                    ))
                                    .child(button("button-3").flat().flex_grow().child(tr!(
                                        "BUTTONS_FLAT_CHECKABLE_BUTTON",
                                        "Flat Checkable Button"
                                    ))),
                            ),
                    ),
            )
    }
}
