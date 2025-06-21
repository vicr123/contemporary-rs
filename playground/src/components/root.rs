use crate::surface_list::SurfaceList;
use contemporary::grandstand::grandstand;
use contemporary::window::ContemporaryWindow;
use contemporary_i18n::tr;
use gpui::{
    div, px, Context, InteractiveElement, IntoElement, ParentElement, Render, Styled,
    WeakEntity, Window,
};
use contemporary::layer::layer;

pub struct ComponentsRoot {
    pub window: WeakEntity<ContemporaryWindow<SurfaceList>>,
}

pub fn components_root(window: WeakEntity<ContemporaryWindow<SurfaceList>>) -> ComponentsRoot {
    ComponentsRoot { window }
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
                div()
                    .flex()
                    .flex_col()
                    .flex_grow()
                    .child(grandstand("content-grandstand").text("Content").pt(px(36.)))
                    .child(div().child("Content goes here")),
            )
    }
}
