use crate::components::context_menu::context_menu_popup::ContextMenuPopup;
use crate::components::context_menu::{ContextMenuItem, OpenContextMenu};
use crate::styling::theme::{Theme, VariableColor, variable_transparent};
use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, App, ClickEvent, Div, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, Rgba, Stateful, StatefulInteractiveElement, StyleRefinement, Styled, Window,
    deferred, div, px,
};
use std::rc::Rc;

#[derive(IntoElement)]
pub struct Button {
    div: Stateful<Div>,
    flat: bool,
    disabled: bool,
    checked: bool,
    destructive: bool,

    button_color: Option<Rgba>,
    button_text_color: Option<Rgba>,

    menu_items: Option<Vec<ContextMenuItem>>,

    on_click: Option<Rc<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>>,
}

pub fn button(id: impl Into<ElementId>) -> Button {
    Button {
        div: div()
            .id(id)
            .flex()
            .items_center()
            .justify_center()
            .p(px(6.0)),
        flat: false,
        disabled: false,
        checked: false,
        destructive: false,
        button_color: None,
        button_text_color: None,
        menu_items: None,
        on_click: None,
    }
}

impl Button {
    pub fn flat(mut self) -> Self {
        self.flat = true;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn checked(mut self) -> Self {
        self.checked = true;
        self
    }

    pub fn checked_when(self, condition: bool) -> Self {
        if condition { self.checked() } else { self }
    }

    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }

    pub fn button_color(mut self, color: impl Into<Rgba>) -> Self {
        self.button_color = Some(color.into());
        self
    }

    pub fn button_text_color(mut self, color: impl Into<Rgba>) -> Self {
        self.button_text_color = Some(color.into());
        self
    }

    pub fn on_click(mut self, fun: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(Box::new(fun)));
        self
    }

    pub fn with_menu(mut self, menu_items: Vec<ContextMenuItem>) -> Self {
        self.menu_items = Some(menu_items);
        self
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.div.extend(elements);
    }
}

impl InteractiveElement for Button {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.div.interactivity()
    }
}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context_menu_state = window.use_state(cx, |_, _| None);
        let context_menu_state_2 = context_menu_state.clone();

        let context_menu_open = context_menu_state.read(cx);

        let theme = cx.global::<Theme>().clone().disable_when(self.disabled);

        let background = if self.flat {
            variable_transparent()
        } else if self.destructive {
            theme.destructive_accent_color
        } else {
            self.button_color.unwrap_or(theme.button_background)
        };

        self.div
            .when_else(
                self.checked,
                |div| div.bg(background.active()),
                |div| div.bg(background),
            )
            .text_color(self.button_text_color.unwrap_or(theme.button_foreground))
            .rounded(theme.border_radius)
            .when(!self.disabled, |div| {
                div.when(!self.checked, |div| {
                    div.hover(|div| div.bg(background.hover()))
                })
                .active(|div| div.bg(background.active()))
            })
            .when(
                self.on_click.is_some() || self.menu_items.is_some(),
                |div| {
                    let on_click_handler = self.on_click;
                    let have_menu_items = self.menu_items.is_some();

                    div.on_click(move |event, window, cx| {
                        let disabled = self.disabled;
                        if disabled {
                            return;
                        }

                        if have_menu_items {
                            context_menu_state.write(
                                cx,
                                Some(OpenContextMenu {
                                    open_position: event.position(),
                                }),
                            );
                        } else if let Some(on_click_handler) = &on_click_handler {
                            on_click_handler(event, window, cx)
                        }
                    })
                },
            )
            .when_some(self.menu_items, |david, items| {
                david.child(deferred(
                    // Context Menu Popup
                    ContextMenuPopup {
                        items,
                        open_position: context_menu_open
                            .as_ref()
                            .map(|context_menu| context_menu.open_position),
                        request_close_listener: Rc::new(Box::new(move |_, _, cx| {
                            context_menu_state_2.write(cx, None);
                        })),
                    },
                ))
            })
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        self.div.style()
    }
}
