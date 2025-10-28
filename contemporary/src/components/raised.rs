use crate::window::window_globals::WindowGlobals;
use gpui::{AnyElement, App, BorrowAppContext, Empty, IntoElement, RenderOnce, Window};

#[derive(IntoElement)]
pub struct Raised {
    raised_element: AnyElement,
}

pub fn raised(element: impl IntoElement) -> Raised {
    Raised {
        raised_element: element.into_any_element(),
    }
}

impl RenderOnce for Raised {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        cx.update_global::<WindowGlobals, _>(|window_globals, cx| {
            window_globals
                .globals_for(window, cx)
                .update(cx, |globals, _| {
                    globals.pending_raised_draws.push_back(self.raised_element)
                });
        });

        Empty
    }
}
