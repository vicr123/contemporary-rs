use crate::components::toast::Toast;
use gpui::{AnyElement, AnyWindowHandle, App, AppContext, Entity, Global, Window};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

pub(crate) struct WindowGlobals {
    globals_map: RefCell<HashMap<AnyWindowHandle, Entity<WindowGlobal>>>,
}

impl WindowGlobals {
    pub fn new() -> Self {
        Self {
            globals_map: RefCell::new(HashMap::new()),
        }
    }

    pub fn globals_for(&self, window: &Window, cx: &mut App) -> Entity<WindowGlobal> {
        self.globals_map
            .borrow_mut()
            .entry(window.window_handle())
            .or_insert_with(|| cx.new(|_| WindowGlobal::new()))
            .clone()
    }
}

impl Global for WindowGlobals {}

pub struct WindowGlobal {
    pub pending_raised_draws: VecDeque<Box<dyn FnOnce((), &mut Window, &mut App) -> AnyElement>>,
    pub pending_toasts: VecDeque<Toast>,
}

impl WindowGlobal {
    pub fn new() -> Self {
        Self {
            pending_raised_draws: VecDeque::new(),
            pending_toasts: VecDeque::new(),
        }
    }
}
