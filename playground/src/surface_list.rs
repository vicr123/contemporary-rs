use contemporary::about_surface::AboutSurface;
use contemporary::button::button;
use contemporary::surface::Surface;
use contemporary::window::{ContemporaryWindow, PushPop};
use contemporary_i18n::trn;
use gpui::{
    div, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, WeakEntity,
    Window,
};

pub struct HelloWorld {
    pub window: WeakEntity<ContemporaryWindow<SurfaceList>>,
    pub count: isize,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let window = self.window.clone();
        div().flex().flex_col().child(
            button("x")
                .child(trn!(
                    "BUTTON",
                    "There is {{count}} stringling",
                    "There are {{count}} stringlings",
                    count = self.count
                ))
                .on_click(move |_, _, cx| {
                    let about_surface = AboutSurface::new(cx, window.clone());
                    let a_surface = cx.new(|_| SurfaceList::About(about_surface));
                    let sf = Surface::new(cx, a_surface);
                    window.upgrade().unwrap().push(cx, sf);
                }),
        )
    }
}

pub enum SurfaceList {
    HelloWorld(Entity<HelloWorld>),
    About(Entity<AboutSurface<SurfaceList>>),
}

impl Render for SurfaceList {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        match self {
            SurfaceList::HelloWorld(hello_world) => hello_world.clone().into_any_element(),
            SurfaceList::About(about) => about.clone().into_any_element(),
        }
    }
}
