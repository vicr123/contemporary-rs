// On Windows do NOT show a console window when opening the app
#![cfg_attr(all(not(test), target_os = "windows"), windows_subsystem = "windows")]

use std::rc::Rc;

use crate::actions::{DarkTheme, LightTheme, SystemTheme, register_actions};
use crate::main_window::MainWindow;
use contemporary::application::new_contemporary_application;
use contemporary::macros::application_details;
use contemporary::setup::ShowAll;
use contemporary::styling::theme::ThemeType::{Dark, Light, System};
use contemporary::styling::theme::{Theme, ThemeType};
use contemporary::{
    application::{ApplicationLink, Details, License, Versions},
    setup::{Contemporary, ContemporaryMenus, setup_contemporary},
    window::contemporary_window_options,
};
use contemporary_i18n::{I18N_MANAGER, tr, tr_load};
use contemporary_icon_tool_macros::application_icon;
use gpui::{App, Bounds, Menu, MenuItem, WindowBounds, WindowOptions, actions, px, size};
use indexmap::IndexMap;
use smol_macros::main;

mod actions;
mod components;
mod main_surface;
mod main_window;
mod patterns;

fn mane() {
    application_icon!("../dist/baseicon.svg");
    new_contemporary_application().run(|cx: &mut App| {
        I18N_MANAGER.write().unwrap().load_source(tr_load!());
        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);

        let default_window_options = contemporary_window_options(cx);
        register_actions(cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..default_window_options
            },
            |_, cx| {
                let window = MainWindow::new(cx);
                let weak_window = window.downgrade();

                setup_contemporary(
                    cx,
                    Contemporary {
                        details: Details {
                            generatable: application_details!(),
                            copyright_holder: "Victor Tran",
                            copyright_year: "2025",
                            application_version: "3.0",
                            license: License::Gpl3OrLater,
                            links: IndexMap::from([
                                (
                                    ApplicationLink::HelpContents,
                                    "https://github.com/vicr123/contemporary-rs/",
                                ),
                                (
                                    ApplicationLink::FileBug,
                                    "https://github.com/vicr123/contemporary-rs/issues",
                                ),
                                (
                                    ApplicationLink::SourceCode,
                                    "https://github.com/vicr123/contemporary-rs",
                                ),
                            ]),
                        },
                        menus: ContemporaryMenus {
                            menus: vec![Menu {
                                name: tr!("MENU_THEME", "Theme").into(),
                                items: vec![
                                    MenuItem::action(tr!("THEME_SYSTEM", "System"), SystemTheme),
                                    MenuItem::action(tr!("THEME_LIGHT", "Light"), LightTheme),
                                    MenuItem::action(tr!("THEME_DARK", "Dark"), DarkTheme),
                                ],
                            }],
                            on_about: Rc::new(move |cx| {
                                weak_window.upgrade().unwrap().update(cx, |window, cx| {
                                    window.about_surface_open(true);
                                    cx.notify()
                                })
                            }),
                        },
                    },
                );

                let versions = cx.global::<Versions>();
                versions
                    .versions
                    .lock()
                    .unwrap()
                    .insert("version thing".into(), "1.0".into());

                window
            },
        )
        .unwrap();
        cx.activate(true);
    });
}

main! {
    async fn main() {
        mane()
    }
}
