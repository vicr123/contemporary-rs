use crate::easing::ease_in_out_cubic;
use crate::styling::theme::ThemeStorage;
use gpui::{
    App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement, LayoutId,
    Path, PathBuilder, Pixels, Point, Refineable, Size, Style, StyleRefinement, Styled, Window,
    ease_in_out, px,
};
use std::panic::Location;
use std::time::Instant;

const SMALL_ARC: u128 = 15;
const LARGE_ARC: u128 = 325;
const ARC_ANIMATE_TIME: u128 = 500;
const ARC_ANIMATE_DELAY: u128 = 250;

pub struct Spinner {
    style_refinement: StyleRefinement,
}

pub fn spinner() -> Spinner {
    Spinner {
        style_refinement: StyleRefinement::default().size(px(32.)),
    }
}

impl Styled for Spinner {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style_refinement
    }
}

impl IntoElement for Spinner {
    type Element = Spinner;

    fn into_element(self) -> Self::Element {
        self
    }
}

pub struct SpinnerPrepaintState {
    arc_path: Path<Pixels>,
}

impl Element for Spinner {
    type RequestLayoutState = ();
    type PrepaintState = SpinnerPrepaintState;

    fn id(&self) -> Option<ElementId> {
        Some("spinner".into())
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
        _cx: &mut App,
    ) -> Self::PrepaintState {
        let size = bounds.size.width.min(bounds.size.height);
        let centered_bounds = Bounds::centered_at(
            bounds.center(),
            Size {
                width: size,
                height: size,
            },
        );

        window.request_animation_frame();

        let start = window.with_element_state(id.unwrap(), |old_state, _app| {
            let instant = old_state.unwrap_or_else(Instant::now);
            (instant, instant)
        });

        let angle_point = |angle: f32, radius: Pixels| -> Point<Pixels> {
            Point::new(
                centered_bounds.center().x + radius * angle.cos(),
                centered_bounds.center().y + radius * angle.sin(),
            )
        };

        let bounds_radius = centered_bounds.size.width / 2.;

        let elapsed = start.elapsed().as_millis();

        // Calculate the arc to draw
        let constant_rotation = start.elapsed().as_secs_f32() * 120.;

        let angle_start = constant_rotation + {
            // Every loop, we increase our arc length by 310 degrees
            let loops = elapsed / (ARC_ANIMATE_TIME * 2 + ARC_ANIMATE_DELAY);
            ((loops * (LARGE_ARC - SMALL_ARC)) % 360) as f32 + {
                let elapsed = elapsed % (ARC_ANIMATE_TIME * 2 + ARC_ANIMATE_DELAY);
                match elapsed {
                    x if x < ARC_ANIMATE_TIME => {
                        // Increase arc length to 325 degrees over 500 ms
                        0.
                    }
                    x if x < ARC_ANIMATE_TIME + ARC_ANIMATE_DELAY => {
                        // Wait 250 ms
                        0.
                    }
                    x if x < ARC_ANIMATE_TIME * 2 + ARC_ANIMATE_DELAY => {
                        // Decrease arc length to 15 degrees over 500 ms
                        (LARGE_ARC - SMALL_ARC) as f32
                            * ease_in_out_cubic(
                                (x - ARC_ANIMATE_TIME - ARC_ANIMATE_DELAY) as f32
                                    / ARC_ANIMATE_TIME as f32,
                            )
                    }
                    _ => unreachable!(),
                }
            }
        };
        let arc_length = {
            let elapsed = elapsed % (ARC_ANIMATE_TIME * 2 + ARC_ANIMATE_DELAY);
            match elapsed {
                x if x < ARC_ANIMATE_TIME => {
                    // Increase arc length to 325 degrees over 500 ms
                    SMALL_ARC as f32
                        + (LARGE_ARC - SMALL_ARC) as f32
                            * ease_in_out(x as f32 / ARC_ANIMATE_TIME as f32)
                }
                x if x < ARC_ANIMATE_TIME + ARC_ANIMATE_DELAY => {
                    // Wait 250 ms
                    LARGE_ARC as f32
                }
                x if x < ARC_ANIMATE_TIME * 2 + ARC_ANIMATE_DELAY => {
                    // Decrease arc length to 15 degrees over 500 ms
                    SMALL_ARC as f32
                        + (LARGE_ARC - SMALL_ARC) as f32
                            * (1.
                                - ease_in_out_cubic(
                                    (x - ARC_ANIMATE_TIME - ARC_ANIMATE_DELAY) as f32
                                        / ARC_ANIMATE_TIME as f32,
                                ))
                }
                _ => unreachable!(),
            }
        };

        let mut path = PathBuilder::stroke(bounds_radius * 0.2);
        path.move_to(angle_point(
            angle_start * std::f32::consts::PI / 180.,
            bounds_radius,
        ));
        path.arc_to(
            Point::new(bounds_radius, bounds_radius),
            px(arc_length * std::f32::consts::PI / 180.),
            arc_length > 180.,
            true,
            angle_point(
                (angle_start + arc_length) * std::f32::consts::PI / 180.,
                bounds_radius,
            ),
        );

        SpinnerPrepaintState {
            arc_path: path.build().unwrap(),
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let theme = cx.theme();
        window.paint_path(prepaint.arc_path.clone(), theme.foreground);
    }
}
