use crate::components::admonition::AdmonitionSeverity;
use crate::window::window_globals::WindowGlobals;
use gpui::{App, BorrowAppContext, Window};

pub struct Toast {
    pub(crate) title: Option<String>,
    pub(crate) body: Option<String>,
    pub(crate) severity: AdmonitionSeverity,
}

impl Default for Toast {
    fn default() -> Self {
        Self::new()
    }
}

impl Toast {
    pub fn new() -> Self {
        Self {
            title: None,
            body: None,
            severity: AdmonitionSeverity::Info,
        }
    }

    pub fn title(mut self, title: &str) -> Toast {
        self.title = Some(title.to_string());
        self
    }

    pub fn body(mut self, body: &str) -> Toast {
        self.body = Some(body.to_string());
        self
    }

    pub fn severity(mut self, severity: AdmonitionSeverity) -> Toast {
        self.severity = severity;
        self
    }

    pub fn post(self, window: &mut Window, cx: &mut App) {
        cx.update_global::<WindowGlobals, _>(|window_globals, cx| {
            let globals = window_globals.globals_for(window, cx);
            globals.update(cx, |globals, cx| {
                globals.pending_toasts.push_back(self);
                cx.notify();
            });
        })
    }
}
