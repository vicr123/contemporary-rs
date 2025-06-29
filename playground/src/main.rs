use std::rc::Rc;

use crate::components::root::ComponentsRoot;
use crate::surface_list::SurfaceList;
use contemporary::application::new_contemporary_application;
use contemporary::{
    about_surface::AboutSurface,
    application::{ApplicationLink, Details, License, Versions},
    setup::{setup_contemporary, Contemporary, ContemporaryMenus},
    surface::Surface,
    window::{contemporary_window_options, ContemporaryWindow, PushPop},
};
use contemporary_i18n::{tr_load, I18N_MANAGER};
use contemporary_icon_tool_macros::application_icon;
use gpui::{px, size, App, AppContext, Bounds, Menu, WindowBounds, WindowOptions};
use indexmap::IndexMap;

mod components;
mod surface_list;

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
                let mut window = ContemporaryWindow::new(cx);
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
                                let about_surface = AboutSurface::new(cx, weak_window.clone());
                                let a_surface = cx.new(|_| SurfaceList::About(about_surface));
                                let sf = Surface::new(cx, a_surface);
                                weak_window.upgrade().unwrap().push(cx, sf);
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

                let window_contents = cx.new(|cx| SurfaceList::Components(ComponentsRoot::new(cx)));
                let surface = Surface::new(cx, window_contents);
                window.push(cx, surface);
                window
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
