use crate::components::pager::pager_animation::{
    PagerAnimation, PagerAnimationDirection, PagerElement,
};
use gpui::{Div, Styled, ease_out_quint, px};

pub struct LiftAnimation {}

impl LiftAnimation {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl PagerAnimation for LiftAnimation {
    fn top_element(&self, direction: PagerAnimationDirection) -> PagerElement {
        match direction {
            PagerAnimationDirection::Forward => PagerElement::Current,
            PagerAnimationDirection::Backward => PagerElement::Previous,
        }
    }

    fn animate_in(&self, element: Div, direction: PagerAnimationDirection, t: f32) -> Div {
        match direction {
            PagerAnimationDirection::Forward => {
                let t = ease_out_quint()(t);
                element.opacity(t).top(px(50. * (1. - t)))
            }
            PagerAnimationDirection::Backward => element,
        }
    }

    fn animate_out(&self, element: Div, direction: PagerAnimationDirection, t: f32) -> Div {
        match direction {
            PagerAnimationDirection::Forward => element,
            PagerAnimationDirection::Backward => {
                let t = ease_out_quint()(1. - t);
                element.opacity(t).top(px(50. * (1. - t)))
            }
        }
    }
}
