use crate::styling::theme::Theme;
use gpui::prelude::FluentBuilder;
use gpui::{
    div, px, rgba, AnyElement, App, ClickEvent, Div, ElementId,
    InteractiveElement, IntoElement, ParentElement, RenderOnce, Stateful, StatefulInteractiveElement, StyleRefinement, Styled,
    Window,
};

#[derive(IntoElement)]
pub struct Button {
    div: Stateful<Div>,
    flat: bool,
    disabled: bool,
    checked: bool,
}

pub fn button(id: impl Into<ElementId>) -> Button {
    Button {
        div: div().id(id),
        flat: false,
        disabled: false,
        checked: false,
    }
}

impl Button {
    pub fn flat(mut self) -> Self {
        self.flat = true;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn checked(mut self) -> Self {
        self.checked = true;
        self
    }

    pub fn checked_when(self, condition: bool) -> Self {
        if condition { self.checked() } else { self }
    }

    pub fn on_click(mut self, fun: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        let disabled = self.disabled;
        self.div = self.div.on_click(move |event, window, cx| {
            if !disabled {
                fun(event, window, cx)
            }
        });
        self
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.div.extend(elements);
    }
}

impl InteractiveElement for Button {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.div.interactivity()
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>().disable_when(self.disabled);

        let hover_color = if self.flat {
            rgba(0xFFFFFF4B)
        } else {
            theme.button_hover_background
        };
        let active_color = if self.flat {
            rgba(0x0000004B)
        } else {
            theme.button_active_background
        };

        self.div
            .when_else(
                self.checked,
                |div| div.bg(active_color),
                |div| div.when(!self.flat, |div| div.bg(theme.button_background)),
            )
            .flex()
            .content_center()
            .justify_center()
            .p(px(6.0))
            .pb(px(4.0))
            .text_color(theme.button_foreground)
            .rounded(theme.border_radius)
            .when(!self.disabled, |div| {
                div.when(!self.checked, |div| {
                    div.hover(|div| {
                        div.bg(if self.flat {
                            theme.layer_background
                        } else {
                            hover_color
                        })
                    })
                })
                .active(|div| div.bg(active_color))
            })
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        self.div.style()
    }
}
