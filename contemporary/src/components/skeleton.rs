use crate::styling::theme::Theme;
use gpui::prelude::FluentBuilder;
use gpui::{
    Animation, AnimationExt, AnyElement, App, Div, ElementId, InteractiveElement, IntoElement,
    ParentElement, Refineable, RenderOnce, Stateful, StyleRefinement, Styled, Window, div,
    ease_in_out,
};
use std::time::Duration;

#[derive(IntoElement)]
pub struct Skeleton {
    id: ElementId,
    inner: Div,
    extended: bool,
    style_refinement: StyleRefinement,
    text: String,
}

pub fn skeleton(id: impl Into<ElementId>) -> Skeleton {
    Skeleton {
        id: id.into(),
        inner: div(),
        extended: false,
        style_refinement: Default::default(),
        text: "DEFAULT".to_string(),
    }
}

impl ParentElement for Skeleton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.inner.extend(elements);
        self.extended = true
    }
}

impl RenderOnce for Skeleton {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let mut david = div().bg(theme.skeleton).rounded(theme.border_radius).child(
            self.inner
                .when(!self.extended, |david| david.child("DEFAULT"))
                .opacity(0.),
        );

        david.style().refine(&self.style_refinement);

        david.with_animation(
            self.id,
            Animation::new(Duration::from_millis(2000))
                .repeat()
                .with_easing(|progress| {
                    ease_in_out(
                        2. * if progress < 0.5 {
                            progress
                        } else {
                            0.5 - (progress - 0.5)
                        },
                    )
                }),
            |div, progress| div.opacity(0.1 + progress * 0.15),
        )
    }
}

impl Styled for Skeleton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style_refinement
    }
}

/// An extension trait for turning an element into a skeleton
pub trait SkeletonExt {
    /// Render this component or element as a skeleton
    fn into_skeleton(self, id: impl Into<ElementId>) -> Skeleton
    where
        Self: Sized + IntoElement,
    {
        skeleton(id).child(self.into_element())
    }

    /// Render this component or element as a skeleton when the condition is true
    fn into_skeleton_when(self, condition: bool, id: impl Into<ElementId>) -> AnyElement
    where
        Self: Sized + IntoElement,
    {
        if condition {
            skeleton(id).child(self.into_element()).into_any_element()
        } else {
            self.into_any_element()
        }
    }
}

impl<E> SkeletonExt for E {}

#[derive(IntoElement)]
pub struct SkeletonRow {
    id: ElementId,
    chunks: Vec<String>,
}

pub fn skeleton_row(id: impl Into<ElementId>) -> SkeletonRow {
    SkeletonRow {
        id: id.into(),
        chunks: Vec::new(),
    }
}

impl SkeletonRow {
    pub fn chunk(mut self, chunk: impl Into<String>) -> Self {
        self.chunks.push(chunk.into());
        self
    }
}

impl RenderOnce for SkeletonRow {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let chunk_count = self.chunks.len();
        self.chunks.iter().enumerate().fold(
            div().id(self.id).flex().items_start(),
            |david: Stateful<Div>, (i, chunk)| {
                david
                    .child(skeleton(i).child(chunk.clone()))
                    .when(i != chunk_count - 1, |david| david.child(" "))
            },
        )
    }
}
