use gpui::{div, AnyElement, App, Div, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, Stateful, StyleRefinement, Styled, Window};
use crate::button::Button;

#[derive(IntoElement)]
pub struct Pager {
    div: Stateful<Div>,
    element: AnyElement,
    page: usize,
    current_page: usize,
}

pub fn pager(id: impl Into<ElementId>, page: usize) -> Pager {
    Pager {
        div: div().id(id),
        element: div().into_any_element(),
        page,
        current_page: 0,
    }
}

impl Pager {
    pub fn page(mut self, element: AnyElement) -> Pager {
        if self.current_page == self.page {
            self.element = element;
        }
        self.current_page += 1;
        self
    }
}

impl Styled for Pager {
    fn style(&mut self) -> &mut StyleRefinement {
        self.div.style()
    }
}

impl RenderOnce for Pager {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.div.child(self.element)
    }
}
