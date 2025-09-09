use cntp_i18n::{tr, trn};
use contemporary::components::button::button;
use contemporary::components::constrainer::constrainer;
use contemporary::components::context_menu::{ContextMenuExt, ContextMenuItem};
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::subtitle::subtitle;
use contemporary::styling::theme::Theme;
use gpui::{
    App, AppContext, ClickEvent, Context, Entity, IntoElement, ParentElement, Render, Styled,
    Window, div, px,
};
use tracing::info;

pub struct Buttons {
    buttons_click_count: u8,
    checkable_button_checked: bool,
    flat_checkable_button_checked: bool,
}

impl Buttons {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Buttons {
            buttons_click_count: 0,
            checkable_button_checked: true,
            flat_checkable_button_checked: true,
        })
    }
}

impl Render for Buttons {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .bg(theme.background)
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
                        layer()
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
                                            info!("Default button was clicked");
                                            if match event {
                                                ClickEvent::Mouse(mouse_event) => mouse_event.down.modifiers.shift,
                                                ClickEvent::Keyboard(_) => false
                                            } {
                                                this.buttons_click_count = 0
                                            } else {
                                                this.buttons_click_count += 1;
                                            }
                                            cx.notify()
                                        })))
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
                                    )).checked_when(self.checkable_button_checked).on_click(cx.listener(|this, _, _, cx| {
                                        this.checkable_button_checked = !this.checkable_button_checked;
                                        cx.notify()
                                    })))),
                            )
                            .child(div().child(trn!(
                                "BUTTONS_COUNT_TEXT",
                                "You have clicked the default button once (shift-click to reset)",
                                "You have clicked the default button {{count}} times (shift-click to reset)",
                                count = self.buttons_click_count as isize
                            )).with_context_menu(vec![
                                ContextMenuItem::separator().label(tr!("BUTTONS_COUNT_CONTEXT_MENU_TITLE", "For the counter")).build(),
                                ContextMenuItem::menu_item().label(tr!("COUNT_ADD_ONE", "Add 1")).remain_open().on_triggered(cx.listener(|this, _, _, cx| {
                                    this.buttons_click_count += 1;
                                    cx.notify()
                                })).build(),
                                ContextMenuItem::menu_item().label(tr!("COUNT_ADD_TEN", "Add 10")).remain_open().on_triggered(cx.listener(|this, _, _, cx| {
                                    this.buttons_click_count += 10;
                                    cx.notify()
                                })).build(),
                                ContextMenuItem::menu_item().label(tr!("COUNT_ADD_ONE_HUNDRED", "Add 100")).remain_open().on_triggered(cx.listener(|this, _, _, cx| {
                                    this.buttons_click_count += 10;
                                    cx.notify()
                                })).build(),
                                ContextMenuItem::separator().build(),
                                ContextMenuItem::menu_item().label(tr!("COUNT_RESET", "Reset")).icon("view-refresh").on_triggered(cx.listener(|this, _, _, cx| {
                                    this.buttons_click_count = 0;
                                    cx.notify()
                                })).build()
                            ])),
                    )
                    .child(
                        layer()
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
                                    )).checked_when(self.flat_checkable_button_checked).on_click(cx.listener(|this, _, _, cx| {
                                        this.flat_checkable_button_checked = !this.flat_checkable_button_checked;
                                        cx.notify()
                                    }))),
                            ),
                    ),
            )
    }
}
