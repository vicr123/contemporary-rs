use crate::components::scrim::{Scrim, scrim};
use crate::styling::theme::Theme;
use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, Length, ParentElement, RenderOnce,
    Styled, Window, div, px, relative,
};
use std::cmp::PartialEq;
use std::ops::Sub;

#[derive(PartialEq)]
enum Anchor {
    Top,
    Leading,
    Trailing,
    Bottom,
}

enum Size {
    Length(Length),
    Neg(f32),
}

#[derive(IntoElement)]
pub struct Popover {
    scrim: Scrim,
    content: AnyElement,
    size: Size,
    anchor: Anchor,
}

pub fn popover(id: impl Into<ElementId>) -> Popover {
    Popover {
        scrim: scrim(id),
        content: div().into_any_element(),
        size: Size::Length(relative(1.).into()),
        anchor: Anchor::Bottom,
    }
}

impl Popover {
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = content.into_any_element();
        self
    }

    pub fn size(mut self, size: impl Into<Length>) -> Self {
        self.size = Size::Length(size.into());
        self
    }

    pub fn size_neg(mut self, size: f32) -> Self {
        self.size = Size::Neg(size);
        self
    }

    pub fn anchor_bottom(mut self) -> Self {
        self.anchor = Anchor::Bottom;
        self
    }

    pub fn anchor_top(mut self) -> Self {
        self.anchor = Anchor::Top;
        self
    }

    pub fn anchor_leading(mut self) -> Self {
        self.anchor = Anchor::Leading;
        self
    }

    pub fn anchor_trailing(mut self) -> Self {
        self.anchor = Anchor::Trailing;
        self
    }
}

impl RenderOnce for Popover {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        self.scrim.child(
            div()
                .flex()
                .w_full()
                .h_full()
                .when(self.anchor == Anchor::Top, |david| {
                    david.flex_col_reverse().child(div().flex_grow())
                })
                .when(self.anchor == Anchor::Bottom, |david| {
                    david.flex_col().child(div().flex_grow())
                })
                .when(self.anchor == Anchor::Leading, |david| {
                    david.flex_row_reverse().child(div().flex_grow())
                })
                .when(self.anchor == Anchor::Trailing, |david| {
                    david.flex_row().child(div().flex_grow())
                })
                .child(
                    div()
                        .bg(theme.background)
                        .rounded(theme.border_radius)
                        .flex()
                        .flex_col()
                        .when(
                            self.anchor == Anchor::Bottom || self.anchor == Anchor::Top,
                            |div| {
                                div.h(match self.size {
                                    Size::Length(length) => length,
                                    Size::Neg(pixels) => {
                                        window.viewport_size().height.sub(px(pixels)).into()
                                    }
                                })
                            },
                        )
                        .when(
                            self.anchor == Anchor::Leading || self.anchor == Anchor::Trailing,
                            |div| {
                                div.w(match self.size {
                                    Size::Length(length) => length,
                                    Size::Neg(pixels) => {
                                        window.viewport_size().width.sub(px(pixels)).into()
                                    }
                                })
                            },
                        )
                        .occlude()
                        .child(self.content),
                ),
        )
    }
}
