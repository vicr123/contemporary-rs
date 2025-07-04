pub use crate::components::base::text_input::bind_text_input_keys;
use crate::components::base::text_input::TextInput;
use crate::components::focus_decoration::focus_decoration;
use crate::styling::theme::Theme;
use gpui::prelude::FluentBuilder;
use gpui::{
    div, px, App, AppContext, Context, ElementId, Entity, FocusHandle,
    Focusable, InteractiveElement, IntoElement, MouseUpEvent, ParentElement, Refineable,
    Render, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window,
};

pub struct TextField {
    id: ElementId,
    text_input: Entity<TextInput>,
    focus_handle: FocusHandle,
    style: StyleRefinement,
    borderless: bool,
}

impl TextField {
    pub fn new(
        cx: &mut App,
        id: impl Into<ElementId>,
        default_text: SharedString,
        placeholder: SharedString,
    ) -> Entity<Self> {
        cx.new(|cx| TextField {
            id: id.into(),
            text_input: TextInput::new(cx, default_text, placeholder),
            focus_handle: cx.focus_handle(),
            style: StyleRefinement::default(),
            borderless: false,
        })
    }

    pub fn disabled<C: AppContext>(&self, cx: &mut C, disabled: bool) -> &Self {
        self.text_input.update(cx, |text_input, cx| {
            text_input.disabled(disabled);
            cx.notify();
        });
        self
    }

    pub fn borderless(&mut self, borderless: bool) -> &Self {
        self.borderless = borderless;
        self
    }
}

impl Focusable for TextField {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl TextField {
    fn on_reset_click(&mut self, _: &MouseUpEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.text_input
            .update(cx, |text_input, _cx| text_input.reset());
        cx.notify();
    }
}

impl Render for TextField {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx
            .global::<Theme>()
            .disable_when(self.text_input.read(cx).is_disabled());

        let mut david = div()
            .id(self.id.clone())
            .when(!self.borderless, |div| div.bg(theme.layer_background))
            .overflow_x_scroll()
            .track_focus(&self.focus_handle(cx))
            .size_full()
            .relative()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .h_full()
                    .justify_center()
                    .rounded(theme.border_radius)
                    .when(!self.borderless, |div| {
                        div.border(px(1.)).border_color(theme.border_color)
                    })
                    .child(self.text_input.clone()),
            )
            .when(
                self.focus_handle.contains_focused(window, cx) && !self.borderless,
                |david| {
                    david.child(
                        focus_decoration()
                            .w_full()
                            .h_full()
                            .absolute()
                            .top_0()
                            .left_0(),
                    )
                },
            );
        david.style().refine(&self.style);
        david
    }
}

impl Styled for TextField {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
