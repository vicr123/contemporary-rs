use crate::application::{ApplicationLink, Details, Versions};
use crate::components::text_field::bind_text_input_keys;
use crate::platform_support::setup_platform;
use crate::styling::theme::Theme;
use contemporary_i18n::{tr, tr_load, I18N_MANAGER};
use gpui::{actions, Action, App, Global, KeyBinding, Menu, MenuItem};
use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

actions!(contemporary, [Quit, HideSelf, HideOthers, ShowAll, About]);

#[derive(PartialEq, Clone, Default, Deserialize, JsonSchema, Action)]
struct OpenLink {
    #[serde(default)]
    link: String,
}

pub struct Contemporary {
    pub details: Details,
    pub menus: ContemporaryMenus,
}

pub struct ContemporaryMenus {
    pub menus: Vec<Menu>,
    pub on_about: Rc<dyn Fn(&mut App)>,
}

struct Callbacks {
    pub on_about: Rc<dyn Fn(&mut App)>,
}

impl Global for Callbacks {}

pub fn setup_contemporary(cx: &mut App, mut application: Contemporary) {
    // TODO: Set up event handlers for system theme changes
    bind_text_input_keys(cx);

    cx.on_action(quit);
    cx.on_action(hide_self);
    cx.on_action(hide_others);
    cx.on_action(show_all);
    cx.on_action(about);
    cx.on_action(open_link);
    cx.bind_keys([KeyBinding::new("cmd-h", HideSelf, None)]);
    cx.bind_keys([KeyBinding::new("cmd-alt-h", HideOthers, None)]);
    cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

    if let Some(link) = application
        .details
        .links
        .get(&ApplicationLink::HelpContents)
    {
        cx.bind_keys([KeyBinding::new(
            "F1",
            OpenLink {
                link: link.to_string(),
            },
            None,
        )])
    }

    I18N_MANAGER.write().unwrap().load_source(tr_load!());
    let mut menus = vec![Menu {
        name: application.details.application_name.into(),
        items: vec![
            MenuItem::action(
                tr!("APPLE_APP_MENU_ABOUT", "About {{application}}", application=application.details.application_name,
                    #description = "Please use the string that macOS uses for the About action in the application menu."
                ),
                About,
            ),
            MenuItem::separator(),
            MenuItem::Submenu(Menu {
                name: tr!("APPLE_APP_MENU_SERVICES", "Services",
                    #description = "Please use the string that macOS uses for the Services action in the application menu."
                ).into(),
                items: vec![],
            }),
            MenuItem::separator(),
            MenuItem::action(
                tr!("APPLE_APP_MENU_HIDE_SELF", "Hide {{application}}", application = application.details.application_name,
                    #description = "Please use the string that macOS uses for the Hide this application action in the application menu."),
                HideSelf,
            ),
            MenuItem::action(
                tr!("APPLE_APP_MENU_HIDE_OTHERS", "Hide Others",
                    #description = "Please use the string that macOS uses for the Hide Others action in the application menu."),
                HideOthers,
            ),
            MenuItem::action(
                tr!("APPLE_APP_MENU_SHOW_ALL", "Show All", 
                    #description = "Please use the string that macOS uses for the Show All action in the application menu."),
                ShowAll,
            ),
            MenuItem::separator(),
            MenuItem::action(
                tr!(
                    "APPLE_APP_MENU_QUIT",
                    "Quit {{application}}",
                    application = application.details.application_name,
                    #description = "Please use the string that macOS uses for the Quit action in the application menu."
                ),
                Quit,
            ),
        ],
    }];
    menus.append(&mut application.menus.menus);

    let help_menu_items = application
        .details
        .links
        .iter()
        .flat_map(|(key, url)| {
            if *key == ApplicationLink::HelpContents {
                [
                    Some(MenuItem::action(
                        tr!(
                            "MENU_HELP_CONTENTS",
                            "{{application}} Help",
                            application = application.details.application_name
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

    menus.push(Menu {
        name: tr!("MENU_HELP", "Help",
            #description = "Please use the string that macOS uses for the Help menu."
        )
        .into(),
        items: help_menu_items,
    });

    cx.set_menus(menus);

    cx.set_global(application.details);
    cx.set_global(Theme::default());
    cx.set_global(Callbacks {
        on_about: application.menus.on_about,
    });
    cx.set_global(Versions {
        contemporary_version: "alpha",
        versions: Arc::new(Mutex::new(HashMap::new())),
    });

    setup_platform(cx);
}

fn quit(_: &Quit, cx: &mut App) {
    cx.quit();
}

fn hide_self(_: &HideSelf, cx: &mut App) {
    cx.hide();
}

fn hide_others(_: &HideOthers, cx: &mut App) {
    cx.hide_other_apps();
}

fn show_all(_: &ShowAll, cx: &mut App) {
    cx.unhide_other_apps();
}

fn about(_: &About, cx: &mut App) {
    let callbacks = cx.global::<Callbacks>();
    callbacks.on_about.clone()(cx);
}

fn open_link(action: &OpenLink, cx: &mut App) {
    cx.open_url(&action.link);
}
