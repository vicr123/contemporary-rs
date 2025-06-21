use crate::button::button;
use crate::styling::theme::Theme;
use gpui::{
    App, AppContext, Context, Entity, InteractiveElement, IntoElement, MouseButton, ParentElement,
    Render, RenderOnce, Styled, Window, WindowControlArea, div, px, rgb,
};

pub struct Surface<T>
where
    T: Render,
{
    child: Entity<T>,
}

impl<T> Surface<T>
where
    T: Render,
{
    pub fn new(cx: &mut App, child: Entity<T>) -> Entity<Surface<T>> {
        cx.new(|_| Surface { child })
    }
}

impl<T> Render for Surface<T>
where
    T: Render,
{
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .absolute()
            .top(px(0.))
            .left(px(0.))
            .w_full()
            .h_full()
            .occlude()
            .bg(theme.background)
            .child(self.child.clone())
            .child(window_controls())
    }
}

#[derive(IntoElement)]
struct WindowTitle;

fn window_controls() -> WindowTitle {
    WindowTitle
}

impl RenderOnce for WindowTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        #[cfg(target_os = "macos")]
        {
            return div();
        }

        div()
            .id("contemporary-window-title")
            .absolute()
            .top(px(0.))
            .left(px(0.))
            .w_full()
            .h(px(40.))
            .flex()
            .child(div())
            .child(div().flex_grow())
            .window_control_area(WindowControlArea::Drag)
            .child(
                div()
                    .flex()
                    .occlude()
                    .child(
                        button("window-minimise")
                            .flat()
                            .w(px(36.))
                            .h(px(36.))
                            .child("Min")
                            .on_click(move |_, window, _| window.minimize_window())
                            .window_control_area(WindowControlArea::Min),
                    )
                    .child(
                        button("window-maximise")
                            .flat()
                            .w(px(36.))
                            .h(px(36.))
                            .child("Max")
                            .on_click(move |_, window, _| window.zoom_window())
                            .window_control_area(WindowControlArea::Max),
                    )
                    .child(
                        button("window-close")
                            .flat()
                            .w(px(36.))
                            .h(px(36.))
                            .child("X")
                            .on_click(move |_, _, cx| cx.quit())
                            .window_control_area(WindowControlArea::Close),
                    ),
            )
            .on_mouse_down(MouseButton::Left, move |_, window, _| {
                window.start_window_move()
            })
    }
}
