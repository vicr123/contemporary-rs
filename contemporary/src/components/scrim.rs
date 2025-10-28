use crate::components::raised::raised;
use crate::platform_support::platform_settings::PlatformSettings;
use crate::styling::theme::Theme;
use crate::transition::float_transition_element::TransitionExt;
use gpui::prelude::FluentBuilder;
use gpui::{
    Animation, AnyElement, App, ClickEvent, Div, ElementId, InteractiveElement, IntoElement,
    ParentElement, RenderOnce, Stateful, StatefulInteractiveElement, Styled, Window, anchored,
    black, div, point, px,
};

#[derive(IntoElement)]
pub struct Scrim {
    root_div: Stateful<Div>,
    scrim_div: Stateful<Div>,
    child_div: Div,
    visible: bool,
    as_deferred: bool,
}

pub fn scrim(id: impl Into<ElementId>) -> Scrim {
    Scrim {
        root_div: div().id(id),
        scrim_div: div().id("scrim"),
        child_div: div(),
        visible: false,
        as_deferred: false,
    }
}

impl Scrim {
    pub fn on_click(mut self, fun: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.scrim_div = self.scrim_div.on_click(fun);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn render_as_deferred(mut self, as_deferred: bool) -> Self {
        self.as_deferred = as_deferred;
        self
    }
}

impl RenderOnce for Scrim {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let root_div = self.root_div;
        let child_div = self.child_div;
        let visible = self.visible;
        raised(move |_, window, cx| {
            let theme = cx.global::<Theme>();
            let platform_settings = cx.global::<PlatformSettings>();
            let window_size = window.viewport_size();
            let inset = window.client_inset().unwrap_or_else(|| px(0.));

            anchored()
                .position(point(px(0.), px(0.)))
                .child(
                    root_div
                        .w(window_size.width - inset - inset)
                        .h(window_size.height - inset - inset)
                        .m(inset)
                        .rounded(theme.border_radius)
                        .child(
                            self.scrim_div
                                .absolute()
                                .bg(black())
                                .w_full()
                                .h_full()
                                .with_transition(
                                    "scrim-transition",
                                    if visible { 0.7 } else { 0. },
                                    Animation::new(platform_settings.animation_duration),
                                    |div, opacity| {
                                        div.opacity(opacity).when_else(
                                            opacity == 0.,
                                            |div| div.invisible(),
                                            |div| div.occlude(),
                                        )
                                    },
                                ),
                        )
                        .child(child_div.absolute().w_full().h_full()),
                )
                .into_any_element()
        })
        .render_as_deferred(self.as_deferred)
    }
}

impl ParentElement for Scrim {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.child_div.extend(elements);
    }
}
