use crate::components::application_menu::ApplicationMenu;
use crate::components::button::button;
use crate::styling::theme::Theme;
use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, App, AppContext, Context, Entity, InteractiveElement, IntoElement, MouseButton,
    ParentElement, Render, RenderOnce, Styled, Window, WindowControlArea, div, img, px, rgb, svg,
};

#[derive(IntoElement)]
pub struct Surface {
    child: AnyElement,
    actions: AnyElement,
    application_menu: Option<Entity<ApplicationMenu>>,
}

pub fn surface() -> Surface {
    Surface {
        child: div().into_any_element(),
        actions: div().into_any_element(),
        application_menu: None,
    }
}

impl Surface {
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = child.into_any_element();
        self
    }

    pub fn actions(mut self, actions: impl IntoElement) -> Self {
        self.actions = actions.into_any_element();
        self
    }

    pub fn application_menu(mut self, application_menu: Entity<ApplicationMenu>) -> Self {
        self.application_menu = Some(application_menu);
        self
    }
}

impl RenderOnce for Surface {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .absolute()
            .top(px(0.))
            .left(px(0.))
            .w_full()
            .h_full()
            .occlude()
            .bg(theme.background)
            .child(self.child)
            .child(window_controls(self.actions, self.application_menu))
    }
}

#[derive(IntoElement)]
struct WindowTitle {
    actions: AnyElement,
    application_menu: Option<Entity<ApplicationMenu>>,
}

fn window_controls(
    actions: AnyElement,
    application_menu: Option<Entity<ApplicationMenu>>,
) -> WindowTitle {
    WindowTitle {
        actions,
        application_menu,
    }
}

#[allow(unreachable_code)]
impl RenderOnce for WindowTitle {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .id("contemporary-window-title")
            .absolute()
            .top(px(0.))
            .left(px(0.))
            .w_full()
            .h(px(40.))
            .flex()
            .gap(px(6.))
            .child(if cfg!(target_os = "macos") {
                // Make space for the window controls
                div().w(px(80.))
            } else {
                div()
                    .child(
                        button("window-menu")
                            .flat()
                            .w(px(40.))
                            .h(px(40.))
                            .child(img("contemporary-icon:/application").w(px(24.)).h(px(24.)))
                            .when_some(self.application_menu, |this, application_menu| {
                                this.child(application_menu.clone())
                                    .on_click(move |_, _, cx| {
                                        application_menu.update(cx, |this, cx| {
                                            println!("Opening application menu");
                                            this.set_open(true);
                                            cx.notify();
                                        })
                                    })
                            }),
                    )
                    .occlude()
            })
            .child(
                div()
                    .flex()
                    .h(px(40.))
                    .flex_grow()
                    .content_stretch()
                    .child(self.actions),
            )
            .window_control_area(WindowControlArea::Drag)
            .when(!cfg!(target_os = "macos"), |david| {
                david.child(
                    div()
                        .flex()
                        .occlude()
                        .child(
                            button("window-minimise")
                                .flat()
                                .w(px(40.))
                                .h(px(40.))
                                .child(
                                    svg()
                                        .w(px(24.))
                                        .h(px(24.))
                                        .text_color(theme.foreground)
                                        .path("window-controls:/min"),
                                )
                                .on_click(move |_, window, _| window.minimize_window())
                                .window_control_area(WindowControlArea::Min),
                        )
                        .child(
                            button("window-maximise")
                                .flat()
                                .w(px(40.))
                                .h(px(40.))
                                .child(
                                    svg()
                                        .w(px(24.))
                                        .h(px(24.))
                                        .text_color(theme.foreground)
                                        .path(if window.is_maximized() {
                                            "window-controls:/res"
                                        } else {
                                            "window-controls:/max"
                                        }),
                                )
                                .on_click(move |_, window, _| window.zoom_window())
                                .window_control_area(WindowControlArea::Max),
                        )
                        .child(
                            button("window-close")
                                .flat()
                                .w(px(40.))
                                .h(px(40.))
                                .child(
                                    svg()
                                        .w(px(24.))
                                        .h(px(24.))
                                        .text_color(theme.foreground)
                                        .path("window-controls:/close"),
                                )
                                .on_click(move |_, _, cx| cx.quit())
                                .window_control_area(WindowControlArea::Close),
                        ),
                )
            })
            .on_mouse_down(MouseButton::Left, move |_, window, _| {
                window.start_window_move()
            })
    }
}
