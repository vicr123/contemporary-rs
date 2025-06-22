use crate::components::buttons::buttons;
use crate::components::checkboxes_radio_buttons::CheckboxesRadioButtons;
use crate::components::text_input::TextInput;
use crate::surface_list::SurfaceList;
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::pager::pager;
use contemporary::styling::theme::Theme;
use contemporary::window::ContemporaryWindow;
use contemporary_i18n::tr;
use gpui::prelude::FluentBuilder;
use gpui::{
    div, px, uniform_list, App, AppContext, Context, Entity, InteractiveElement,
    IntoElement, ParentElement, Render, StatefulInteractiveElement, Styled, WeakEntity, Window,
};

pub struct ComponentsRoot {
    pub window: WeakEntity<ContemporaryWindow<SurfaceList>>,

    checkboxes_radio_buttons: Entity<CheckboxesRadioButtons>,
    text_input: Entity<TextInput>,

    current_page: usize,
}

impl ComponentsRoot {
    pub fn new(
        cx: &mut App,
        window: WeakEntity<ContemporaryWindow<SurfaceList>>,
    ) -> Entity<ComponentsRoot> {
        cx.new(|cx| ComponentsRoot {
            window,
            checkboxes_radio_buttons: CheckboxesRadioButtons::new(cx),
            text_input: TextInput::new(cx),
            current_page: 0,
        })
    }
}

impl Render for ComponentsRoot {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let window = self.window.clone();
        div()
            .id("components")
            .flex()
            .w_full()
            .h_full()
            .gap(px(2.))
            .child(
                layer("sidebar-layer")
                    .w(px(300.))
                    .flex()
                    .flex_col()
                    .child(
                        grandstand("sidebar-grandstand")
                            .text(tr!("COMPONENTS_TITLE", "Components"))
                            .pt(px(36.)),
                    )
                    .child(
                        div().flex_grow().p(px(2.)).child(
                            uniform_list(
                                "sidebar-items",
                                3,
                                cx.processor(|this, range, _, cx| {
                                    let theme = cx.global::<Theme>();
                                    let mut items = Vec::new();
                                    for ix in range {
                                        let item = ix + 1;

                                        items.push(
                                            div()
                                                .id(ix)
                                                .p(px(2.))
                                                .rounded(theme.border_radius)
                                                .on_click(cx.listener(move |this, _, _, cx| {
                                                    this.current_page = ix;
                                                    cx.notify()
                                                }))
                                                .child(match ix {
                                                    0 => tr!("BUTTONS_TITLE"),
                                                    1 => tr!("CHECKBOXES_RADIO_BUTTONS_TITLE"),
                                                    2 => tr!("TEXT_INPUT_TITLE"),
                                                    _ => format!("Item {item}"),
                                                })
                                                .when(this.current_page == ix, |div| {
                                                    div.bg(theme.button_background)
                                                }),
                                        );
                                    }
                                    items
                                }),
                            )
                            .h_full()
                            .w_full(),
                        ),
                    ),
            )
            .child(
                pager("main-area", self.current_page)
                    .flex_grow()
                    .page(buttons().into_any_element())
                    .page(self.checkboxes_radio_buttons.clone().into_any_element())
                    .page(self.text_input.clone().into_any_element()),
            )
    }
}
