use crate::components::root::ComponentsRoot;
use contemporary::about_surface::AboutSurface;
use gpui::{Context, Entity, IntoElement, Render, Window};

pub enum SurfaceList {
    Components(Entity<ComponentsRoot>),
    About(Entity<AboutSurface<SurfaceList>>),
}

impl Render for SurfaceList {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        match self {
            SurfaceList::Components(components) => components.clone().into_any_element(),
            SurfaceList::About(about) => about.clone().into_any_element(),
        }
    }
}
