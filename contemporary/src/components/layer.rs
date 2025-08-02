use crate::styling::theme::Theme;
use gpui::{
    AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
    div,
};

#[derive(IntoElement)]
pub struct Layer {
    div: Div,
}

pub fn layer() -> Layer {
    Layer { div: div() }
}

impl Layer {}

impl ParentElement for Layer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.div.extend(elements);
    }
}

impl Styled for Layer {
    fn style(&mut self) -> &mut StyleRefinement {
        self.div.style()
    }
}

impl RenderOnce for Layer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        self.div
            .bg(theme.layer_background)
            .rounded(theme.border_radius)
    }
}
