use crate::window::window_globals::WindowGlobals;
use gpui::{App, BorrowAppContext, Empty, IntoElement, ParentElement, RenderOnce, Window, div};

#[derive(IntoElement)]
pub(crate) struct RaisedDraw;

impl RenderOnce for RaisedDraw {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let next_element = cx.update_global::<WindowGlobals, _>(|window_globals, cx| {
            window_globals
                .globals_for(window, cx)
                .update(cx, |globals, _| globals.pending_raised_draws.pop_front())
        });

        match next_element {
            None => Empty.into_any_element(),
            Some(next_element) => div()
                .child(next_element((), window, cx))
                .child(RaisedDraw)
                .into_any_element(),
        }
    }
}
