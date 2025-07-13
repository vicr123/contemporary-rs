pub mod fade_animation;
pub mod lift_animation;
pub mod pager_animation;
pub mod slide_horizontal_animation;

use crate::components::pager::pager_animation::{
    PagerAnimation, PagerAnimationDirection, PagerElement,
};
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
    animation: Option<Box<dyn PagerAnimation>>,
    force_direction: Option<PagerAnimationDirection>,
}

pub fn pager(id: impl Into<ElementId>, page: usize) -> Pager {
    Pager {
        id: id.into(),
        elements: vec![],
        page,
        style_refinement: StyleRefinement::default(),
        animation: None,
        force_direction: None,
    }
}

impl Pager {
    pub fn page(mut self, element: AnyElement) -> Pager {
        self.elements.push(Some(
            div().absolute().w_full().h_full().occlude().child(element),
        ));
        self
    }

    pub fn animation(mut self, animation: Box<dyn PagerAnimation>) -> Pager {
        self.animation = Some(animation);
        self
    }

    pub fn animation_direction(mut self, direction: PagerAnimationDirection) -> Pager {
        self.force_direction = Some(direction);
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
            animation: self.animation,
            force_direction: self.force_direction,
        }
    }
}

struct PagerInternal {
    id: ElementId,
    current_page_number: usize,
    elements: Vec<Option<Div>>,
    style_refinement: StyleRefinement,
    animation_duration: Duration,
    animation: Option<Box<dyn PagerAnimation>>,
    force_direction: Option<PagerAnimationDirection>,
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
    top_element: PagerElement,
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

            let mut top_element = PagerElement::Current;

            // Set up animations
            if let Some(animation) = &mut self.animation {
                let animation_direction = self.force_direction.unwrap_or_else(|| {
                    if state.current_page > state.previous_current_page {
                        PagerAnimationDirection::Forward
                    } else {
                        PagerAnimationDirection::Backward
                    }
                });
                previous_page = animation.animate_out(previous_page, animation_direction, delta);
                current_page = animation.animate_in(current_page, animation_direction, delta);
                top_element = animation.top_element(animation_direction);
            }

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
                        top_element,
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
        if !request_layout.animation_done && self.animation.is_some() {
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
        if request_layout.top_element == PagerElement::Previous {
            // Render the current page at the bottom
            request_layout.current_page.paint(window, cx);
        }
        if !request_layout.animation_done && self.animation.is_some() {
            request_layout.previous_page.paint(window, cx);
        }
        if request_layout.top_element == PagerElement::Current {
            // Render the current page at the top
            request_layout.current_page.paint(window, cx);
        }
    }
}
