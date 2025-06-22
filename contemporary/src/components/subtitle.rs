use gpui::{
    div, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window,
};

#[derive(IntoElement)]
pub struct Subtitle {
    text: SharedString,
}

pub fn subtitle(text: SharedString) -> Subtitle {
    Subtitle { text }
}

impl RenderOnce for Subtitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .font_weight(FontWeight::BOLD)
            // TODO: Make locale aware
            .child(self.text.to_uppercase())
    }
}
