use crate::styling::theme::ThemeStorage;
use gpui::{
    Animation, AnimationExt, App, Div, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, Stateful, StyleRefinement, Styled, Window, div, px, relative,
};
use std::time::Duration;

#[derive(IntoElement)]
pub struct ProgressBar {
    div: Div,
    value: f32,
}

#[derive(IntoElement)]
pub struct IndeterminateProgressBar {
    div: Stateful<Div>,
    indeterminate: bool,
}

pub fn progress_bar() -> ProgressBar {
    ProgressBar {
        div: div(),
        value: 0.0,
    }
}

impl ProgressBar {
    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn indeterminate(self, id: impl Into<ElementId>) -> IndeterminateProgressBar {
        IndeterminateProgressBar {
            div: self.div.id(id),
            indeterminate: true,
        }
    }
}

impl RenderOnce for ProgressBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        self.div
            .rounded(theme.border_radius)
            .bg(theme.layer_background)
            .border_color(theme.border_color)
            .border(px(1.))
            .h(px(28.))
            .w_full()
            .child(
                div()
                    .left_0()
                    .top_0()
                    .h_full()
                    .w(relative(self.value))
                    .rounded(theme.border_radius)
                    .bg(theme.button_background),
            )
    }
}

impl Styled for ProgressBar {
    fn style(&mut self) -> &mut StyleRefinement {
        self.div.style()
    }
}

impl RenderOnce for IndeterminateProgressBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        self.div
            .rounded(theme.border_radius)
            .bg(theme.layer_background)
            .h(px(28.))
            .w_full()
            .relative()
            .child(
                div()
                    .top_0()
                    .h_full()
                    .rounded(theme.border_radius)
                    .bg(theme.button_background)
                    .absolute()
                    .with_animation(
                        "progress-bar-animation",
                        Animation::new(Duration::from_secs(5)).repeat(),
                        |div, delta| {
                            let delta = delta * 4.;
                            match delta.floor() {
                                0. => div
                                    .left(relative(delta))
                                    .w(relative(delta.clamp(0., 1. - delta))),
                                1. => div.left_0().w(relative((delta - 1.).clamp(0., 1.))),
                                2. => div.left_0().w(relative((delta - 2.).clamp(0., 1.))),
                                3. => div
                                    .left(relative(delta - 3.))
                                    .w(relative(1. - (delta - 3.))),
                                _ => unreachable!(),
                            }
                        },
                    ),
            )
            .child(
                div()
                    .left_0()
                    .top_0()
                    .h_full()
                    .rounded(theme.border_radius)
                    .bg(theme.button_background)
                    .absolute()
                    .with_animation(
                        "progress-bar-animation-2",
                        Animation::new(Duration::from_secs(5)).repeat(),
                        |div, delta| {
                            let delta = delta * 4.;
                            match delta.floor() {
                                0. | 1. | 3. => div.invisible(),
                                2. => div
                                    .left(relative((delta - 2.) * 2.))
                                    .w(relative(1. - ((delta - 2.) * 2.))),
                                _ => unreachable!(),
                            }
                        },
                    ),
            )
    }
}

impl Styled for IndeterminateProgressBar {
    fn style(&mut self) -> &mut StyleRefinement {
        self.div.style()
    }
}
