use crate::platform_support::platform_settings::PlatformSettings;
use crate::styling::theme::Theme;
use crate::transition::Transition;
use gpui::{
    Animation, App, BorderStyle, Bounds, Corners, Element, ElementId, GlobalElementId,
    InspectorElementId, IntoElement, LayoutId, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    Pixels, Point, Refineable, SharedString, Size, Style, StyleRefinement, Styled, Window, px,
    quad, transparent_white,
};
use std::cell::RefCell;
use std::panic::Location;
use std::rc::Rc;

#[derive(Copy, Clone)]
pub struct SwitchChangeEvent {
    pub checked: bool,
}

type SwitchChangeHandler = dyn Fn(&SwitchChangeEvent, &mut Window, &mut App);

pub struct Switch {
    id: ElementId,
    style_refinement: StyleRefinement,
    disabled: bool,
    checked: bool,
    label: Option<SharedString>,
    on_change: Option<Rc<SwitchChangeHandler>>,
}

pub fn switch(id: impl Into<ElementId>) -> Switch {
    Switch {
        id: id.into(),
        style_refinement: StyleRefinement::default().h(px(24.)).w(px(48.)),
        disabled: false,
        checked: false,
        label: None,
        on_change: None,
    }
}

impl Switch {
    pub fn checked(mut self) -> Self {
        self.checked = true;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn on_change(
        mut self,
        on_change: impl Fn(&SwitchChangeEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(on_change));
        self
    }
}

impl IntoElement for Switch {
    type Element = Switch;

    fn into_element(self) -> Self::Element {
        self
    }
}

pub struct SliderPrepaintState {
    thumb_full_size: Size<Pixels>,
    thumb_bounds: Bounds<Pixels>,
    fill_bounds: Bounds<Pixels>,
}

struct SwitchInteractiveState {
    active_state: Option<SwitchInteractiveActiveState>,
    thumb_inset: Transition<f32>,
}

#[derive(Clone)]
struct SwitchInteractiveActiveState {
    start_drag_coordinate: f32,
    current_drag_coordinate: f32,
    started_checked: bool,
    mouse_moved: bool,
}

pub struct SwitchPrepaintState {
    thumb_full_size: Size<Pixels>,
    thumb_bounds: Bounds<Pixels>,
    fill_bounds: Bounds<Pixels>,
}

impl Styled for Switch {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style_refinement
    }
}

impl Element for Switch {
    type RequestLayoutState = ();
    type PrepaintState = SwitchPrepaintState;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let layout = window.request_layout(
            Style::default().refined(self.style_refinement.clone()),
            vec![],
            cx,
        );

