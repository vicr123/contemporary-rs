use crate::styling::theme::Theme;
use gpui::{
    AnyElement, App, Bounds, Div, Element, ElementId, GlobalElementId, InspectorElementId,
    InteractiveElement, IntoElement, LayoutId, ParentElement, Pixels, Refineable, RenderOnce,
    Style, StyleRefinement, Styled, Window, div,
};
use std::panic::Location;
use std::time::{Duration, Instant};

#[derive(IntoElement)]
pub struct Pager {
    id: ElementId,
    elements: Vec<Option<Div>>,
    page: usize,
    style_refinement: StyleRefinement,
}

pub fn pager(id: impl Into<ElementId>, page: usize) -> Pager {
    Pager {
        id: id.into(),
        elements: vec![],
        page,
        style_refinement: StyleRefinement::default(),
    }
}

impl Pager {
    pub fn page(mut self, element: AnyElement) -> Pager {
        self.elements.push(Some(
            div().absolute().w_full().h_full().occlude().child(element),
        ));
        self
    }
}

impl Styled for Pager {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style_refinement
    }
}

impl RenderOnce for Pager {
    fn render(mut self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        PagerInternal {
            id: self.id,
            style_refinement: self.style_refinement,
            current_page_number: self.page,
            elements: self.elements,
            animation_duration: theme.animation_duration,
        }
    }
}

struct PagerInternal {
    id: ElementId,
    current_page_number: usize,
    elements: Vec<Option<Div>>,
    style_refinement: StyleRefinement,
    animation_duration: Duration,
}

impl IntoElement for PagerInternal {
    type Element = PagerInternal;

    fn into_element(self) -> Self::Element {
        self
    }
}

struct PagerInternalState {
    animation_start: Instant,
    previous_current_page: usize,
    current_page: usize,
}

struct PagerRequestLayoutState {
    current_page: AnyElement,
    previous_page: AnyElement,
    animation_done: bool,
}

impl Element for PagerInternal {
    type RequestLayoutState = PagerRequestLayoutState;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        window.with_element_state(id.unwrap(), |state, window| {
            let mut state = state.unwrap_or_else(|| PagerInternalState {
                animation_start: Instant::now(),
                previous_current_page: self.current_page_number,
                current_page: self.current_page_number,
            });

            if state.current_page != self.current_page_number {
                state.animation_start = Instant::now();
                state.previous_current_page = state.current_page;
                state.current_page = self.current_page_number;
            }

            let mut delta = state.animation_start.elapsed().as_secs_f32()
                / self.animation_duration.as_secs_f32();

            let mut done = false;
            if delta > 1.0 {
                done = true;
                delta = 1.0;
            }

            let mut current_page = self
                .elements
                .get_mut(state.current_page)
                .unwrap_or(&mut None)
                .take()
                .unwrap_or_else(|| div());
            let mut previous_page = self
                .elements
                .get_mut(state.previous_current_page)
                .unwrap_or(&mut None)
                .take()
                .unwrap_or_else(|| div());

            // Set up animations
            previous_page = previous_page.opacity(1. - delta);
            current_page = current_page.opacity(delta);

            let mut previous_page = previous_page.into_any_element();
            let mut current_page = current_page.into_any_element();

            let previous_page_layout_id = previous_page.request_layout(window, cx);
            let current_page_layout_id = current_page.request_layout(window, cx);

            let final_layout_id = window.request_layout(
                Style::default().refined(self.style_refinement.clone()),
                vec![current_page_layout_id, previous_page_layout_id],
                cx,
            );

            if !done {
                window.request_animation_frame();
            }

            (
                (
                    final_layout_id,
                    PagerRequestLayoutState {
                        current_page,
                        previous_page,
                        animation_done: done,
                    },
                ),
                state,
            )
        })
    }

    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        if !request_layout.animation_done {
            request_layout.previous_page.prepaint(window, cx);
        }
        request_layout.current_page.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        if !request_layout.animation_done {
            request_layout.previous_page.paint(window, cx);
        }
        request_layout.current_page.paint(window, cx);
    }
}
