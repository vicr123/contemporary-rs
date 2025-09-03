use crate::components::buttons::Buttons;
use crate::components::checkboxes_radio_buttons::CheckboxesRadioButtons;
use crate::components::progress_bars::ProgressBars;
use crate::components::ranges::Ranges;
use crate::components::skeletons::Skeletons;
use crate::components::text_input::TextInput;
use cntp_i18n::tr;
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::pager::lift_animation::LiftAnimation;
use contemporary::components::pager::pager;
use contemporary::components::pager::pager_animation::PagerAnimationDirection;
use contemporary::styling::theme::Theme;
use gpui::prelude::FluentBuilder;
use gpui::{
    App, AppContext, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, Window, div, px, uniform_list,
};

pub struct ComponentsRoot {
    buttons: Entity<Buttons>,
    checkboxes_radio_buttons: Entity<CheckboxesRadioButtons>,
    text_input: Entity<TextInput>,
    progress_bars: Entity<ProgressBars>,
    ranges: Entity<Ranges>,
    skeletons: Entity<Skeletons>,

    current_page: usize,
}

impl ComponentsRoot {
    pub fn new(cx: &mut App) -> Entity<ComponentsRoot> {
        cx.new(|cx| ComponentsRoot {
            buttons: Buttons::new(cx),
            checkboxes_radio_buttons: CheckboxesRadioButtons::new(cx),
            text_input: TextInput::new(cx),
            progress_bars: ProgressBars::new(cx),
            ranges: Ranges::new(cx),
            skeletons: Skeletons::new(cx),
            current_page: 0,
        })
    }
}

impl Render for ComponentsRoot {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("components")
            .flex()
            .w_full()
            .h_full()
            .gap(px(2.))
            .child(
                layer()
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
                                6,
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
                                                    3 => tr!("PROGRESS_BARS_TITLE"),
                                                    4 => tr!("RANGES_TITLE"),
                                                    5 => tr!("SKELETONS_TITLE"),
                                                    _ => format!("Item {item}").into(),
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
                    .animation(LiftAnimation::new())
                    .animation_direction(PagerAnimationDirection::Forward)
                    .page(self.buttons.clone().into_any_element())
                    .page(self.checkboxes_radio_buttons.clone().into_any_element())
                    .page(self.text_input.clone().into_any_element())
                    .page(self.progress_bars.clone().into_any_element())
                    .page(self.ranges.clone().into_any_element())
                    .page(self.skeletons.clone().into_any_element()),
            )
    }
}
