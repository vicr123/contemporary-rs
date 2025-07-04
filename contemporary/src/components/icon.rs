use crate::styling::theme::Theme;
use gpui::{
    div, px, svg, App, IntoElement, ParentElement, RenderOnce, Rgba, SharedString, Styled, Window,
};

#[derive(IntoElement)]
pub struct Icon {
    name: SharedString,
    size: f32,
    foreground: Option<Rgba>,
}

pub fn icon(name: SharedString) -> Icon {
    Icon {
        name,
        size: 16.,
        foreground: None,
    }
}

impl Icon {
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn foreground(mut self, color: Rgba) -> Self {
        self.foreground = Some(color);
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div().size(px(self.size)).child(
            svg()
                .text_color(self.foreground.unwrap_or_else(|| {
                    let theme = cx.global::<Theme>();
                    theme.foreground
                }))
                .path(format!(
                    "icon://contemporary/{}?size={}",
                    self.name, self.size
                ))
                .size(px(self.size)),
        )
    }
}
