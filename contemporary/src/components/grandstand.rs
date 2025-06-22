use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder, px, App, ClickEvent, Div, ElementId, IntoElement,
    ParentElement, RenderOnce, SharedString, StyleRefinement, Styled, Window,
};

use crate::components::button::button;
use crate::components::icon::icon;
use crate::components::layer::layer;
use crate::styling::theme::Theme;

#[derive(IntoElement)]
pub struct Grandstand {
    id: ElementId,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    text: SharedString,
    div: Div,
}

pub fn grandstand(id: impl Into<ElementId>) -> Grandstand {
    Grandstand {
        id: id.into(),
        on_click: None,
        text: "".into(),
        div: div(),
    }
}

impl Grandstand {
    pub fn on_click(mut self, fun: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(fun));
        self
    }

    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = text.into();
        self
    }
}

impl Styled for Grandstand {
    fn style(&mut self) -> &mut StyleRefinement {
        self.div.style()
    }
}

impl RenderOnce for Grandstand {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        layer(self.id).child(
            self.div.flex().flex_row().child(div().h(px(20.0))).child(
                div()
                    .flex()
                    .justify_center()
                    .gap(px(4.))
                    .when_some(self.on_click, move |div, on_click| {
                        div.child(
                            button("back-button")
                                .flat()
                                .child(icon("go-previous".into()))
                                .on_click(move |ev, window, cx| {
                                    (on_click)(ev, window, cx);
                                }),
                        )
                    })
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .text_size(theme.heading_font_size)
                            .child(self.text)
                            .pt(px(2.))
                            .m(px(4.)),
                    ),
            ),
        )
    }
}
