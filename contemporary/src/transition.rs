use crate::lerp::Lerpable;
use gpui::Animation;
use std::time::Instant;

pub mod float_transition_element;

pub struct Transition<TValueType> {
    start: Instant,
    animation: Animation,
    animate_from: TValueType,
    animate_to: TValueType,
}

impl<TValueType> Transition<TValueType>
where
    TValueType: Clone + PartialEq + Lerpable,
{
    pub fn new(animation: Animation, start_value: TValueType) -> Self {
        Self {
            start: Instant::now() - animation.duration,
            animation,
            animate_from: start_value.clone(),
            animate_to: start_value,
        }
    }

    pub fn set_new_target(&mut self, new_target: TValueType) {
        self.start = Instant::now();
        // TODO: This should be taken from the current value
        self.animate_from = std::mem::replace(&mut self.animate_to, new_target);
    }

    pub fn current_value(&self) -> TValueType {
        let mut delta = self.start.elapsed().as_secs_f32() / self.animation.duration.as_secs_f32();

        if delta > 1.0 {
            delta = 1.0;
        }
        let delta = (self.animation.easing)(delta);

        debug_assert!(
            (0.0..=1.0).contains(&delta),
            "delta should always be between 0 and 1"
        );

        self.animate_from.lerp(&self.animate_to, delta)
    }

    pub fn is_done(&self) -> bool {
        self.start.elapsed() > self.animation.duration
    }
}
