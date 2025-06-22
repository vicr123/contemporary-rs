use crate::styling::theme::Theme;
use gpui::prelude::FluentBuilder;
use gpui::MouseButton::Left;
use gpui::{
    div, px, App, Div, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, Stateful, StatefulInteractiveElement, Styled, Window,
};

#[derive(PartialEq, Copy, Clone)]
pub enum CheckState {
    Off,
    On,
    Indeterminate,
}

#[derive(Copy, Clone)]
pub struct CheckedChangeEvent {
    pub check_state: CheckState,
}

#[derive(IntoElement)]
pub struct Checkbox {
    div: Stateful<Div>,
    check_state: CheckState,
    label: Option<SharedString>,
    draw_as_radio: bool,

    checked_changed_handlers: Vec<Box<dyn Fn(&CheckedChangeEvent, &mut Window, &mut App)>>,
}

pub fn checkbox(id: impl Into<ElementId>) -> Checkbox {
    Checkbox {
        div: div().id(id),
        check_state: CheckState::Off,
        label: None,
        draw_as_radio: false,
        checked_changed_handlers: Vec::new(),
    }
}

pub fn radio_button(id: impl Into<ElementId>) -> Checkbox {
    Checkbox {
        div: div().id(id),
        check_state: CheckState::Off,
        label: None,
        draw_as_radio: true,
        checked_changed_handlers: Vec::new(),
    }
}

impl Checkbox {
    pub fn checked(mut self) -> Self {
        self.check_state = CheckState::On;
        self
    }

    pub fn indeterminate(mut self) -> Self {
        self.check_state = CheckState::Indeterminate;
        self
    }

    pub fn check_state(mut self, state: CheckState) -> Self {
        self.check_state = state;
        self
    }

    pub fn label(mut self, label: SharedString) -> Self {
        self.label = Some(label);
        self
    }

    pub fn on_checked_changed(
        mut self,
        listener: impl Fn(&CheckedChangeEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.checked_changed_handlers
            .push(Box::new(move |event, window, cx| {
                listener(event, window, cx)
            }));
        self
    }
}

impl RenderOnce for Checkbox {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let checked_changed_handlers = self.checked_changed_handlers;

        self.div
            .flex()
            .gap(px(6.))
            .child(
                div()
                    .when_else(
                        self.draw_as_radio,
                        |div| div.rounded(px(300.)),
                        |div| div.rounded(theme.border_radius),
                    )
                    .border(px(1.))
                    .border_color(theme.foreground)
                    .min_w(px(12.))
                    .min_h(px(12.))
                    .when(self.check_state == CheckState::On, |div| {
                        div.bg(theme.foreground)
                    }),
            )
            .when_some(self.label, |div, label| div.child(label))
            .on_click(move |event, window, cx| {
                if event.down.button != Left {
                    return;
                }

                let event = CheckedChangeEvent {
                    check_state: match self.check_state {
                        CheckState::Indeterminate => CheckState::On,
                        CheckState::On => CheckState::Off,
                        CheckState::Off => CheckState::On,
                    },
                };

                for fun in checked_changed_handlers.iter() {
                    fun(&event, window, cx)
                }
            })
    }
}
