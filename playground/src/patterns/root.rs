use crate::patterns::dialog_boxes::DialogBoxes;
use crate::patterns::i18n::I18n;
use crate::patterns::popovers::Popovers;
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

pub struct PatternsRoot {
    dialog_boxes: Entity<DialogBoxes>,
    i18n: Entity<I18n>,
    popovers: Entity<Popovers>,

    current_page: usize,
}

impl PatternsRoot {
    pub fn new(cx: &mut App) -> Entity<PatternsRoot> {
        cx.new(|cx| PatternsRoot {
            dialog_boxes: DialogBoxes::new(cx),
            i18n: I18n::new(cx),
            popovers: Popovers::new(cx),
            current_page: 0,
        })
    }
}

impl Render for PatternsRoot {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("patterns")
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
                            .text(tr!("PATTERNS_TITLE", "Patterns"))
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
                                                    0 => tr!("DIALOG_BOXES_TITLE"),
                                                    1 => tr!("I18N_TITLE"),
                                                    2 => tr!("POPOVERS_TITLE"),
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
                    .page(self.dialog_boxes.clone().into_any_element())
                    .page(self.i18n.clone().into_any_element())
                    .page(self.popovers.clone().into_any_element()),
            )
    }
}
