use crate::lerp::Lerpable;
use gpui::{
    Animation, AnyElement, App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId,
    IntoElement, LayoutId, Pixels, Rgba, Window,
};
use std::time::Instant;

pub struct TransitionElement<TElement, TValueType> {
    id: ElementId,
    element: Option<TElement>,
    animation: Animation,
    target_value: TValueType,
    animator: Box<dyn Fn(TElement, TValueType) -> TElement + 'static>,
}

impl<TElement, TValueType> TransitionElement<TElement, TValueType> {
    pub fn map_element(
        mut self,
        f: impl FnOnce(TElement) -> TElement,
    ) -> TransitionElement<TElement, TValueType> {
        self.element = self.element.map(f);
        self
    }
}

impl<TElement: IntoElement + 'static, TValueType: PartialEq + Clone + Lerpable + 'static>
    IntoElement for TransitionElement<TElement, TValueType>
{
    type Element = TransitionElement<TElement, TValueType>;

    fn into_element(self) -> Self::Element {
        self
    }
}

struct TransitionState<TValueType> {
    start: Instant,
    animate_from: TValueType,
    animate_to: TValueType,
}

impl<TElement: IntoElement + 'static, TValueType: PartialEq + Clone + Lerpable + 'static> Element
    for TransitionElement<TElement, TValueType>
{
    type RequestLayoutState = AnyElement;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        window.with_element_state(global_id.unwrap(), |state, window| {
            let mut state = state.unwrap_or_else(|| TransitionState {
                start: Instant::now(),
                animate_from: self.target_value.clone(),
                animate_to: self.target_value.clone(),
            });

            if self.target_value != state.animate_to {
                state.start = Instant::now();
                // TODO: This should be taken from the current value
                state.animate_from = state.animate_to;
                state.animate_to = self.target_value.clone();
            }

            let mut delta =
                state.start.elapsed().as_secs_f32() / self.animation.duration.as_secs_f32();

            let mut done = false;
            if delta > 1.0 {
                done = true;
                delta = 1.0;
            }
            let delta = (self.animation.easing)(delta);

            debug_assert!(
                (0.0..=1.0).contains(&delta),
                "delta should always be between 0 and 1"
            );

            let calculated_animation_value = state.animate_from.lerp(&state.animate_to, delta);

            let element = self.element.take().expect("should only be called once");
            let mut element =
                (self.animator)(element, calculated_animation_value).into_any_element();

            if !done {
                window.request_animation_frame();
            }

            ((element.request_layout(window, cx), element), state)
        })
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        element: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        element.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        element: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        element.paint(window, cx);
    }
}

/// An extension trait for adding the animation wrapper to both Elements and Components
pub trait TransitionExt<TTransitionValue> {
    /// Render this component or element with an animation
    fn with_transition(
        self,
        id: impl Into<ElementId>,
        target_value: TTransitionValue,
        animation: Animation,
        animator: impl Fn(Self, TTransitionValue) -> Self + 'static,
    ) -> TransitionElement<Self, TTransitionValue>
    where
        Self: Sized;
}

impl<E> TransitionExt<f32> for E {
    fn with_transition(
        self,
        id: impl Into<ElementId>,
        target_value: f32,
        animation: Animation,
        animator: impl Fn(Self, f32) -> Self + 'static,
    ) -> TransitionElement<Self, f32>
    where
        Self: Sized,
    {
        TransitionElement {
            id: id.into(),
            element: Some(self),
            animation,
            target_value,
            animator: Box::new(animator),
        }
    }
}

impl<E> TransitionExt<Rgba> for E {
    fn with_transition(
        self,
        id: impl Into<ElementId>,
        target_value: Rgba,
        animation: Animation,
        animator: impl Fn(Self, Rgba) -> Self + 'static,
    ) -> TransitionElement<Self, Rgba>
    where
        Self: Sized,
    {
        TransitionElement {
            id: id.into(),
            element: Some(self),
            animation,
            target_value,
            animator: Box::new(animator),
        }
    }
}
