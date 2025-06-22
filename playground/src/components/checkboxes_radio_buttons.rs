use contemporary::constrainer::constrainer;
use contemporary::grandstand::grandstand;
use contemporary_i18n::tr;
use gpui::{div, px, App, IntoElement, ParentElement, RenderOnce, Styled, Window};

#[derive(IntoElement)]
pub struct CheckboxesRadioButtons;

pub fn checkboxes_radio_buttons() -> CheckboxesRadioButtons {
    CheckboxesRadioButtons {}
}

impl RenderOnce for CheckboxesRadioButtons {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
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
                    .child("Children"),
            )
    }
}
