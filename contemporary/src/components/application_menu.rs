use crate::application::{ApplicationLink, Details};
use crate::components::button::button;
use crate::components::icon::icon;
use crate::components::icon_text::icon_text;
use crate::components::scrim::scrim;
use crate::setup::{About, OpenLink};
use crate::styling::theme::{Theme, VariableColor};
use cntp_i18n::{i18n_manager, tr};
use gpui::prelude::FluentBuilder;
use gpui::{
    App, AppContext, ClickEvent, Context, Entity, InteractiveElement, IntoElement, Menu, MenuItem,
    OwnedMenu, OwnedMenuItem, ParentElement, Render, RenderOnce, SharedString, Styled, Window, div,
    img, px,
};
use std::rc::Rc;

pub struct ApplicationMenu {
    is_open: bool,
    root_menu: Rc<OwnedMenu>,
    menu_stack: Vec<Rc<OwnedMenu>>,
}

impl ApplicationMenu {
    pub fn new(cx: &mut App, menu: Menu) -> Entity<Self> {
        let root_menu = Rc::new(menu.owned());
        cx.new(|cx| ApplicationMenu {
            is_open: false,
            root_menu,
            menu_stack: vec![],
        })
    }

    pub fn set_open(&mut self, is_open: bool) {
        self.is_open = is_open;
        if !is_open {
            self.menu_stack.clear();
        }
    }
}

impl Render for ApplicationMenu {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_open {
            return div();
        }

        let theme = cx.global::<Theme>();
        let details = cx.global::<Details>();

        let locale = &i18n_manager!().locale;

        div().child(
            scrim("application-menu")
                .visible(self.is_open)
                .on_click(cx.listener(|this, _, _, cx| {
                    this.set_open(false);
                    cx.notify();
                }))
                .child(
                    div()
                        .bg(theme.background)
                        .border_color(theme.border_color)
                        .border(px(1.))
                        .rounded(theme.border_radius)
                        .occlude()
                        .flex()
                        .flex_col()
                        .w(px(300.))
                        .child(
                            div()
                                .flex()
                                .p(px(7.))
                                .gap(px(6.))
                                .child(img("contemporary-icon:/application").w(px(24.)).h(px(24.)))
                                .child(
                                    details
                                        .generatable
                                        .application_name
                                        .resolve_languages_or_default(&locale.messages),
                                ),
                        )
                        .when(!self.menu_stack.is_empty(), |div| {
                            div.child(
                                button("back-button")
                                    .child(icon_text(
                                        "go-previous".into(),
                                        tr!("MENU_GO_BACK", "Back").into(),
                                    ))
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.menu_stack.pop();
                                        cx.notify();
                                    })),
                            )
                        })
                        .child(
                            menu_list(
                                self.menu_stack
                                    .last()
                                    .map(|menu| menu.clone())
                                    .unwrap_or(self.root_menu.clone()),
                            )
                            .on_menu_click(cx.listener(|this, event: &SubmenuClickEvent, _, cx| {
                                this.menu_stack.push(event.menu.clone());
                                cx.notify();
                            }))
                            .on_menu_should_close(cx.listener(|this, _, _, cx| {
                                this.set_open(false);
                                cx.notify();
                            })),
                        )
                        .when(self.menu_stack.is_empty(), |david| {
                            david.child(
                                div()
                                    .flex()
                                    .justify_between()
                                    .p(px(8.))
                                    .child(
                                        button("menu-help-button")
                                            .flat()
                                            .child(icon("help-contents".into()))
                                            .size(px(32.))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                let details = cx.global::<Details>();

                                                let locale = &i18n_manager!().locale;

                                                let mut menu_items: Vec<MenuItem> = details
                                                    .links
                                                    .iter()
                                                    .flat_map(|(key, url)| {
                                                        if *key == ApplicationLink::HelpContents {
                                                            [
                                                                Some(MenuItem::action(
                                                                    tr!(
                                                                        "MENU_HELP_CONTENTS",
                                                                        application = details
                                                                            .generatable
                                                                            .application_name
                                .resolve_languages_or_default(&locale.messages)
                                                                    ),
                                                                    OpenLink {
                                                                        link: url.to_string(),
                                                                    },
                                                                )),
                                                                Some(MenuItem::separator()),
                                                            ]
                                                        } else {
                                                            [
                                                                Some(MenuItem::action(
                                                                    key.get_name(),
                                                                    OpenLink {
                                                                        link: url.to_string(),
                                                                    },
                                                                )),
                                                                None,
                                                            ]
                                                        }
                                                    })
                                                    .flatten()
                                                    .collect();

                                                if !menu_items.is_empty() {
                                                    menu_items.push(MenuItem::separator())
                                                }

                                                menu_items.push(MenuItem::action(
                                                    tr!(
                                                        "APPLE_APP_MENU_ABOUT",
                                                        application = details
                                                            .generatable
                                                            .application_name
                                                            .resolve_languages_or_default(
                                                                &locale.messages
                                                            )
                                                    ),
                                                    About,
                                                ));

                                                this.menu_stack.push(Rc::new(
                                                    Menu {
                                                        name: "Help".into(),
                                                        items: menu_items,
                                                    }
                                                    .owned(),
                                                ));
                                                cx.notify();
                                            })),
                                    )
                                    .child(
                                        button("menu-exit-button")
                                            .flat()
                                            .child(icon("application-exit".into()))
                                            .size(px(32.))
                                            .on_click(|_, _, cx| {
                                                cx.quit();
                                            }),
                                    ),
                            )
                        }),
                ),
        )
    }
}

