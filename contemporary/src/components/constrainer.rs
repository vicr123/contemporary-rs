use gpui::{
    AnyElement, App, Div, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px,
};

#[derive(IntoElement)]
pub struct Constrainer {
    div: Div,
    id: ElementId,
}

pub fn constrainer(id: impl Into<ElementId>) -> Constrainer {
    Constrainer {
        div: div(),
        id: id.into(),
    }
}

impl Constrainer {}

impl ParentElement for Constrainer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.div.extend(elements);
    }
}

impl Styled for Constrainer {
    fn style(&mut self) -> &mut StyleRefinement {
        self.div.style()
    }
}

impl RenderOnce for Constrainer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .items_center()
            .w_full()
            .overflow_y_scroll()
            .child(self.div.max_w(px(600.)).w_full())
    }
}
