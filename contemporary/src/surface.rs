use crate::styling::theme::Theme;
use gpui::{
    App, AppContext, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render,
    Styled, Window, div, px, rgb,
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
    }
}
