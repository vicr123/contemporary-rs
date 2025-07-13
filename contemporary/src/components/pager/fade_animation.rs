use crate::components::pager::pager_animation::{PagerAnimation, PagerAnimationDirection};
use gpui::{Div, Styled};

pub struct FadeAnimation {}

impl FadeAnimation {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl PagerAnimation for FadeAnimation {
    fn animate_in(&self, element: Div, _direction: PagerAnimationDirection, t: f32) -> Div {
        element.opacity(t)
    }

    fn animate_out(&self, element: Div, _direction: PagerAnimationDirection, _t: f32) -> Div {
        element
    }
}
