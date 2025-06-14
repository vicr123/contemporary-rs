use crate::styling::theme::Theme;
use crate::surface::Surface;
use gpui::{
    App, AppContext, Bounds, Context, Entity, IntoElement, ParentElement, Render, Styled,
    TitlebarOptions, Window, WindowBounds, WindowDecorations, WindowOptions, div, point, px, size,
};

pub trait PushPop<T>
where
    T: Render,
{
    fn push(&mut self, cx: &mut App, entity: Entity<Surface<T>>);
    fn pop(&mut self, cx: &mut App);
}

pub struct ContemporaryWindow<T>
where
    T: Render,
{
    surfaces: Vec<Entity<Surface<T>>>,
}

impl<T> ContemporaryWindow<T>
where
    T: Render,
{
    pub fn new(cx: &mut App) -> Entity<ContemporaryWindow<T>> {
        cx.new(|_| ContemporaryWindow { surfaces: vec![] })
    }
}

impl<T> PushPop<T> for Entity<ContemporaryWindow<T>>
where
    T: Render,
{
    fn push(&mut self, cx: &mut App, entity: Entity<Surface<T>>) {
        self.update(cx, |this, cx| {
            this.surfaces.push(entity);
            cx.notify()
        })
    }

    fn pop(&mut self, cx: &mut App) {
        self.update(cx, |this, cx| {
            let last_surface = this.surfaces.pop();
            if last_surface.is_none() {
                panic!("Tried to pop a surface with no children")
            }
            cx.notify()
        })
    }
}

impl<T> Render for ContemporaryWindow<T>
where
    T: Render,
{
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        self.surfaces.iter().fold(
            div()
                .bg(theme.background)
                .text_color(theme.foreground)
                .text_size(theme.system_font_size)
                .w_full()
                .h_full()
                .font_family(theme.system_font_family)
                .flex()
                .flex_col(),
            |div, surface| div.child(surface.clone()),
        )
    }
}

pub fn contemporary_window_options(cx: &mut App) -> WindowOptions {
    let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        titlebar: Some(TitlebarOptions {
            title: Some("Title".into()),
            appears_transparent: true,
            traffic_light_position: Some(point(px(10.0), px(10.0))),
        }),
        window_decorations: Some(WindowDecorations::Client),
        ..Default::default()
    }
}
