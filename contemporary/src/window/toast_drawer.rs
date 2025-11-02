use crate::components::admonition::AdmonitionSeverity;
use crate::components::anchorer::WithAnchorer;
use crate::components::button::button;
use crate::components::icon::icon;
use crate::components::raised::raised;
use crate::components::subtitle::subtitle;
use crate::components::toast::Toast;
use crate::easing::{ease_in_cubic, ease_out_cubic};
use crate::platform_support::platform_settings::PlatformSettings;
use crate::styling::theme::ThemeStorage;
use crate::window::window_globals::WindowGlobals;
use gpui::prelude::FluentBuilder;
use gpui::{
    AppContext, BorrowAppContext, Bounds, Context, Entity, InteractiveElement, IntoElement,
    ParentElement, Pixels, Point, Render, Styled, Window, anchored, div, px,
};
use std::time::Instant;

pub struct ToastDrawer {
    current_toast: Option<Toast>,
    toast_displayed_time: Instant,
    toast_animation_state: ToastAnimationState,
    last_bounds: Entity<Option<Bounds<Pixels>>>,
}

enum ToastAnimationState {
    Idle,
    In,
    Out,
}

impl ToastDrawer {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let window_globals = cx.update_global::<WindowGlobals, _>(|window_globals, cx| {
            window_globals.globals_for(window, cx)
        });
        let weak_this = cx.weak_entity();
        window
            .observe(&window_globals, cx, move |_, window, cx| {
                let weak_this = weak_this.clone();
                window.on_next_frame(move |window, cx| {
                    let _ = weak_this.update(cx, |this, cx| {
                        this.decide_if_next_toast_required(window, cx);
                    });
                })
            })
            .detach();

        Self {
            current_toast: None,
            toast_displayed_time: Instant::now(),
            toast_animation_state: ToastAnimationState::Idle,
            last_bounds: cx.new(|_| None),
        }
    }

    fn next_toast(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.current_toast = cx.update_global::<WindowGlobals, _>(|window_globals, cx| {
            let globals = window_globals.globals_for(window, cx);
            globals.update(cx, |globals, cx| globals.pending_toasts.pop_front())
        });
        self.toast_displayed_time = Instant::now();
        self.toast_animation_state = ToastAnimationState::In;
        self.last_bounds.write(cx, None);

        if self.current_toast.is_some() {
            // Kick off the timer to ensure that the toast disappears after a little while
            cx.on_next_frame(window, |this, window, cx| {
                this.decide_if_next_toast_required(window, cx);
            })
        }

        cx.notify();
    }

    fn decide_if_next_toast_required(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let platform_settings = cx.global::<PlatformSettings>();

        if self.current_toast.is_some() {
            match self.toast_animation_state {
                ToastAnimationState::Idle => {
                    if self.toast_displayed_time.elapsed().as_secs_f32() > 5. {
                        self.toast_animation_state = ToastAnimationState::Out;
                        self.toast_displayed_time = Instant::now();
                    }

                    // Check again next frame
                    cx.on_next_frame(window, |this, window, cx| {
                        this.decide_if_next_toast_required(window, cx);
                    })
                }
                ToastAnimationState::In => {
                    if self.toast_displayed_time.elapsed() > platform_settings.animation_duration {
                        self.toast_animation_state = ToastAnimationState::Idle;
                        self.toast_displayed_time = Instant::now();
                    }

                    // Check again next frame
                    cx.on_next_frame(window, |this, window, cx| {
                        this.decide_if_next_toast_required(window, cx);
                    })
                }
                ToastAnimationState::Out => {
                    if self.toast_displayed_time.elapsed() > platform_settings.animation_duration {
                        self.next_toast(window, cx);
                    } else {
                        // Check again next frame
                        cx.on_next_frame(window, |this, window, cx| {
                            this.decide_if_next_toast_required(window, cx);
                        })
                    }
                }
            }
        } else {
            // We're free to show a toast now
            self.next_toast(window, cx);
        }

        cx.notify()
    }
}

impl Render for ToastDrawer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        raised(cx.processor(|this, _, window, cx| {
            let platform_settings = cx.global::<PlatformSettings>();
            let theme = cx.theme();
            let window_size = window.viewport_size();
            let inset = window.client_inset().unwrap_or_else(|| px(0.));

            let last_bounds = this.last_bounds.read(cx);
            let displayed_width = last_bounds.unwrap_or_default().size.width + px(10.);
            let x_offset = match this.toast_animation_state {
                ToastAnimationState::Idle => px(0.),
                ToastAnimationState::In => {
                    -(displayed_width
                        - (ease_out_cubic(
                            this.toast_displayed_time.elapsed().as_secs_f32()
                                / platform_settings.animation_duration.as_secs_f32(),
                        ) * displayed_width))
                }
                ToastAnimationState::Out => {
                    ease_in_cubic(
                        this.toast_displayed_time.elapsed().as_secs_f32()
                            / platform_settings.animation_duration.as_secs_f32(),
                    ) * -displayed_width
                }
            };

            let last_bounds_entity = this.last_bounds.clone();

            anchored()
                .position(Point::new(px(0.), px(0.)))
                .child(
                    div()
                        .flex()
                        .top_0()
                        .left_0()
                        .w(window_size.width - inset - inset)
                        .h(window_size.height - inset - inset)
                        .m(inset)
                        .p(px(10.))
                        .items_end()
                        .child(div().flex().flex_col().items_end().when_some(
                            this.current_toast.as_ref(),
                            |david, toast| {
                                david.child(
                                    div()
                                        .bg(theme.background)
                                        .rounded(theme.border_radius)
                                        .left(x_offset)
                                        .when_none(last_bounds, |david| david.invisible())
                                        .child(
                                            div()
                                                .occlude()
                                                .p(px(4.))
                                                .border(px(1.))
                                                .border_color(theme.border_color)
                                                .rounded(theme.border_radius)
                                                .flex()
                                                .items_center()
                                                .gap(px(4.))
                                                .bg(match toast.severity {
                                                    AdmonitionSeverity::Info => {
                                                        theme.info_accent_color
                                                    }
                                                    AdmonitionSeverity::Warning => {
                                                        theme.warning_accent_color
                                                    }
                                                    AdmonitionSeverity::Error => {
                                                        theme.error_accent_color
                                                    }
                                                })
                                                .child(subtitle(
                                                    toast.title.clone().unwrap_or_default(),
                                                ))
                                                .child(toast.body.clone().unwrap_or_default())
                                                .child(
                                                    button("close-button")
                                                        .flat()
                                                        .child(icon("window-close".into()))
                                                        .on_click(cx.listener(|this, _, _, cx| {
                                                            // Immediately start the dismiss animation
                                                            this.toast_animation_state =
                                                                ToastAnimationState::Out;
                                                            this.toast_displayed_time =
                                                                Instant::now();
                                                            cx.notify();
                                                        })),
                                                ),
                                        )
                                        .with_anchorer(move |david, bounds, _, cx| {
                                            last_bounds_entity.write(cx, Some(bounds));
                                            david
                                        }),
                                )
                            },
                        )),
                )
                .into_any_element()
        }))
    }
}
