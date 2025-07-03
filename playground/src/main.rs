use std::rc::Rc;

use crate::main_window::MainWindow;
use contemporary::application::new_contemporary_application;
use contemporary::{
    application::{ApplicationLink, Details, License, Versions},
    setup::{setup_contemporary, Contemporary, ContemporaryMenus},
    window::contemporary_window_options,
};
use contemporary_i18n::{tr_load, I18N_MANAGER};
use contemporary_icon_tool_macros::application_icon;
use gpui::{px, size, App, Bounds, Menu, WindowBounds, WindowOptions};
use indexmap::IndexMap;

mod components;
mod main_window;

fn main() {
    application_icon!("../dist/baseicon.svg");
    new_contemporary_application().run(|cx: &mut App| {
        I18N_MANAGER.write().unwrap().load_source(tr_load!());
        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);

        let default_window_options = contemporary_window_options(cx);
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
                            application_name: "Contemporary Playground",
                            application_generic_name: "Control Playground",
                            desktop_entry: "com.vicr123.contemporary-playground",
                            copyright_holder: "me :)",
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
                                name: "File".into(),
                                items: vec![],
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
