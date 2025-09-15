use crate::platform_support::platform_settings::PlatformSettings;
use crate::styling::theme::Theme;
use crate::transition::Transition;
use gpui::{
    Animation, App, BorderStyle, Bounds, Corners, Element, ElementId, GlobalElementId,
    InspectorElementId, IntoElement, LayoutId, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    Pixels, Point, Refineable, Size, Style, StyleRefinement, Styled, Window, px, quad,
    transparent_white,
};
use std::cell::RefCell;
use std::panic::Location;
use std::rc::Rc;

type ChangeHandler = dyn FnMut(&SliderChangeEvent, &mut Window, &mut App);
type PressHandler = dyn FnMut(&SliderPressEvent, &mut Window, &mut App);
type ReleaseHandler = dyn FnMut(&SliderReleaseEvent, &mut Window, &mut App);

pub struct SliderChangeEvent {
    pub new_value: u32,
}

pub struct SliderPressEvent;
pub struct SliderReleaseEvent;

struct SliderInteractiveState {
    active_state: Option<SliderInteractiveActiveState>,
    thumb_inset: Transition<f32>,
}

#[derive(Clone)]
struct SliderInteractiveActiveState {
    start_drag_coordinate: f32,
    current_drag_coordinate: f32,
    start_value: u32,
}

pub struct Slider {
    id: ElementId,
    value: u32,
    max_value: u32,
    style_refinement: StyleRefinement,
    on_change: Option<Rc<RefCell<ChangeHandler>>>,
    on_press: Option<Rc<RefCell<PressHandler>>>,
    on_release: Option<Rc<RefCell<ReleaseHandler>>>,
    disabled: bool,
}

pub fn slider(id: impl Into<ElementId>) -> Slider {
    Slider {
        id: id.into(),
        value: 0,
        max_value: 100,
        style_refinement: StyleRefinement::default().h(px(28.)),
        on_change: None,
        on_release: None,
        on_press: None,
        disabled: false,
    }
}

impl Slider {
    pub fn value(mut self, value: u32) -> Self {
        self.value = value;
        self
    }

    pub fn max_value(mut self, value: u32) -> Self {
        self.max_value = value;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn on_change(
        mut self,
        on_change: impl FnMut(&SliderChangeEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(RefCell::new(on_change)));
        self
    }

    pub fn on_press(
        mut self,
        on_press: impl FnMut(&SliderPressEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_press = Some(Rc::new(RefCell::new(on_press)));
        self
    }

    pub fn on_release(
        mut self,
        on_release: impl FnMut(&SliderReleaseEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_release = Some(Rc::new(RefCell::new(on_release)));
        self
    }
}

impl IntoElement for Slider {
    type Element = Slider;

    fn into_element(self) -> Self::Element {
        self
    }
}

pub struct SliderPrepaintState {
    thumb_full_size: Size<Pixels>,
    thumb_bounds: Bounds<Pixels>,
    fill_bounds: Bounds<Pixels>,
}

impl Styled for Slider {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style_refinement
    }
}

impl Element for Slider {
    type RequestLayoutState = ();
    type PrepaintState = SliderPrepaintState;

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
                Rc::new(RefCell::new(SliderInteractiveState {
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

        let percentage = if let Some(active_state) = &state.active_state {
            active_state.start_value
        } else {
            self.value
        } as f32
            / self.max_value as f32;
        let thumb_size = bounds.size.height;
        let thumb_x = (bounds.size.width - thumb_size) * percentage;

        let drag_delta = state
            .active_state
            .as_ref()
            .map(|active_state| {
                active_state.current_drag_coordinate - active_state.start_drag_coordinate
            })
            .unwrap_or_default();

        let fill_size = px((thumb_x.0 + thumb_size.0 + drag_delta)
            .clamp(thumb_size.0.min(bounds.size.width.0), bounds.size.width.0));

        if !state.thumb_inset.is_done() {
            window.request_animation_frame();
        }

        SliderPrepaintState {
            thumb_full_size: Size {
                width: thumb_size,
                height: thumb_size,
            },
            thumb_bounds: Bounds {
                origin: Point {
                    x: bounds.origin.x
                        + px((thumb_x.0 + drag_delta)
                            .clamp(0., 0_f32.max(bounds.size.width.0 - thumb_size.0))),
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

        if let Some(on_change_handler) = self.on_change.as_ref() {
            let thumb_full_size = prepaint.thumb_full_size;
            let mouse_down_current_view = window.current_view();
            let mouse_up_current_view = window.current_view();
            let mouse_move_current_view = window.current_view();
            let mouse_down_handler = self.on_press.as_ref().cloned();
            let mouse_move_on_change_handler = on_change_handler.clone();
            let mouse_up_handler = self.on_release.as_ref().cloned();
            let current_value = self.value;
            let max_value = self.max_value;
            window.with_optional_element_state(id, |state, cx| {
                let state: Rc<RefCell<SliderInteractiveState>> = state
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

                        if let Some(mouse_down_handler) = mouse_down_handler.clone() {
                            mouse_down_handler.borrow_mut()(&SliderPressEvent, window, cx)
                        }

                        let mut state = mouse_down_state.borrow_mut();
                        state.active_state = Some(SliderInteractiveActiveState {
                            start_drag_coordinate: event.position.x.into(),
                            current_drag_coordinate: event.position.x.into(),
                            start_value: current_value,
                        });
                        state
                            .thumb_inset
                            .set_new_target(thumb_full_size.width.0 / 8.);
                        cx.notify(mouse_down_current_view);
                    });
                    cx.on_mouse_event(move |event: &MouseMoveEvent, _, window, cx| {
                        let mut state = mouse_move_state.borrow_mut();

                        let Some(active_state) = &mut state.active_state else {
                            return;
                        };

                        window.prevent_default();
                        cx.stop_propagation();

                        // Calculate the new value to be set
                        let pixels_moved = event.position.x.0 - active_state.start_drag_coordinate;
                        let total_pixels = bounds.size.width.0 - thumb_full_size.width.0;
                        let percentage_moved = pixels_moved / total_pixels;
                        let new_value = ((max_value as f32 * percentage_moved) as i64
                            + active_state.start_value as i64)
                            .clamp(0, max_value as i64)
                            as u32;

                        mouse_move_on_change_handler.borrow_mut()(
                            &SliderChangeEvent { new_value },
                            window,
                            cx,
                        );

                        active_state.current_drag_coordinate = event.position.x.into();
                        cx.notify(mouse_move_current_view);
                    });
                    cx.on_mouse_event(move |event: &MouseUpEvent, _, window, cx| {
                        let mut state = mouse_up_state.borrow_mut();

                        let Some(_active_state) = &mut state.active_state else {
                            return;
                        };

                        window.prevent_default();
                        cx.stop_propagation();

                        if let Some(mouse_up_handler) = mouse_up_handler.clone() {
                            mouse_up_handler.borrow_mut()(&SliderReleaseEvent, window, cx)
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
}
