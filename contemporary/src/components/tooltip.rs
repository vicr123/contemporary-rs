use crate::styling::theme::ThemeStorage;
use gpui::{
    AnyElement, AnyView, App, AppContext, Context, IntoElement, ParentElement, Render,
    SharedString, Styled, Window, div, px,
};

pub struct Tooltip {
    build_child: Box<dyn Fn((), &mut Window, &mut App) -> AnyElement>,
}

impl Tooltip {
    pub fn new(
        build_child: impl Fn((), &mut Window, &mut App) -> AnyElement + 'static,
        _: &mut Context<Self>,
    ) -> Self {
        Self {
            build_child: Box::new(build_child),
        }
    }
}

impl Render for Tooltip {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .p(px(2.))
            .bg(theme.background)
            .text_color(theme.foreground)
            .rounded(theme.border_radius)
            .border(px(1.))
            .border_color(theme.border_color)
            .font_family(theme.system_font_family.clone())
            .text_size(theme.system_font_size)
            .child((self.build_child)((), window, cx))
    }
}

pub fn simple_tooltip(text: impl Into<SharedString>) -> impl Fn(&mut Window, &mut App) -> AnyView {
    let text = text.into();
    move |_, cx| {
        cx.new({
            let text = text.clone();
            move |cx| {
                Tooltip::new(
                    {
                        let text = text.clone();
                        move |_, _, _| div().child(text.clone()).into_any_element()
                    },
                    cx,
                )
            }
        })
        .into()
    }
}
