use gpui::{
    App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div,
};

#[derive(IntoElement)]
pub struct Subtitle {
    text: SharedString,
}

pub fn subtitle(text: impl Into<SharedString>) -> Subtitle {
    Subtitle { text: text.into() }
}

impl RenderOnce for Subtitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .font_weight(FontWeight::BOLD)
            // TODO: Make locale aware
            .child(self.text.to_uppercase())
    }
}
