use gpui::{
    AnyElement, App, ClickEvent, Div, Element, ElementId, InteractiveElement, IntoElement,
    ParentElement, RenderOnce, Stateful, StatefulInteractiveElement, Styled, Window, anchored,
    deferred, div, point, px, rgba,
};

#[derive(IntoElement)]
pub struct Scrim {
    div: Stateful<Div>,
}

pub fn scrim(id: impl Into<ElementId>) -> Scrim {
    Scrim { div: div().id(id) }
}

impl Scrim {
    pub fn on_click(mut self, fun: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.div = self.div.on_click(fun);
        self
    }
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
