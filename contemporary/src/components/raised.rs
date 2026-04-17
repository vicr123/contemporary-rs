use crate::window::window_globals::WindowGlobals;
use gpui::{AnyElement, App, BorrowAppContext, Empty, IntoElement, RenderOnce, Window, deferred};

#[derive(IntoElement)]
pub struct Raised {
    raised_element: Box<dyn FnOnce((), &mut Window, &mut App) -> AnyElement>,
}

pub fn raised(element: impl FnOnce((), &mut Window, &mut App) -> AnyElement + 'static) -> Raised {
    Raised {
        raised_element: Box::new(element),
    }
}

impl RenderOnce for Raised {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        deferred((self.raised_element)((), window, cx)).into_any_element()
    }
}
