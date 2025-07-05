use crate::components::button::{button, Button};
use crate::components::icon_text::icon_text;
use crate::components::layer::layer;
use crate::components::scrim::{scrim, Scrim};
use crate::styling::theme::{Theme, VariableColor};
use contemporary_i18n::tr;
use gpui::prelude::FluentBuilder;
use gpui::{div, px, relative, AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window};

#[derive(IntoElement)]
pub struct DialogBox {
    scrim: Scrim,
    title: Option<SharedString>,
    content: AnyElement,
    buttons: Vec<Button>,
}

pub enum StandardButton {
    Ok,
    Cancel,
    Yes,
    No,
}

impl StandardButton {
    pub fn button(&self) -> Button {
        let (id, icon, text) = match self {
            StandardButton::Ok => ("ok-button", "dialog-ok", tr!("DIALOG_OK", "OK")),
            StandardButton::Cancel => (
                "cancel-button",
                "dialog-cancel",
                tr!("DIALOG_CANCEL", "Cancel"),
            ),
            StandardButton::Yes => ("yes-button", "dialog-ok", tr!("DIALOG_YES", "Yes")),
            StandardButton::No => ("no-button", "dialog-cancel", tr!("DIALOG_NO", "No")),
        };

        button(id).child(icon_text(icon.into(), text.into()))
    }
}

pub fn dialog_box(id: impl Into<ElementId>) -> DialogBox {
    DialogBox {
        scrim: scrim(id),
        title: None,
        content: div().into_any_element(),
        buttons: vec![],
    }
}

impl DialogBox {
    pub fn title(mut self, title: SharedString) -> Self {
        self.title = Some(title);
        self
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = content.into_any_element();
        self
    }

    pub fn content_text_informational(
        mut self,
        text: SharedString,
        informational_text: SharedString,
    ) -> Self {
        self.content = DialogBoxContent {
            content: text,
            informational_content: informational_text,
        }
        .into_any_element();
        self
    }

    pub fn button(mut self, button: Button) -> Self {
        self.buttons.push(button);
        self
    }

    pub fn on_click_outside(mut self, fun: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.scrim = self.scrim.on_click(fun);
        self
    }

    pub fn standard_button(
        mut self,
        standard_button: StandardButton,
        callback: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.buttons
            .push(standard_button.button().on_click(callback));
        self
    }
}

impl RenderOnce for DialogBox {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let buttons_layer = self.buttons.into_iter().fold(
            layer("buttons-layer").flex().p(px(9.)).gap(px(6.)),
            move |layer, button| layer.child(button.flex_grow()),
        );

        self.scrim.child(
            div()
                .flex()
                .w_full()
                .h_full()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .bg(theme.background)
                        .border_color(theme.border_color)
                        .border(px(1.))
                        .rounded(theme.border_radius)
                        .max_w(relative(0.9))
                        .flex()
                        .flex_col()
                        .occlude()
                        .when_some(self.title, |david, title| {
                            david.child(
                                layer("title-layer")
                                    .p(px(9.))
                                    .text_size(theme.heading_font_size)
                                    .child(title),
                            )
                        })
                        .child(div().p(px(9.)).child(self.content))
                        .child(buttons_layer),
                ),
        )
    }
}

#[derive(IntoElement)]
struct DialogBoxContent {
    content: SharedString,
    informational_content: SharedString,
}

impl RenderOnce for DialogBoxContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .child(self.content)
            .child(
                div()
                    .text_color(theme.foreground.disabled())
                    .child(self.informational_content),
            )
    }
}