        (layout, ())
    }

    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let platform_settings = cx.global::<PlatformSettings>();
        let state_cell = window.with_optional_element_state(id, |state, cx| {
            let state = state.flatten().unwrap_or_else(|| {
                Rc::new(RefCell::new(SwitchInteractiveState {
                    active_state: None,
                    thumb_inset: Transition::new(
                        Animation::new(platform_settings.animation_duration / 4),
                        0.,
                    ),
                }))
            });

            (state.clone(), Some(state))
        });
        let state = state_cell.borrow();

        let percentage = if self.checked { 1. } else { 0. };
        let thumb_size = bounds.size.height;
        let thumb_x = (bounds.size.width - thumb_size) * percentage;

        let drag_delta = state
            .active_state
            .as_ref()
            .map(|active_state| {
                active_state.current_drag_coordinate - active_state.start_drag_coordinate
            })
            .unwrap_or_default();

        let fill_size = px(
            (f32::from(thumb_x) + f32::from(thumb_size) + drag_delta).clamp(
                f32::from(thumb_size).min(bounds.size.width.into()),
                bounds.size.width.into(),
            ),
        );

        if !state.thumb_inset.is_done() {
            window.request_animation_frame();
        }

        SwitchPrepaintState {
            thumb_full_size: Size {
                width: thumb_size,
                height: thumb_size,
            },
            thumb_bounds: Bounds {
                origin: Point {
                    x: bounds.origin.x
                        + px((f32::from(thumb_x) + drag_delta).clamp(
                            0.,
                            0_f32.max(f32::from(bounds.size.width) - f32::from(thumb_size)),
                        )),
                    y: bounds.origin.y,
                },
                size: Size {
                    width: thumb_size.min(bounds.size.width),
                    height: thumb_size.min(bounds.size.height),
                },
            }
            .inset(px(state.thumb_inset.current_value())),
            fill_bounds: Bounds {
                origin: bounds.origin,
                size: Size {
                    width: fill_size,
                    height: bounds.size.height,
                },
            },
        }
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let theme = cx.global::<Theme>().clone().disable_when(self.disabled);

        window.paint_quad(quad(
            bounds,
            Corners::all(theme.border_radius),
            theme.layer_background,
            px(0.),
            transparent_white(),
            BorderStyle::Solid,
        ));

        window.paint_quad(quad(
            prepaint.fill_bounds,
            Corners::all(theme.border_radius),
            theme.button_background,
            px(0.),
            transparent_white(),
            BorderStyle::Solid,
        ));

        window.paint_quad(quad(
            prepaint.thumb_bounds,
            Corners::all(theme.border_radius),
            theme.foreground,
            px(0.),
            transparent_white(),
            BorderStyle::Solid,
        ));

        window.paint_quad(quad(
            bounds,
            Corners::all(theme.border_radius),
            transparent_white(),
            px(1.),
            theme.border_color,
            BorderStyle::Solid,
        ));

        let change_event = self.on_change.clone();
        let thumb_full_size = prepaint.thumb_full_size;
        let thumb_bounds = prepaint.thumb_bounds;
        let mouse_down_current_view = window.current_view();
        let mouse_up_current_view = window.current_view();
        let mouse_move_current_view = window.current_view();
        let checked = self.checked;

        window.with_optional_element_state(id, |state, cx| {
            let state: Rc<RefCell<SwitchInteractiveState>> = state
                .flatten()
                .expect("slider state not set during prepaint");

            let mouse_down_state = state.clone();
            let mouse_move_state = state.clone();
            let mouse_up_state = state.clone();

            if !self.disabled {
                cx.on_mouse_event(move |event: &MouseDownEvent, _, window, cx| {
                    if !bounds.contains(&event.position) {
                        return;
                    }

                    window.prevent_default();
                    cx.stop_propagation();

                    let mut state = mouse_down_state.borrow_mut();
                    state.active_state = Some(SwitchInteractiveActiveState {
                        start_drag_coordinate: event.position.x.into(),
                        current_drag_coordinate: event.position.x.into(),
                        started_checked: checked,
                        mouse_moved: false,
                    });
                    state
                        .thumb_inset
                        .set_new_target(f32::from(thumb_full_size.width) / 8.);
                    cx.notify(mouse_down_current_view);
                });
                cx.on_mouse_event(move |event: &MouseMoveEvent, _, window, cx| {
                    let mut state = mouse_move_state.borrow_mut();

                    let Some(active_state) = &mut state.active_state else {
                        return;
                    };

                    window.prevent_default();
                    cx.stop_propagation();

                    active_state.current_drag_coordinate = event.position.x.into();
                    active_state.mouse_moved = true;
                    cx.notify(mouse_move_current_view);
                });
                cx.on_mouse_event(move |event: &MouseUpEvent, _, window, cx| {
                    let mut state = mouse_up_state.borrow_mut();

                    let Some(active_state) = &mut state.active_state else {
                        return;
                    };

                    window.prevent_default();
                    cx.stop_propagation();

                    if let Some(change_event) = change_event.as_ref() {
                        if !active_state.mouse_moved {
                            // Flip the switch
                            change_event(
                                &SwitchChangeEvent {
                                    checked: !active_state.started_checked,
                                },
                                window,
                                cx,
                            );
                        } else {
                            let new_state = thumb_bounds.center().x > bounds.center().x;
                            change_event(&SwitchChangeEvent { checked: new_state }, window, cx);
                        }
                    }

                    state.active_state = None;
                    state.thumb_inset.set_new_target(0.);
                    cx.notify(mouse_up_current_view);
                });
            }

            ((), Some(state))
        });
    }
}
