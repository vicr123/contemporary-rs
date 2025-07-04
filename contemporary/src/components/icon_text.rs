use crate::components::icon::icon;
use gpui::{
    div, px, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window,
};

#[derive(IntoElement)]
pub struct IconText {
    icon: SharedString,
    text: SharedString,
    icon_size: f32,
}

pub fn icon_text(icon: SharedString, text: SharedString) -> IconText {
    IconText {
        icon,
        text,
        icon_size: 16.,
    }
}

impl IconText {
    pub fn icon_size(mut self, icon_size: f32) -> IconText {
        self.icon_size = icon_size;
        self
    }
}

impl RenderOnce for IconText {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(6.))
            .child(icon(self.icon).size(self.icon_size))
            .child(self.text)
    }
}
