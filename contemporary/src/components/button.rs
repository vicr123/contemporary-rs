use crate::styling::theme::{Theme, VariableColor, variable_transparent};
use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, App, ClickEvent, Div, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, Stateful, StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px,
};

#[derive(IntoElement)]
pub struct Button {
    div: Stateful<Div>,
    flat: bool,
    disabled: bool,
    checked: bool,
    destructive: bool,
}

pub fn button(id: impl Into<ElementId>) -> Button {
    Button {
        div: div().id(id),
        flat: false,
        disabled: false,
        checked: false,
        destructive: false,
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

    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
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

        let background = if self.flat {
            variable_transparent()
        } else if self.destructive {
            theme.destructive_accent_color
        } else {
            theme.button_background
        };

        self.div
            .when_else(
                self.checked,
                |div| div.bg(background.active()),
                |div| div.bg(background),
            )
            .flex()
            .content_center()
            .justify_center()
            .p(px(6.0))
            // .pb(px(4.0))
            .text_color(theme.button_foreground)
            .rounded(theme.border_radius)
            .when(!self.disabled, |div| {
                div.when(!self.checked, |div| {
                    div.hover(|div| div.bg(background.hover()))
                })
                .active(|div| div.bg(background.active()))
            })
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        self.div.style()
    }
}
