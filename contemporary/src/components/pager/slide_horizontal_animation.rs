use crate::components::pager::pager_animation::{PagerAnimation, PagerAnimationDirection};
use crate::lerp::Lerpable;
use gpui::{Div, Styled, ease_out_quint, px};

pub struct SlideHorizontalAnimation {}

impl SlideHorizontalAnimation {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl PagerAnimation for SlideHorizontalAnimation {
    fn animate_in(&self, element: Div, direction: PagerAnimationDirection, t: f32) -> Div {
        let t = ease_out_quint()(t);
        element.opacity(t).left(px(match direction {
            PagerAnimationDirection::Forward => 50.0.lerp(&0., t),
            PagerAnimationDirection::Backward => -50.0.lerp(&0., t),
        }))
    }

    fn animate_out(&self, element: Div, direction: PagerAnimationDirection, t: f32) -> Div {
        let t = ease_out_quint()(t);
        element.opacity(1. - t).left(px(match direction {
            PagerAnimationDirection::Forward => 0.0.lerp(&-50., t),
            PagerAnimationDirection::Backward => 0.0.lerp(&50., t),
        }))
    }
}
