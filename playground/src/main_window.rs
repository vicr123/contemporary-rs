use crate::main_surface::MainSurface;
use cntp_i18n::tr;
use contemporary::about_surface::about_surface;
use contemporary::components::dialog_box::{StandardButton, dialog_box};
use contemporary::window::contemporary_window;
use gpui::prelude::FluentBuilder;
use gpui::{App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Window};

pub struct MainWindow {
    main_surface: Entity<MainSurface>,
    is_about_surface_open: bool,
    is_settings_surface_open: bool,
}

impl MainWindow {
    pub fn new(cx: &mut App) -> Entity<MainWindow> {
        cx.new(|cx| MainWindow {
            main_surface: MainSurface::new(cx),
            is_about_surface_open: false,
            is_settings_surface_open: false,
        })
    }

    pub fn about_surface_open(&mut self, is_open: bool) -> &Self {
        self.is_about_surface_open = is_open;
        self
    }

    pub fn settings_surface_open(&mut self, is_open: bool) -> &Self {
        self.is_settings_surface_open = is_open;
        self
    }
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        contemporary_window()
            .child(self.main_surface.clone())
            .when(self.is_settings_surface_open, |w| {
                w.child(
                    dialog_box("settings-surface")
                        .title(tr!("SETTINGS", "Settings").into())
                        .content(tr!(
                            "SETTINGS_DESCRIPTION",
                            "The Settings surface would ordinarily display now."
                        ))
                        .standard_button(
                            StandardButton::Ok,
                            cx.listener(|this, _, _, cx| {
                                this.is_settings_surface_open = false;
                                cx.notify();
                            }),
                        ),
                )
            })
            .when(self.is_about_surface_open, |w| {
                w.child(about_surface().on_back_click(cx.listener(|this, _, _, cx| {
                    this.is_about_surface_open = false;
                    cx.notify();
                })))
            })
    }
}
