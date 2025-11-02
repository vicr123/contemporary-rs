use crate::actions::{DarkTheme, LightTheme, SystemTheme};
use crate::components::root::ComponentsRoot;
use crate::main_surface::MainSurfaceTab::{Components, Patterns};
use crate::patterns::root::PatternsRoot;
use cntp_i18n::tr;
use contemporary::components::application_menu::ApplicationMenu;
use contemporary::components::button::button;
use contemporary::components::pager::pager;
use contemporary::components::pager::slide_horizontal_animation::SlideHorizontalAnimation;
use contemporary::styling::theme::ThemeStorage;
use contemporary::surface::surface;
use gpui::{
    App, AppContext, Context, Entity, InteractiveElement, IntoElement, Menu, MenuItem,
    ParentElement, Render, Styled, Window, div, px,
};

pub struct MainSurface {
    components_root: Entity<ComponentsRoot>,
    patterns_root: Entity<PatternsRoot>,

    application_menu: Entity<ApplicationMenu>,

    selected_tab: MainSurfaceTab,
}

#[derive(PartialEq)]
enum MainSurfaceTab {
    Components,
    Patterns,
}

impl MainSurfaceTab {
    fn index(&self) -> usize {
        match self {
            Components => 0,
            Patterns => 1,
        }
    }
}

impl MainSurface {
    pub fn new(cx: &mut App) -> Entity<MainSurface> {
        cx.new(|cx| MainSurface {
            components_root: ComponentsRoot::new(cx),
            patterns_root: PatternsRoot::new(cx),
            application_menu: ApplicationMenu::new(
                cx,
                Menu {
                    name: "Application Menu".into(),
                    items: vec![MenuItem::submenu(Menu {
                        name: tr!("MENU_THEME").into(),
                        items: vec![
                            MenuItem::action(tr!("THEME_SYSTEM"), SystemTheme),
                            MenuItem::action(tr!("THEME_LIGHT"), LightTheme),
                            MenuItem::action(tr!("THEME_DARK"), DarkTheme),
                        ],
                    })],
                },
            ),
            selected_tab: Components,
        })
    }
}

impl Render for MainSurface {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

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
            .child(
                pager("main-pager", self.selected_tab.index())
                    .w_full()
                    .h_full()
                    .animation(SlideHorizontalAnimation::new())
                    .page(self.components_root.clone().into_any_element())
                    .page(self.patterns_root.clone().into_any_element()),
            )
            .application_menu(self.application_menu.clone())
    }
}
