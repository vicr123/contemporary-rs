use crate::styling::theme::Theme;
use gpui::{
    AnyElement, App, Bounds, Div, IntoElement, ParentElement, RenderOnce, Styled, TitlebarOptions,
    Window, WindowBounds, WindowDecorations, WindowOptions, div, point, px, size,
};

#[derive(IntoElement)]
pub struct ContemporaryWindow {
    div: Div,
}

pub fn contemporary_window() -> ContemporaryWindow {
    ContemporaryWindow { div: div() }
}

impl ParentElement for ContemporaryWindow {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.div.extend(elements);
    }
}

impl RenderOnce for ContemporaryWindow {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        self.div
            .bg(theme.background)
            .text_color(theme.foreground)
            .text_size(theme.system_font_size)
            .w_full()
            .h_full()
            .font_family(theme.system_font_family)
            .flex()
            .flex_col()
    }
}

pub fn contemporary_window_options(cx: &mut App) -> WindowOptions {
    let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        titlebar: Some(TitlebarOptions {
            title: Some("Contemporary Playground".into()),
            appears_transparent: true,
            traffic_light_position: Some(point(px(10.0), px(10.0))),
        }),
        window_decorations: Some(WindowDecorations::Client),
        ..Default::default()
    }
}
