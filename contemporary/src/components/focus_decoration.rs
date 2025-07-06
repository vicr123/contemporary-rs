use crate::hsv::Hsva;
use crate::lerp::Lerpable;
use crate::styling::theme::Theme;
use gpui::{
    Animation, AnimationExt, App, Div, IntoElement, RenderOnce, StyleRefinement, Styled, Window,
    div, pulsating_between, px,
};
use std::time::Duration;

#[derive(IntoElement)]
pub struct FocusDecoration {
    div: Div,
}

pub fn focus_decoration() -> FocusDecoration {
    FocusDecoration { div: div() }
}

impl RenderOnce for FocusDecoration {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let p1 = theme.focus_decoration;
        let p2: Hsva = theme.focus_decoration.into();
        let p2 = p2.lighter(1.5);

        self.div
            .rounded(theme.border_radius)
            .border(px(3.))
            .with_animation(
                "focus-decoration-animation",
                Animation::new(Duration::from_secs(1))
                    .repeat()
                    .with_easing(pulsating_between(0., 1.)),
                move |div, delta| div.border_color(p1.lerp(p2.into(), delta)),
            )
    }
}

impl Styled for FocusDecoration {
    fn style(&mut self) -> &mut StyleRefinement {
        self.div.style()
    }
}
