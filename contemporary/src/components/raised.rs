use crate::window::window_globals::WindowGlobals;
use gpui::{AnyElement, App, BorrowAppContext, Empty, IntoElement, RenderOnce, Window, deferred};

#[derive(IntoElement)]
pub struct Raised {
    raised_element: Box<dyn FnOnce((), &mut Window, &mut App) -> AnyElement>,
    as_deferred: bool,
}

pub fn raised(element: impl FnOnce((), &mut Window, &mut App) -> AnyElement + 'static) -> Raised {
    Raised {
        raised_element: Box::new(element),
        as_deferred: false,
    }
}

impl Raised {
    pub fn render_as_deferred(mut self, as_deferred: bool) -> Self {
        self.as_deferred = as_deferred;
        self
    }
}

impl RenderOnce for Raised {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if self.as_deferred {
            deferred((self.raised_element)((), window, cx)).into_any_element()
        } else {
            window.with_global_id("raised".into(), |global_id, window| {
                cx.update_global::<WindowGlobals, _>(|window_globals, cx| {
                    window_globals
                        .globals_for(window, cx)
                        .update(cx, |globals, _| {
                            globals.pending_raised_draws.push_back(self.raised_element)
                        });
                });
            });

            Empty.into_any_element()
        }
    }
}
