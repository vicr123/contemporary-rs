use crate::components::button::{Button, button};
use crate::components::icon_text::icon_text;
use crate::components::layer::layer;
use crate::components::scrim::{Scrim, scrim};
use crate::components::spinner::spinner;
use crate::platform_support::platform_settings::PlatformSettings;
use crate::styling::theme::{Theme, VariableColor};
use crate::transition::float_transition_element::TransitionExt;
use cntp_i18n::tr;
use gpui::prelude::FluentBuilder;
use gpui::{
    Animation, AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement,
    ParentElement, RenderOnce, SharedString, Styled, Window, div, px, relative,
};

#[derive(IntoElement)]
pub struct DialogBox {
    scrim: Scrim,
    id: ElementId,
    title: Option<SharedString>,
    content: AnyElement,
    buttons: Vec<Button>,
    visible: bool,
    processing: bool,
    as_deferred: bool,
}

pub enum StandardButton {
    Ok,
    Cancel,
    Yes,
    No,
    Sorry,
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
            StandardButton::Sorry => ("sorry-button", "dialog-ok", tr!("DIALOG_SORRY", "Sorry")),
        };

        button(id).child(icon_text(icon.into(), text.into()))
    }
}

pub fn dialog_box(id: impl Into<ElementId>) -> DialogBox {
    let id = id.into();
    DialogBox {
        scrim: scrim(id.clone()),
        id,
        title: None,
        content: div().into_any_element(),
        buttons: vec![],
        visible: false,
        processing: false,
        as_deferred: false,
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

    pub fn on_click_outside(
        mut self,
        fun: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
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

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn processing(mut self, processing: bool) -> Self {
        self.processing = processing;
        self
    }

    pub fn render_as_deferred(mut self, as_deferred: bool) -> Self {
        self.as_deferred = as_deferred;
        self
    }
}

impl RenderOnce for DialogBox {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let platform_settings = cx.global::<PlatformSettings>();

        let buttons_layer = self.buttons.into_iter().fold(
            layer().flex().p(px(9.)).gap(px(6.)),
            move |layer, button| layer.child(button.flex_grow()),
        );

        self.scrim
            .visible(self.visible)
            .render_as_deferred(self.as_deferred)
            .child(
                div()
                    .flex()
                    .w_full()
                    .h_full()
                    .items_center()
                    .justify_center()
                    .when(self.visible, |david| {
                        david.child(
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
                                        layer()
                                            .p(px(9.))
                                            .text_size(theme.heading_font_size)
                                            .child(title),
                                    )
                                })
                                .child(div().p(px(9.)).child(self.content))
                                .child(buttons_layer)
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .bg(theme.background)
                                        .absolute()
                                        .top(px(0.))
                                        .left(px(0.))
                                        .right(px(0.))
                                        .bottom(px(0.))
                                        .child(spinner())
                                        .with_transition(
                                            self.id,
                                            if self.processing { 1. } else { 0. },
                                            Animation::new(platform_settings.animation_duration),
                                            |div, opacity| {
                                                div.opacity(opacity).when_else(
                                                    opacity == 0.,
                                                    |div| div.invisible(),
                                                    |div| div.occlude(),
                                                )
                                            },
                                        ),
                                ),
                        )
                    }),
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
