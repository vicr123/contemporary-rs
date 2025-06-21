use crate::components::buttons::buttons;
use crate::surface_list::SurfaceList;
use contemporary::grandstand::grandstand;
use contemporary::layer::layer;
use contemporary::pager::pager;
use contemporary::window::ContemporaryWindow;
use contemporary_i18n::tr;
use gpui::{
    div, px, Context, InteractiveElement, IntoElement, ParentElement, Render, Styled,
    WeakEntity, Window,
};

pub struct ComponentsRoot {
    pub window: WeakEntity<ContemporaryWindow<SurfaceList>>,
    current_page: usize,
}

pub fn components_root(window: WeakEntity<ContemporaryWindow<SurfaceList>>) -> ComponentsRoot {
    ComponentsRoot {
        window,
        current_page: 0,
    }
}

impl Render for ComponentsRoot {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let window = self.window.clone();
        div()
            .id("components")
            .flex()
            .w_full()
            .h_full()
            .gap(px(2.))
            .child(
                layer("sidebar-layer")
                    .w(px(300.))
                    .flex()
                    .flex_col()
                    .child(
                        grandstand("sidebar-grandstand")
                            .text(tr!("COMPONENTS_TITLE", "Components"))
                            .pt(px(36.)),
                    )
                    .child(div().child("Sidebar options go here")),
            )
            .child(
                pager("main-area", self.current_page)
                    .flex_grow()
                    .page(buttons().into_any_element())
                    .page(
                        div()
                            .w_full()
                            .h_full()
                            .flex()
                            .flex_col()
                            .child(grandstand("content-grandstand").text("Content").pt(px(36.)))
                            .child(div().child("Content 2 goes here"))
                            .into_any_element(),
                    )
                    .page(
                        div()
                            .w_full()
                            .h_full()
                            .flex()
                            .flex_col()
                            .child(grandstand("content-grandstand").text("Content").pt(px(36.)))
                            .child(div().child("Content 3 goes here"))
                            .into_any_element(),
                    ),
            )
    }
}
