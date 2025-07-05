use gpui::{
    anchored, deferred, div, point, px, rgba, AnyElement, App,
    Div, InteractiveElement, IntoElement, ParentElement, RenderOnce, Styled, Window,
};

#[derive(IntoElement)]
pub struct Scrim {
    div: Div,
}

pub fn scrim() -> Scrim {
    Scrim { div: div() }
}

impl RenderOnce for Scrim {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let window_size = window.viewport_size();
        anchored().position(point(px(0.), px(0.))).child(deferred(
            div()
                .w(window_size.width)
                .h(window_size.height)
                .bg(rgba(0x00000090))
                .occlude()
                .child(self.div.w_full().h_full()),
        ))
    }
}

impl ParentElement for Scrim {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.div.extend(elements);
    }
}
