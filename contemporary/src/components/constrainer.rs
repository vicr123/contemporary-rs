use crate::components::anchorer::WithAnchorer;
use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, App, Bounds, Div, ElementId, InteractiveElement, IntoElement, ParentElement,
    Pixels, RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px,
};

#[derive(IntoElement)]
pub struct Constrainer {
    div: Div,
    id: ElementId,
    width: Pixels,
}

pub fn constrainer(id: impl Into<ElementId>) -> Constrainer {
    Constrainer {
        div: div(),
        id: id.into(),
        width: px(600.),
    }
}

impl Constrainer {
    pub fn inner_w(mut self, width: Pixels) -> Self {
        self.width = width;
        self
    }
}

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
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let bounds = window.use_state(cx, |_, _| None::<Bounds<Pixels>>);

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .items_center()
            .w_full()
            .overflow_y_scroll()
            .child(
                self.div.when_else(
                    bounds
                        .read(cx)
                        .is_some_and(|bounds| bounds.size.width > self.width),
                    |david| david.w(self.width),
                    |david| david.max_w(self.width).w_full(),
                ),
            )
            .with_anchorer({
                let bounds_entity = bounds.clone();
                move |david, bounds, _, cx| {
                    bounds_entity.write(cx, Some(bounds));
                    david
                }
            })
    }
}
