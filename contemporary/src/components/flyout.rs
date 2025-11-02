use crate::components::raised::raised;
use crate::styling::theme::ThemeStorage;
use gpui::{
    AnyElement, App, Bounds, Div, InteractiveElement, IntoElement, ParentElement, Pixels, Point,
    RenderOnce, StyleRefinement, Styled, Window, anchored, div, point, px,
};
use std::rc::Rc;

pub struct FlyoutRequestCloseEvent;
pub type FlyoutRequestCloseListener =
    dyn Fn(&FlyoutRequestCloseEvent, &mut Window, &mut App) + 'static;

#[derive(IntoElement)]
pub struct Flyout {
    anchorer_bounds: Bounds<Pixels>,
    div: Div,
    visible: bool,
    request_close_listener: Option<Rc<Box<FlyoutRequestCloseListener>>>,
    anchor_position: FlyoutAnchorPoisition,
    anchor_gap: Pixels,
    as_deferred: bool,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum FlyoutAnchorPoisition {
    #[default]
    BottomLeft,
    TopRight,
}

pub fn flyout(anchorer_bounds: Bounds<Pixels>) -> Flyout {
    Flyout {
        anchorer_bounds,
        div: div(),
        visible: false,
        request_close_listener: None,
        anchor_position: FlyoutAnchorPoisition::BottomLeft,
        anchor_gap: px(2.),
        as_deferred: false,
    }
}

impl Flyout {
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn on_close(
        mut self,
        close_listener: impl Fn(&FlyoutRequestCloseEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.request_close_listener = Some(Rc::new(Box::new(close_listener)));
        self
    }

    pub fn anchor_position(mut self, anchor_position: FlyoutAnchorPoisition) -> Self {
        self.anchor_position = anchor_position;
        self
    }

    pub fn anchor_top_right(mut self) -> Self {
        self.anchor_position = FlyoutAnchorPoisition::TopRight;
        self
    }

    pub fn anchor_bottom_left(mut self) -> Self {
        self.anchor_position = FlyoutAnchorPoisition::BottomLeft;
        self
    }

    pub fn anchor_gap(mut self, anchor_gap: Pixels) -> Self {
        self.anchor_gap = anchor_gap;
        self
    }

    pub fn render_as_deferred(mut self, as_deferred: bool) -> Self {
        self.as_deferred = as_deferred;
        self
    }
}

impl RenderOnce for Flyout {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if !self.visible {
            return div().into_any_element();
        }

        let request_close_listener = self.request_close_listener.clone();
        let open_position = match self.anchor_position {
            FlyoutAnchorPoisition::BottomLeft => {
                self.anchorer_bounds.bottom_left() + point(px(0.), self.anchor_gap)
            }
            FlyoutAnchorPoisition::TopRight => {
                self.anchorer_bounds.top_right() + point(self.anchor_gap, px(0.))
            }
        };

        raised(move |_, window, cx| {
            let theme = cx.theme();

            let window_size = window.viewport_size();
            let inset = window.client_inset().unwrap_or_else(|| px(0.));

            anchored()
                .position(Point::new(px(0.), px(0.)))
                .child(
                    div()
                        .top_0()
                        .left_0()
                        .w(window_size.width - inset - inset)
                        .h(window_size.height - inset - inset)
                        .m(inset)
                        .occlude()
                        .on_any_mouse_down(move |_, window, cx| {
                            if let Some(request_close_listener) = request_close_listener.as_ref() {
                                request_close_listener(&FlyoutRequestCloseEvent, window, cx);
                            }
                        })
                        .child(
                            anchored().position(open_position).child(
                                self.div
                                    .bg(theme.background)
                                    .border(px(1.))
                                    .border_color(theme.border_color)
                                    .rounded(theme.border_radius),
                            ),
                        ),
                )
                .into_any_element()
        })
        .render_as_deferred(self.as_deferred)
        .into_any_element()
    }
}

impl Styled for Flyout {
    fn style(&mut self) -> &mut StyleRefinement {
        self.div.style()
    }
}

impl ParentElement for Flyout {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.div.extend(elements)
    }
}