#[derive(Clone)]
struct SubmenuClickEvent {
    pub menu: Rc<OwnedMenu>,
}
type SubmenuClickListener = Box<dyn Fn(&SubmenuClickEvent, &mut Window, &mut App) + 'static>;
type MenuShouldCloseListener = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
struct MenuList {
    menu: Rc<OwnedMenu>,
    menu_click_listeners: Vec<SubmenuClickListener>,
    menu_should_close_listeners: Vec<MenuShouldCloseListener>,
}

fn menu_list(menu: Rc<OwnedMenu>) -> MenuList {
    MenuList {
        menu,
        menu_click_listeners: vec![],
        menu_should_close_listeners: vec![],
    }
}

impl MenuList {
    pub fn on_menu_click<F: 'static>(mut self, listener: F) -> Self
    where
        F: Fn(&SubmenuClickEvent, &mut Window, &mut App),
    {
        self.menu_click_listeners.push(Box::new(listener));
        self
    }

    pub fn on_menu_should_close<F: 'static>(mut self, listener: F) -> Self
    where
        F: Fn(&ClickEvent, &mut Window, &mut App),
    {
        self.menu_should_close_listeners.push(Box::new(listener));
        self
    }
}

impl RenderOnce for MenuList {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let submenu_click_listeners = Rc::new(self.menu_click_listeners);
        let menu_should_close_listeners = Rc::new(self.menu_should_close_listeners);

        self.menu.items.iter().fold(div(), |david, item| {
            david.child(match item.clone() {
                OwnedMenuItem::Separator => {
                    div().h(px(1.)).bg(theme.border_color).into_any_element()
                }
                OwnedMenuItem::Submenu(menu) => {
                    let submenu_click_listeners = submenu_click_listeners.clone();
                    button(menu.name.clone())
                        .child(
                            div()
                                .w_full()
                                .flex()
                                .items_center()
                                .justify_between()
                                .text_ellipsis()
                                .overflow_hidden()
                                .child(menu.name.clone())
                                .child(icon("go-next".into())),
                        )
                        .flat()
                        .on_click(move |event, window, cx| {
                            let submenu_click_event = SubmenuClickEvent {
                                menu: Rc::new(menu.clone()),
                            };
                            for listener in submenu_click_listeners.iter() {
                                listener(&submenu_click_event, window, cx);
                            }
                        })
                        .into_any_element()
                }
                OwnedMenuItem::Action {
                    name,
                    action,
                    os_action: _os_action,
                } => {
                    let button_id: SharedString = format!("menu-item-{name}").into();
                    let menu_should_close_listeners = menu_should_close_listeners.clone();

                    let keybind = window
                        .bindings_for_action(action.as_ref())
                        .first()
                        .map(|key| key.keystrokes())
                        .map(|keystrokes| {
                            if keystrokes.len() == 1 {
                                let mut parts: Vec<String> = Vec::new();
                                let keystroke = keystrokes.first().unwrap();
                                if keystroke.modifiers.control {
                                    parts.push(tr!("KEY_CONTROL", "Ctrl", #description="Control key, as shown next to menu items").into());
                                }
                                if keystroke.modifiers.shift {
                                    parts.push(tr!("KEY_SHIFT", "Shift", #description="Shift key, as shown next to menu items").into());
                                }
                                if keystroke.modifiers.alt {
                                    parts.push(tr!("KEY_ALT", "Alt", #description="Alt key, as shown next to menu items").into());
                                }
                                if keystroke.modifiers.platform {
                                    parts.push(tr!("KEY_PLATFORM", "Super", #description="Super key, as shown next to menu items").into());
                                }
                                if keystroke.modifiers.function {
                                    parts.push(tr!("KEY_FUNCTION", "Fn", #description="Function key, as shown next to menu items").into());
                                }
                                parts.push(keystroke.key.to_uppercase().clone());
                                parts.join("+")
                            } else {
                                String::new()
                            }
                        })
                        .unwrap_or_default();

                    button(button_id)
                        .child(
                            div()
                                .w_full()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(div().text_ellipsis().overflow_hidden().child(name))
                                .child(div().text_color(theme.foreground.disabled()).child(keybind)))
                        .flat()
                        .on_click(move |event, window, cx| {
                            window.dispatch_action(action.boxed_clone(), cx);

                            for listener in menu_should_close_listeners.iter() {
                                listener(&event, window, cx);
                            }
                        })
                        .into_any_element()
                }
            })
        })
    }
}
