use crate::components::root::ComponentsRoot;
use crate::main_surface::MainSurfaceTab::{Components, Patterns};
use contemporary::components::button::button;
use contemporary::styling::theme::Theme;
use contemporary::surface::surface;
use contemporary_i18n::tr;
use gpui::prelude::FluentBuilder;
use gpui::{
    div, px, App, AppContext, Context, Entity, InteractiveElement, IntoElement,
    ParentElement, Render, Styled, Window,
};

pub struct MainSurface {
    components_root: Entity<ComponentsRoot>,

    selected_tab: MainSurfaceTab,
}

#[derive(PartialEq)]
enum MainSurfaceTab {
    Components,
    Patterns,
}

impl MainSurface {
    pub fn new(cx: &mut App) -> Entity<MainSurface> {
        cx.new(|cx| MainSurface {
            components_root: ComponentsRoot::new(cx),
            selected_tab: Components,
        })
    }
}

impl Render for MainSurface {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        surface()
            .actions(
                div().occlude().flex().content_stretch().child(
                    div()
                        .flex()
                        .id("action-bar")
                        .bg(theme.button_background)
                        .rounded(theme.border_radius)
                        .gap(px(2.))
                        .content_stretch()
                        .child(
                            button("components-button")
                                .child(tr!("COMPONENTS_BUTTON", "Components"))
                                .checked_when(self.selected_tab == Components)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.selected_tab = Components;
                                    cx.notify();
                                })),
                        )
                        .child(
                            button("patterns-button")
                                .child(tr!("PATTERNS_BUTTON", "Patterns"))
                                .checked_when(self.selected_tab == Patterns)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.selected_tab = Patterns;
                                    cx.notify();
                                })),
                        ),
                ),
            )
            .when(self.selected_tab == Components, |div| {
                div.child(self.components_root.clone())
            })
    }
}
