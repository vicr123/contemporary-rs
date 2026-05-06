use crate::components::scrollbar::SelfScrollable;
use gpui::{
    AnyElement, App, Context, ElementId, IntoElement, ListHorizontalSizingBehavior, Refineable,
    RenderOnce, StyleRefinement, Styled, Window, uniform_list,
};
use std::cell::RefCell;

#[derive(IntoElement)]
pub struct ScrollArea {
    id: ElementId,
    child: Box<dyn Fn(&mut Window, &mut App) -> AnyElement>,
    style_refinement: StyleRefinement,
    scroll_horizontally: bool,
}

pub fn scroll_area<Element>(
    id: impl Into<ElementId>,
    child: impl Fn(&mut Window, &mut App) -> Element + 'static,
) -> ScrollArea
where
    Element: IntoElement,
{
    ScrollArea {
        id: id.into(),
        child: Box::new(move |window, app| child(window, app).into_any_element()),
        style_refinement: Default::default(),
        scroll_horizontally: false,
    }
}

pub fn scroll_area_cx<Entity, Element>(
    id: impl Into<ElementId>,
    child: impl Fn(&mut Entity, &mut Window, &mut Context<Entity>) -> Element + 'static,
    cx: &mut Context<Entity>,
) -> ScrollArea
where
    Entity: 'static,
    Element: IntoElement,
{
    let weak_entity = cx.entity().downgrade();
    ScrollArea {
        id: id.into(),
        child: Box::new(move |window, cx| {
            weak_entity
                .update(cx, |t, cx| child(t, window, cx).into_any_element())
                .unwrap()
        }),
        style_refinement: Default::default(),
        scroll_horizontally: false,
    }
}

impl ScrollArea {
    pub fn overflow_x_scroll(mut self) -> Self {
        self.scroll_horizontally = true;
        self
    }
}

impl RenderOnce for ScrollArea {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let child = self.child;
        let mut list = uniform_list(self.id, 1, move |_, window, cx| vec![child(window, cx)])
            .with_horizontal_sizing_behavior(if self.scroll_horizontally {
                ListHorizontalSizingBehavior::Unconstrained
            } else {
                ListHorizontalSizingBehavior::FitList
            })
            .self_scrollable(window, cx);
        list.style().refine(&self.style_refinement);
        list
    }
}

impl Styled for ScrollArea {
    fn overflow_x_hidden(mut self) -> Self {
        self.scroll_horizontally = false;
        self
    }

    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style_refinement
    }
}
