use crate::styling::theme::ThemeStorage;
use gpui::{
    App, BorderStyle, Bounds, Context, DefiniteLength, Div, Edges, Element, ElementId,
    GlobalElementId, InspectorElementId, InteractiveElement, IntoElement, LayoutId, Length,
    ParentElement, Pixels, Point, Render, RenderOnce, ScrollHandle, Stateful,
    StatefulInteractiveElement, Style, Styled, UniformList, UniformListScrollHandle, Window, div,
    point, px, quad, rgb, size, transparent_black,
};
use std::panic::Location;
use std::sync::Arc;

#[derive(Clone)]
pub enum ScrollableScrollHandle {
    Interactive(ScrollHandle),
    UniformList(UniformListScrollHandle),
}

impl ScrollableScrollHandle {
    pub fn offset(&self) -> Point<Pixels> {
        match self {
            ScrollableScrollHandle::Interactive(handle) => handle.offset(),
            ScrollableScrollHandle::UniformList(handle) => handle.0.borrow().base_handle.offset(),
        }
    }

    pub fn max_offset(&self) -> Point<Pixels> {
        match self {
            ScrollableScrollHandle::Interactive(handle) => handle.max_offset(),
            ScrollableScrollHandle::UniformList(handle) => {
                handle.0.borrow().base_handle.max_offset()
            }
        }
    }

    pub fn bounds(&self) -> Bounds<Pixels> {
        match self {
            ScrollableScrollHandle::Interactive(handle) => handle.bounds(),
            ScrollableScrollHandle::UniformList(handle) => handle.0.borrow().base_handle.bounds(),
        }
    }
}

impl From<ScrollHandle> for ScrollableScrollHandle {
    fn from(value: ScrollHandle) -> Self {
        ScrollableScrollHandle::Interactive(value)
    }
}

impl From<UniformListScrollHandle> for ScrollableScrollHandle {
    fn from(value: UniformListScrollHandle) -> Self {
        ScrollableScrollHandle::UniformList(value)
    }
}

#[derive(IntoElement)]
pub struct ScrollbarContainer {
    handle: ScrollableScrollHandle,
}

impl RenderOnce for ScrollbarContainer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(
                div()
                    .flex()
                    .flex_grow()
                    .child(div().flex_grow())
                    .child(Scrollbar {
                        orientation: ScrollbarOrientation::Vertical,
                        handle: self.handle.clone(),
                    }),
            )
            .child(Scrollbar {
                orientation: ScrollbarOrientation::Horizontal,
                handle: self.handle.clone(),
            })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ScrollbarOrientation {
    Vertical,
    Horizontal,
}

pub struct Scrollbar {
    orientation: ScrollbarOrientation,
    handle: ScrollableScrollHandle,
}

impl IntoElement for Scrollbar {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

pub struct ScrollbarPrepaintState {
    thumb_bounds: Bounds<Pixels>,
    should_draw: bool,
}

impl Element for Scrollbar {
    type RequestLayoutState = ();
    type PrepaintState = ScrollbarPrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        match self.orientation {
            ScrollbarOrientation::Vertical => {
                style.size.width = Length::Definite(DefiniteLength::Absolute(px(4.).into()));
            }
            ScrollbarOrientation::Horizontal => {
                style.size.height = Length::Definite(DefiniteLength::Absolute(px(4.).into()));
            }
        }
        let layout_id = window.request_layout(style, [], cx);
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let (current_scroll_offset, max_offset, page_size, available_size, scrollbar_origin) =
            match self.orientation {
                ScrollbarOrientation::Vertical => (
                    -self.handle.offset().y,
                    self.handle.max_offset().y,
                    self.handle.bounds().size.height,
                    bounds.size.height,
                    bounds.origin.y,
                ),
                ScrollbarOrientation::Horizontal => (
                    -self.handle.offset().x,
                    self.handle.max_offset().x,
                    self.handle.bounds().size.width,
                    bounds.size.width,
                    bounds.origin.x,
                ),
            };
        let total_size = max_offset + page_size;

        let thumb_size = ((page_size / total_size) * available_size).max(px(20.));
        let available_track = available_size - thumb_size;
        let thumb_offset = scrollbar_origin
            + available_track
                * if max_offset > px(0.) {
                    (current_scroll_offset / max_offset).clamp(0., 1.)
                } else {
                    0.
                };

        let should_draw = page_size != total_size;

        ScrollbarPrepaintState {
            thumb_bounds: match self.orientation {
                ScrollbarOrientation::Vertical => Bounds::new(
                    point(bounds.origin.x, thumb_offset),
                    size(bounds.size.width, thumb_size),
                ),
                ScrollbarOrientation::Horizontal => Bounds::new(
                    point(thumb_offset, bounds.origin.y),
                    size(thumb_size, bounds.size.height),
                ),
            },
            should_draw,
        }
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        // Don't draw the scrollbar unnecessarily
        if !prepaint.should_draw {
            return;
        }

        let theme = cx.theme();
        let thumb_color = theme.button_background;
        let border_radius = theme.border_radius;
        window.paint_quad(quad(
            prepaint.thumb_bounds,
            border_radius,
            thumb_color,
            Edges::default(),
            transparent_black(),
            BorderStyle::Solid,
        ));
    }
}

pub trait Scrollable: Sized {
    type ReturnValue: IntoElement;

    fn scrollable(self, handle: impl Into<ScrollableScrollHandle>) -> Self::ReturnValue;
}

impl<T> Scrollable for Stateful<T>
where
    T: Element + ParentElement,
{
    type ReturnValue = Self;
    fn scrollable(self, handle: impl Into<ScrollableScrollHandle>) -> Self {
        self.child(
            div()
                .absolute()
                .left_0()
                .top_0()
                .w_full()
                .h_full()
                .child(ScrollbarContainer {
                    handle: handle.into(),
                }),
        )
    }
}

impl Scrollable for UniformList {
    type ReturnValue = Div;
    fn scrollable(self, handle: impl Into<ScrollableScrollHandle>) -> Div {
        div().child(self.w_full().h_full()).child(
            div()
                .absolute()
                .left_0()
                .top_0()
                .w_full()
                .h_full()
                .child(ScrollbarContainer {
                    handle: handle.into(),
                }),
        )
    }
}

pub trait SelfScrollable {
    type ReturnValue: IntoElement;
    fn self_scrollable(self, window: &mut Window, cx: &mut App) -> Self::ReturnValue;
}

impl<T> SelfScrollable for Stateful<T>
where
    T: InteractiveElement + Element + ParentElement,
{
    type ReturnValue = Self;
    fn self_scrollable(mut self, window: &mut Window, cx: &mut App) -> Self {
        let scroll_handle = window.use_keyed_state(
            ElementId::NamedChild(
                Arc::new(self.interactivity().element_id.clone().unwrap()),
                "-scrollable".into(),
            ),
            cx,
            |_, _| ScrollHandle::new(),
        );
        let scroll_handle = scroll_handle.read(cx);
        self.track_scroll(scroll_handle)
            .scrollable(scroll_handle.clone())
    }
}

impl SelfScrollable for UniformList {
    type ReturnValue = Div;
    fn self_scrollable(mut self, window: &mut Window, cx: &mut App) -> Div {
        let scroll_handle = window.use_keyed_state(
            ElementId::NamedChild(
                Arc::new(self.interactivity().element_id.clone().unwrap()),
                "-scrollable".into(),
            ),
            cx,
            |_, _| UniformListScrollHandle::new(),
        );
        let scroll_handle = scroll_handle.read(cx);
        self.track_scroll(scroll_handle)
            .scrollable(scroll_handle.clone())
    }
}
