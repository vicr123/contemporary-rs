use cntp_i18n::tr;
use contemporary::components::checkbox::{CheckState, CheckedChangeEvent, checkbox, radio_button};
use contemporary::components::constrainer::constrainer;
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::subtitle::subtitle;
use contemporary::styling::theme::Theme;
use gpui::prelude::FluentBuilder;
use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
};

pub struct CheckboxesRadioButtons {
    default_off_checkbox: CheckState,
    default_on_checkbox: CheckState,
    default_radio: u8,
}

impl CheckboxesRadioButtons {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| CheckboxesRadioButtons {
            default_off_checkbox: CheckState::Off,
            default_on_checkbox: CheckState::On,
            default_radio: 1,
        })
    }
}

impl Render for CheckboxesRadioButtons {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .bg(theme.background)
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(
                grandstand("checkboxes-radio-buttons-grandstand")
                    .text(tr!(
                        "CHECKBOXES_RADIO_BUTTONS_TITLE",
                        "Checkboxes & Radio Buttons"
                    ))
                    .pt(px(36.)),
            )
            .child(
                constrainer("checkboxes-radio-buttons")
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
                            .child(subtitle(tr!("CHECKBOXES_TITLE", "Checkboxes")))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(8.))
                                    .child(
                                        checkbox("default-off-checkbox")
                                            .check_state(self.default_off_checkbox)
                                            .label(tr!("CHECKBOXES_OFF", "Off"))
                                            .on_checked_changed(cx.listener(
                                                |this, event: &CheckedChangeEvent, _, cx| {
                                                    this.default_off_checkbox = event.check_state;
                                                    cx.notify()
                                                },
                                            )),
                                    )
                                    .child(
                                        checkbox("default-on-checkbox")
                                            .check_state(self.default_on_checkbox)
                                            .label(tr!("CHECKBOXES_ON", "On"))
                                            .on_checked_changed(cx.listener(
                                                |this, event: &CheckedChangeEvent, _, cx| {
                                                    this.default_on_checkbox = event.check_state;
                                                    cx.notify()
                                                },
                                            )),
                                    ),
                            ),
                    )
                    .child(
                        layer()
                            .flex()
                            .flex_col()
                            .p(px(8.))
                            .w_full()
                            .child(subtitle(tr!("RADIO_BUTTONS_TITLE", "Radio Buttons")))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(8.))
                                    .child(
                                        radio_button("default-off-radio")
                                            .when(self.default_radio == 0, |radio_button| {
                                                radio_button.checked()
                                            })
                                            .label(tr!("CHECKBOXES_OFF"))
                                            .on_checked_changed(cx.listener(
                                                |this, event: &CheckedChangeEvent, _, cx| {
                                                    if event.check_state == CheckState::On {
                                                        this.default_radio = 0;
                                                        cx.notify()
                                                    }
                                                },
                                            )),
                                    )
                                    .child(
                                        radio_button("default-on-radio")
                                            .when(self.default_radio == 1, |radio_button| {
                                                radio_button.checked()
                                            })
                                            .label(tr!("CHECKBOXES_ON"))
                                            .on_checked_changed(cx.listener(
                                                |this, event: &CheckedChangeEvent, _, cx| {
                                                    if event.check_state == CheckState::On {
                                                        this.default_radio = 1;
                                                        cx.notify()
                                                    }
                                                },
                                            )),
                                    ),
                            ),
                    ),
            )
    }
}
