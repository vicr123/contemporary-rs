use gpui::{App, Context, ElementId, Entity, Rgba, Window, transparent_black};
use std::time::Instant;

pub struct ErrorFlasher {
    start: Option<Instant>,
}

impl ErrorFlasher {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { start: None }
    }

    pub fn flash(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.start = Some(Instant::now());
        self.trigger_animation_frame_if_needed(window, cx);
    }

    pub fn color(&self) -> Rgba {
        match self.start {
            None => transparent_black().into(),
            Some(start) => {
                let duration = start.elapsed().as_secs_f32();
                Rgba {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: ((-duration + 0.5) * 2.).clamp(0.0, 1.0),
                }
            }
        }
    }

    fn trigger_animation_frame_if_needed(&self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(start) = self.start {
            let duration = start.elapsed().as_secs_f32();
            if duration < 0.5 {
                cx.notify();
                let weak_this = cx.weak_entity();
                window.on_next_frame(move |window, cx| {
                    let _ = weak_this.update(cx, |this, cx| {
                        this.trigger_animation_frame_if_needed(window, cx);
                    });
                });
            }
        }
    }
}

pub trait ErrorFlasherWindowExtensions {
    fn use_error_flasher(&mut self, cx: &mut App) -> Entity<ErrorFlasher>;
}

impl ErrorFlasherWindowExtensions for Window {
    #[track_caller]
    fn use_error_flasher(&mut self, cx: &mut App) -> Entity<ErrorFlasher> {
        self.use_keyed_state(
            ElementId::CodeLocation(*core::panic::Location::caller()),
            cx,
            |_, cx| ErrorFlasher::new(cx),
        )
    }
}
