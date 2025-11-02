use gpui::prelude::FluentBuilder;
use gpui::{
    App, Bounds, Div, IntoElement, ParentElement, Pixels, RenderOnce, Styled, Window, canvas, div,
};

#[derive(IntoElement)]
pub struct Anchorer {
    render_fn: Box<dyn FnOnce(Div, Bounds<Pixels>, &mut Window, &mut App) -> Div>,
}

impl RenderOnce for Anchorer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let last_bounds = window.use_state(cx, |_, _| None);
        let last_bounds_clone = last_bounds.clone();

        div()
            .child(
                canvas(
                    move |bounds, _, cx| {
                        last_bounds.write(cx, Some(bounds));
                    },
                    |_, _, _, _| (),
                )
                .size_full(),
            )
            .when_some(*last_bounds_clone.read(cx), |david, last_bounds| {
                (self.render_fn)(david, last_bounds, window, cx)
            })
            .absolute()
            .size_full()
    }
}

pub trait WithAnchorer {
    fn with_anchorer(
        self,
        render: impl FnOnce(Div, Bounds<Pixels>, &mut Window, &mut App) -> Div + 'static,
    ) -> Self;
}

impl<T> WithAnchorer for T
where
    T: ParentElement + 'static,
{
    fn with_anchorer(
        self,
        render: impl FnOnce(Div, Bounds<Pixels>, &mut Window, &mut App) -> Div + 'static,
    ) -> Self {
        self.child(Anchorer {
            render_fn: Box::new(render),
        })
    }
}
