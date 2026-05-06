use crate::components::scrollbar::SelfScrollable;
use gpui::{
    AnyElement, App, Context, ElementId, IntoElement, Refineable, RenderOnce, StyleRefinement,
    Styled, Window, uniform_list,
};
use std::cell::RefCell;

#[derive(IntoElement)]
pub struct ScrollArea {
    id: ElementId,
    child: Box<dyn Fn(&mut Window, &mut App) -> AnyElement>,
    style_refinement: StyleRefinement,
}

pub fn scroll_area(
    id: impl Into<ElementId>,
    child: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
) -> ScrollArea {
    ScrollArea {
        id: id.into(),
        child: Box::new(child),
        style_refinement: Default::default(),
    }
}

pub fn scroll_area_cx<T: 'static>(
    id: impl Into<ElementId>,
    child: impl Fn(&mut T, &mut Window, &mut Context<T>) -> AnyElement + 'static,
    cx: &mut Context<T>,
) -> ScrollArea {
    let weak_entity = cx.entity().downgrade();
    ScrollArea {
        id: id.into(),
        child: Box::new(move |window, cx| {
            weak_entity
                .update(cx, |t, cx| child(t, window, cx))
                .unwrap()
        }),
        style_refinement: Default::default(),
    }
}

impl RenderOnce for ScrollArea {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let child = self.child;
        let mut list = uniform_list(self.id, 1, move |_, window, cx| vec![child(window, cx)])
            .self_scrollable(window, cx);
        list.style().refine(&self.style_refinement);
        list
    }
}

impl Styled for ScrollArea {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style_refinement
    }
}
