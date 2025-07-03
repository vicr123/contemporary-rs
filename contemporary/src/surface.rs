use crate::components::button::button;
use crate::styling::theme::Theme;
use gpui::{
    div, img, px, svg, AnyElement, App, AppContext, Context,
    Entity, InteractiveElement, IntoElement, MouseButton, ParentElement, Render, RenderOnce, Styled, Window, WindowControlArea,
};

#[derive(IntoElement)]
pub struct Surface {
    child: AnyElement,
}

pub fn surface() -> Surface {
    Surface {
        child: div().into_any_element(),
    }
}

impl Surface {
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = child.into_any_element();
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
            .child(window_controls())
    }
}

#[derive(IntoElement)]
struct WindowTitle;

fn window_controls() -> WindowTitle {
    WindowTitle
}

#[allow(unreachable_code)]
impl RenderOnce for WindowTitle {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        #[cfg(target_os = "macos")]
        {
            return div().id("contemporary-window-title");
        }

        let theme = _cx.global::<Theme>();

        div()
            .id("contemporary-window-title")
            .absolute()
            .top(px(0.))
            .left(px(0.))
            .w_full()
            .h(px(40.))
            .flex()
            .child(if cfg!(target_os = "macos") {
                div()
            } else {
                div()
                    .child(
                        button("window-menu")
                            .flat()
                            .w(px(40.))
                            .h(px(40.))
                            .child(img("contemporary-icon:/application").w(px(24.)).h(px(24.))),
                    )
                    .occlude()
            })
            .child(div().flex_grow())
            .window_control_area(WindowControlArea::Drag)
            .child(
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
            .on_mouse_down(MouseButton::Left, move |_, window, _| {
                window.start_window_move()
            })
    }
}
