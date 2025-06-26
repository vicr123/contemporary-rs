use contemporary::components::button::button;
use contemporary::components::constrainer::constrainer;
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::subtitle::subtitle;
use contemporary_i18n::{tr, trn};
use contemporary_icon_tool_macros::application_icon_asset_path;
use gpui::{
    div, img, px, App, AppContext, ClickEvent, Context, Entity, IntoElement,
    ParentElement, Render, Styled, Window,
};

pub struct Buttons {
    buttons_click_count: u8,
}

impl Buttons {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Buttons {
            buttons_click_count: 0,
        })
    }
}

impl Render for Buttons {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
            .child(img(application_icon_asset_path!()).h(px(32.)).w(px(32.)))
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
                                        )).on_click(cx.listener(|this, event: &ClickEvent, _, cx| {
                                            if event.down.modifiers.shift {
                                                this.buttons_click_count = 0
                                            } else {
                                                this.buttons_click_count += 1;
                                            }
                                            cx.notify()
                                        }))),
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
                            )
                            .child(trn!(
                                "BUTTONS_COUNT_TEXT",
                                "You have clicked the default button once (shift-click to reset)",
                                "You have clicked the default button {{count}} times (shift-click to reset)",
                                count = self.buttons_click_count as isize
                            )),
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
