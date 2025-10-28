pub(crate) mod context_menu_popup;

use crate::components::context_menu::context_menu_popup::ContextMenuPopup;
use crate::components::raised::raised;
use crate::styling::theme::VariableColor;
use gpui::prelude::FluentBuilder;
use gpui::{
    App, InteractiveElement, IntoElement, KeyBinding, MouseButton, ParentElement, Pixels, Point,
    RenderOnce, StatefulInteractiveElement, Styled, Window, actions, div,
};
use std::rc::Rc;

actions!(context_menu, [Escape]);

#[derive(IntoElement)]
pub struct ContextMenu {
    items: Vec<ContextMenuItem>,
    as_deferred: bool,
}

impl ContextMenu {
    pub fn render_as_deferred(mut self, as_deferred: bool) -> Self {
        self.as_deferred = as_deferred;
        self
    }
}

#[derive(Clone)]
pub struct OpenContextMenu {
    pub(crate) open_position: Point<Pixels>,
}

impl RenderOnce for ContextMenu {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context_menu_state = window.use_state(cx, |_, _| None);
        let context_menu_state_2 = context_menu_state.clone();

        let items = self.items;

        div()
            .id("__context_menu_handler")
            .absolute()
            .top_0()
            .left_0()
            .w_full()
            .h_full()
            .on_mouse_down(MouseButton::Right, move |mouse_event, _, cx| {
                cx.stop_propagation();

                context_menu_state.write(
                    cx,
                    Some(OpenContextMenu {
                        open_position: mouse_event.position,
                    }),
                );
            })
            .child(
                raised(move |_, _, cx| {
                    // Context Menu Popup
                    let context_menu_open = context_menu_state_2.read(cx);
                    ContextMenuPopup {
                        items,
                        open_position: context_menu_open
                            .as_ref()
                            .map(|context_menu| context_menu.open_position),
                        request_close_listener: Rc::new(Box::new(move |_, _, cx| {
                            context_menu_state_2.write(cx, None);
                        })),
                    }
                    .into_any_element()
                })
                .render_as_deferred(self.as_deferred),
            )
    }
}

pub struct ContextMenuActionEvent {}
pub type ContextMenuActionHandler =
    dyn Fn(&ContextMenuActionEvent, &mut Window, &mut App) + 'static;

#[derive(Clone)]
pub enum ContextMenuItem {
    Separator,
    Group(String),
    MenuItem {
        label: String,
        icon: Option<String>,
        action: Rc<Box<ContextMenuActionHandler>>,
        remain_open: bool,
        disabled: bool,
    },
}

impl ContextMenuItem {
    pub fn separator() -> ContextMenuSeparatorBuilder {
        ContextMenuSeparatorBuilder { label: None }
    }

    pub fn menu_item() -> ContextMenuMenuItemBuilder {
        ContextMenuMenuItemBuilder {
            label: None,
            icon: None,
            action: None,
            remain_open: false,
            disabled: false,
        }
    }
}

pub struct ContextMenuSeparatorBuilder {
    label: Option<String>,
}

impl ContextMenuSeparatorBuilder {
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn build(self) -> ContextMenuItem {
        if let Some(label) = self.label {
            ContextMenuItem::Group(label)
        } else {
            ContextMenuItem::Separator
        }
    }
}

impl FluentBuilder for ContextMenuSeparatorBuilder {}

pub struct ContextMenuMenuItemBuilder {
    label: Option<String>,
    icon: Option<String>,
    action: Option<Box<ContextMenuActionHandler>>,
    remain_open: bool,
    disabled: bool,
}

impl ContextMenuMenuItemBuilder {
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn on_triggered(
        mut self,
        action: impl Fn(&ContextMenuActionEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.action = Some(Box::new(action));
        self
    }

    pub fn remain_open(mut self) -> Self {
        self.remain_open = true;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn build(self) -> ContextMenuItem {
        ContextMenuItem::MenuItem {
            label: self.label.unwrap_or_default(),
            icon: self.icon,
            action: Rc::new(self.action.unwrap_or_else(|| Box::new(|_, _, _| {}))),
            remain_open: self.remain_open,
            disabled: self.disabled,
        }
    }
}

impl FluentBuilder for ContextMenuMenuItemBuilder {}

/// An extension trait for adding a context menu to Elements
pub trait ContextMenuExt<E> {
    fn with_context_menu(self, items: impl IntoIterator<Item = ContextMenuItem>) -> E
    where
        Self: Sized;

    fn with_deferred_context_menu(self, items: impl IntoIterator<Item = ContextMenuItem>) -> E
    where
        Self: Sized;
}

impl<E> ContextMenuExt<E> for E
where
    E: ParentElement,
{
    fn with_context_menu(self, items: impl IntoIterator<Item = ContextMenuItem>) -> E
    where
        Self: Sized,
    {
        self.child(ContextMenu {
            items: items.into_iter().collect(),
            as_deferred: false,
        })
    }

    fn with_deferred_context_menu(self, items: impl IntoIterator<Item = ContextMenuItem>) -> E
    where
        Self: Sized,
    {
        self.child(ContextMenu {
            items: items.into_iter().collect(),
            as_deferred: true,
        })
    }
}

pub fn bind_context_menu_keys(cx: &mut App) {
    cx.bind_keys([KeyBinding::new("escape", Escape, None)])
}
