use gpui::{div, px, App, IntoElement, ParentElement, RenderOnce, Styled, Window};
use contemporary::constrainer::constrainer;
use contemporary::grandstand::grandstand;
use contemporary_i18n::tr;

#[derive(IntoElement)]
pub struct TextInput;

pub fn text_input() -> TextInput {
    TextInput {}
}

impl RenderOnce for TextInput {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(
                grandstand("text-input-grandstand")
                    .text(tr!(
                        "TEXT_INPUT_TITLE",
                        "Text Input"
                    ))
                    .pt(px(36.)),
            )
            .child(
                constrainer("text-input")
                    .flex()
                    .flex_col()
                    .w_full()
                    .p(px(8.))
                    .gap(px(8.))
                    .child("Children"),
            )
    }
}
