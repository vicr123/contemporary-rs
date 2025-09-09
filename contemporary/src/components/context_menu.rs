use crate::components::icon_text::icon_text;
use crate::components::layer::layer;
use crate::styling::theme::{Theme, VariableColor};
use gpui::prelude::FluentBuilder;
use gpui::{
    App, InteractiveElement, IntoElement, KeyBinding, MouseButton, ParentElement, Pixels, Point,
    RenderOnce, StatefulInteractiveElement, Styled, Window, actions, anchored, deferred, div, px,
};
use std::rc::Rc;

actions!(context_menu, [Escape]);

#[derive(IntoElement)]
pub struct ContextMenu {
    items: Vec<ContextMenuItem>,
}

pub struct OpenContextMenu {
    open_position: Point<Pixels>,
}

impl RenderOnce for ContextMenu {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context_menu_state = window.use_state(cx, |_, _| None);
        let context_menu_state_2 = context_menu_state.clone();
        let context_menu_state_3 = context_menu_state.clone();
        let context_menu_state_4 = context_menu_state.clone();

        let context_menu_open = context_menu_state.read(cx);
        if context_menu_open.is_some() {
            cx.on_action(move |_: &Escape, cx| {
                context_menu_state_4.write(cx, None);
            })
        }

        // Reborrow
        let context_menu_open = context_menu_state.read(cx);

        let window_size = window.viewport_size();
        let inset = window.client_inset().unwrap_or_else(|| px(0.));

        let theme = cx.global::<Theme>();

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
            .when_some(context_menu_open.as_ref(), move |david, context_menu| {
                david.child(deferred(
                    anchored().position(Point::new(px(0.), px(0.))).child(
                        div()
                            .top_0()
                            .left_0()
                            .w(window_size.width - inset - inset)
                            .h(window_size.height - inset - inset)
                            .m(inset)
                            .occlude()
                            .on_any_mouse_down(move |_, _, cx| {
                                context_menu_state_2.write(cx, None);
                            })
                            .child(
                                anchored().position(context_menu.open_position).child(
                                    self.items.iter().enumerate().fold(
                                        div()
                                            .border_color(theme.border_color)
                                            .border(px(1.))
                                            .bg(theme.background)
                                            .rounded(theme.border_radius)
                                            .min_w(px(100.))
                                            .occlude()
                                            .flex()
                                            .flex_col(),
                                        move |david, (i, item)| match item {
                                            ContextMenuItem::Separator => {
                                                david.child(div().h(px(1.)).bg(theme.border_color))
                                            }
                                            ContextMenuItem::Group(title) => david.child(
                                                layer()
                                                    .p(px(4.))
                                                    .text_center()
                                                    .child(title.clone()),
                                            ),
                                            ContextMenuItem::MenuItem {
                                                label,
                                                action,
                                                icon,
                                                remain_open,
                                            } => {
                                                let action = action.clone();
                                                let remain_open_local_clone = remain_open.clone();
                                                let context_menu_state_local_clone =
                                                    context_menu_state_3.clone();
                                                david.child(
                                                    div()
                                                        .id(i)
                                                        .p(px(4.))
                                                        .when_some(icon.as_ref(), |div, icon| {
                                                            div.child(icon_text(
                                                                icon.clone().into(),
                                                                label.clone().into(),
                                                            ))
                                                        })
                                                        .when_none(icon, |david| {
                                                            david.child(
                                                                div()
                                                                    .flex()
                                                                    .items_center()
                                                                    .gap(px(6.))
                                                                    .child(div().size(px(16.)))
                                                                    .child(label.clone()),
                                                            )
                                                        })
                                                        .hover(|div| {
                                                            div.bg(theme.background.hover())
                                                        })
                                                        .active(|div| {
                                                            div.bg(theme.background.active())
                                                        })
                                                        .on_click(move |_, window, cx| {
                                                            action(
                                                                &ContextMenuActionEvent {},
                                                                window,
                                                                cx,
                                                            );

                                                            if !remain_open_local_clone {
                                                                context_menu_state_local_clone
                                                                    .write(cx, None);
                                                            }
                                                        }),
                                                )
                                            }
                                        },
                                    ),
                                ),
                            ),
                    ),
                ))
            })
    }
}

pub struct ContextMenuActionEvent {}
pub type ContextMenuActionHandler =
    dyn Fn(&ContextMenuActionEvent, &mut Window, &mut App) + 'static;

pub enum ContextMenuItem {
    Separator,
    Group(String),
    MenuItem {
        label: String,
        icon: Option<String>,
        action: Rc<Box<ContextMenuActionHandler>>,
        remain_open: bool,
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

pub struct ContextMenuMenuItemBuilder {
    label: Option<String>,
    icon: Option<String>,
    action: Option<Box<ContextMenuActionHandler>>,
    remain_open: bool,
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

    pub fn build(self) -> ContextMenuItem {
        ContextMenuItem::MenuItem {
            label: self.label.unwrap_or_default(),
            icon: self.icon,
            action: Rc::new(self.action.unwrap_or_else(|| Box::new(|_, _, _| {}))),
            remain_open: self.remain_open,
        }
    }
}

/// An extension trait for adding a context menu to Elements
pub trait ContextMenuExt<E> {
    fn with_context_menu(self, items: impl IntoIterator<Item = ContextMenuItem>) -> E
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
        })
    }
}

pub fn bind_context_menu_keys(cx: &mut App) {
    cx.bind_keys([KeyBinding::new("escape", Escape, None)])
}
