use std::rc::Rc;

use contemporary::{
    about_surface::AboutSurface,
    application::{Details, License, Versions},
    setup::{Contemporary, ContemporaryMenus, setup_contemporary},
    surface::Surface,
    window::{ContemporaryWindow, PushPop, contemporary_window_options},
};
use gpui::{App, AppContext, Application, Bounds, WindowBounds, WindowOptions, px, size};

use crate::surface_list::{HelloWorld, SurfaceList};

mod surface_list;

fn main() {
    Application::new().run(|cx: &mut App| {
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
                let weak_widow = window.downgrade();

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
                        },
                        menus: ContemporaryMenus {
                            menus: vec![],
                            on_about: Rc::new(move |cx| {
                                let about_surface = AboutSurface::new(cx, weak_widow.clone());
                                let a_surface = cx.new(|_| SurfaceList::About(about_surface));
                                let sf = Surface::new(cx, a_surface);
                                weak_widow.upgrade().unwrap().push(cx, sf);
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

                let window_contents = cx.new(|cx| {
                    SurfaceList::HelloWorld(cx.new(|_| HelloWorld {
                        window: weak_window,
                    }))
                });
                let surface = Surface::new(cx, window_contents);
                window.push(cx, surface);
                window
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
