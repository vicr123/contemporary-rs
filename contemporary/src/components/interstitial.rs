use crate::components::icon::icon;
use crate::styling::theme::{ThemeStorage, VariableColor};
use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, App, Div, IntoElement, ParentElement, Refineable, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px,
};

#[derive(IntoElement)]
pub struct Interstitial {
    icon: Option<SharedString>,
    icon_size: f32,
    title: Option<SharedString>,
    message: Option<SharedString>,
    children_div: Option<Div>,

    style: StyleRefinement,
}

pub fn interstitial() -> Interstitial {
    Interstitial {
        icon: None,
        icon_size: 128.,
        title: None,
        message: None,
        children_div: None,
        style: StyleRefinement::default(),
    }
}

impl Interstitial {
    pub fn icon(mut self, icon: SharedString) -> Self {
        self.icon = Some(icon);
        self
    }
    pub fn title(mut self, title: SharedString) -> Self {
        self.title = Some(title);
        self
    }
    pub fn message(mut self, message: SharedString) -> Self {
        self.message = Some(message);
        self
    }
    pub fn icon_size(mut self, icon_size: f32) -> Self {
        self.icon_size = icon_size;
        self
    }
}

impl RenderOnce for Interstitial {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let mut david = div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .text_center()
            .when_some(self.icon, |david, icon_name| {
                david.child(
                    div()
                        .child(
                            icon(icon_name)
                                .size(self.icon_size)
                                .foreground(theme.foreground.disabled()),
                        )
                        .pb(px(self.icon_size / 8.)),
                )
            })
            .when_some(self.title, |david, title| {
                david.child(div().child(title).text_size(theme.heading_font_size))
            })
            .when_some(self.message, |david, message| david.child(message))
            .when_some(self.children_div, |david, children_div| {
                david.child(children_div.pt(px(self.icon_size / 8.)))
            });

        david.style().refine(&self.style);

        david
    }
}

impl Styled for Interstitial {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Interstitial {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children_div
            .get_or_insert_with(|| div().flex())
            .extend(elements);
    }
}
