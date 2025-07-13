use gpui::Div;

#[derive(Clone, Copy)]
pub enum PagerAnimationDirection {
    Forward,
    Backward,
}

#[derive(PartialEq)]
pub enum PagerElement {
    Previous,
    Current,
}

pub trait PagerAnimation {
    fn top_element(&self, direction: PagerAnimationDirection) -> PagerElement {
        PagerElement::Current
    }

    fn animate_in(&self, element: Div, direction: PagerAnimationDirection, t: f32) -> Div;
    fn animate_out(&self, element: Div, direction: PagerAnimationDirection, t: f32) -> Div;
}
