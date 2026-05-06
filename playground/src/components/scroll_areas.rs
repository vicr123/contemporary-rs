use contemporary::components::scroll_area::{ScrollArea, scroll_area_cx};
use contemporary::styling::theme::ThemeStorage;
use gpui::{App, Context, IntoElement, Render, RenderImage, Styled, Window, div, img};
use image::{Frame, ImageReader};
use smallvec::smallvec;
use std::sync::Arc;

pub struct ScrollAreas {
    image: Arc<RenderImage>,
}

impl ScrollAreas {
    pub fn new(_: &mut Context<Self>) -> Self {
        let image = image::load_from_memory(include_bytes!("../images/exploration.jpeg")).unwrap();
        let frame = Frame::new(image.into_rgba8());
        let image = Arc::new(RenderImage::new(smallvec![frame]));
        Self { image }
    }
}

impl Render for ScrollAreas {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();
        scroll_area_cx("scroll-area", |this, _, _| img(this.image.clone()), cx)
            .bg(theme.background)
            .overflow_x_scroll()
            .size_full()
    }
}
