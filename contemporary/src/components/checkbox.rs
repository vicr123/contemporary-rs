use crate::styling::theme::Theme;
use gpui::MouseButton::Left;
use gpui::prelude::FluentBuilder;
use gpui::{
    App, AvailableSpace, BorderStyle, Bounds, ClickEvent, Corners, Div, Element, ElementId,
    GlobalElementId, InspectorElementId, InteractiveElement, IntoElement, LayoutId, ParentElement,
    Pixels, Refineable, RenderOnce, SharedString, Stateful, StatefulInteractiveElement, Style,
    StyleRefinement, Styled, Window, div, px, quad, size, transparent_black, white,
};
use std::panic::Location;

#[derive(PartialEq, Copy, Clone)]
pub enum CheckState {
    Off,
    On,
    Indeterminate,
}

#[derive(Copy, Clone)]
pub struct CheckedChangeEvent {
    pub check_state: CheckState,
}

type CheckedChangedHandlers = Vec<Box<dyn Fn(&CheckedChangeEvent, &mut Window, &mut App)>>;

#[derive(IntoElement)]
pub struct Checkbox {
    div: Stateful<Div>,
    check_state: CheckState,
    draw_as_radio: bool,
    label: Option<SharedString>,

    checked_changed_handlers: CheckedChangedHandlers,
}

pub fn checkbox(id: impl Into<ElementId>) -> Checkbox {
    Checkbox {
        div: div().id(id),
        check_state: CheckState::Off,
        draw_as_radio: false,
        label: None,
        checked_changed_handlers: Vec::new(),
    }
}

pub fn radio_button(id: impl Into<ElementId>) -> Checkbox {
    Checkbox {
        div: div().id(id),
        check_state: CheckState::Off,
        draw_as_radio: true,
        label: None,
        checked_changed_handlers: Vec::new(),
    }
}

impl Checkbox {
    pub fn checked(mut self) -> Self {
        self.check_state = CheckState::On;
        self
    }

    pub fn indeterminate(mut self) -> Self {
        self.check_state = CheckState::Indeterminate;
        self
    }

    pub fn check_state(mut self, state: CheckState) -> Self {
        self.check_state = state;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn on_checked_changed(
        mut self,
        listener: impl Fn(&CheckedChangeEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.checked_changed_handlers
            .push(Box::new(move |event, window, cx| {
                listener(event, window, cx)
            }));
        self
    }
}

impl RenderOnce for Checkbox {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let checked_changed_handlers = self.checked_changed_handlers;

        self.div
            .flex()
            .items_center()
            .gap(px(4.))
            .child(
                checkbox_box(self.check_state, self.draw_as_radio)
                    .m(px(2.))
                    .rounded(theme.border_radius)
                    .text_color(theme.foreground),
            )
            .when_some(self.label, |div, label| div.child(label))
            .on_click(move |event, window, cx| {
                if let ClickEvent::Mouse(mouse_event) = event
                    && mouse_event.down.button != Left
                {
                    return;
                }

                let event = CheckedChangeEvent {
                    check_state: match self.check_state {
                        CheckState::Indeterminate => CheckState::On,
                        CheckState::On => CheckState::Off,
                        CheckState::Off => CheckState::On,
                    },
                };

                for fun in checked_changed_handlers.iter() {
                    fun(&event, window, cx)
                }
            })
    }
}

struct CheckboxBox {
    check_state: CheckState,
    draw_as_radio: bool,
    style: StyleRefinement,
}

fn checkbox_box(check_state: CheckState, draw_as_radio: bool) -> CheckboxBox {
    CheckboxBox {
        check_state,
        draw_as_radio,
        style: StyleRefinement::default(),
    }
}

impl IntoElement for CheckboxBox {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Styled for CheckboxBox {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Element for CheckboxBox {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some("box".into())
    }

    fn source_location(&self) -> Option<&'static Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        _cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.refine(&self.style);
        let layout_id = window.request_measured_layout(style, |_, available_space, _, _| {
            let mut dim = match available_space.width {
                AvailableSpace::Definite(px) => px.0,
                AvailableSpace::MinContent => f32::NAN,
                AvailableSpace::MaxContent => f32::NAN,
            }
            .min(match available_space.height {
                AvailableSpace::Definite(px) => px.0,
                AvailableSpace::MinContent => f32::NAN,
                AvailableSpace::MaxContent => f32::NAN,
            });
            if dim.is_nan() {
                dim = 16.;
            }
            size(px(dim), px(dim))
        });

        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        // noop
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        _cx: &mut App,
    ) {
        let background = self
            .style
            .background
            .clone()
            .and_then(|background| background.color())
            .unwrap_or(transparent_black().into());
        let foreground = self
            .style
            .text
            .clone()
            .and_then(|text| text.color)
            .unwrap_or(white());

        if self.draw_as_radio {
            window.paint_quad(quad(
                bounds,
                Corners::all(px(bounds.size.width.0 / 2.)),
                background,
                px(1.),
                foreground,
                BorderStyle::Solid,
            ));

            if self.check_state == CheckState::On {
                window.paint_quad(quad(
                    bounds,
                    Corners::all(px(bounds.size.width.0 / 2.)),
                    transparent_black(),
                    px(bounds.size.width.0 / 4.),
                    foreground,
                    BorderStyle::Solid,
                ));
            }
        } else {
            let mut corners = Corners::default();
            corners.refine(&self.style.corner_radii);
            window.paint_quad(quad(
                bounds,
                corners.to_pixels(window.rem_size()),
                background,
                px(1.),
                foreground,
                BorderStyle::Solid,
            ));

            if self.check_state == CheckState::On {
                window.paint_quad(quad(
                    bounds,
                    corners.to_pixels(window.rem_size()),
                    foreground,
                    px(1.),
                    foreground,
                    BorderStyle::Solid,
                ));
            }
        }
    }
}
