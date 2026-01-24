use crate::components::context_menu::{ContextMenuActionEvent, ContextMenuItem, Escape};
use crate::components::icon_text::icon_text;
use crate::components::layer::layer;
use crate::styling::theme::{ThemeStorage, VariableColor};
use gpui::prelude::FluentBuilder;
use gpui::{
    App, InteractiveElement, IntoElement, ParentElement, Pixels, Point, RenderOnce,
    StatefulInteractiveElement, Styled, Window, anchored, div, px,
};
use std::rc::Rc;

pub struct ContextMenuRequestCloseEvent;
pub type ContextMenuRequestCloseListener =
    dyn Fn(&ContextMenuRequestCloseEvent, &mut Window, &mut App) + 'static;

#[derive(IntoElement)]
pub struct ContextMenuPopup {
    pub items: Vec<ContextMenuItem>,
    pub open_position: Option<Point<Pixels>>,
    pub request_close_listener: Rc<Box<ContextMenuRequestCloseListener>>,
}

impl RenderOnce for ContextMenuPopup {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let window_size = window.viewport_size();
        let inset = window.client_inset().unwrap_or_else(|| px(0.));

        let theme = cx.theme();

        let Some(open_position) = self.open_position else {
            return div().into_any_element();
        };

        let request_close_listener = self.request_close_listener.clone();
        let request_close_listener_2 = self.request_close_listener.clone();
        let request_close_listener_3 = self.request_close_listener.clone();

        anchored()
            .position(Point::new(px(0.), px(0.)))
            .child(
                div()
                    .on_action(move |_: &Escape, window, cx| {
                        request_close_listener(&ContextMenuRequestCloseEvent, window, cx);
                    })
                    .top_0()
                    .left_0()
                    .w(window_size.width - inset - inset)
                    .h(window_size.height - inset - inset)
                    .m(inset)
                    .occlude()
                    .on_any_mouse_down(move |_, window, cx| {
                        request_close_listener_2(&ContextMenuRequestCloseEvent, window, cx);
                    })
                    .child(
                        anchored().position(open_position).child(
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
                                        layer().p(px(4.)).text_center().child(title.clone()),
                                    ),
                                    ContextMenuItem::MenuItem {
                                        label,
                                        action,
                                        icon,
                                        remain_open,
                                        disabled,
                                    } => {
                                        let action = action.clone();
                                        let remain_open_local_clone = *remain_open;
                                        let request_close_listener =
                                            request_close_listener_3.clone();

                                        let palette = theme.clone().disable_when(*disabled);

                                        david.child(
                                            div()
                                                .text_color(palette.foreground)
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
                                                .when(!disabled, move |div| {
                                                    div.hover(|div| {
                                                        div.bg(palette.background.hover())
                                                    })
                                                    .active(|div| {
                                                        div.bg(palette.background.active())
                                                    })
                                                    .on_click(move |_, window, cx| {
                                                        action(
                                                            &ContextMenuActionEvent {},
                                                            window,
                                                            cx,
                                                        );

                                                        if !remain_open_local_clone {
                                                            request_close_listener(
                                                                &ContextMenuRequestCloseEvent,
                                                                window,
                                                                cx,
                                                            );
                                                        }
                                                    })
                                                }),
                                        )
                                    }
                                },
                            ),
                        ),
                    ),
            )
            .into_any_element()
    }
}
