use cntp_i18n::tr;
use contemporary::components::constrainer::constrainer;
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::slider::{SliderChangeEvent, slider};
use contemporary::components::subtitle::subtitle;
use contemporary::styling::theme::Theme;
use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
};

pub struct Ranges {
    slider_value: u32,
}

impl Ranges {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Ranges { slider_value: 20 })
    }
}

impl Render for Ranges {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .bg(theme.background)
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(
                grandstand("ranges-grandstand")
                    .text(tr!("RANGES_TITLE", "Ranges"))
                    .pt(px(36.)),
            )
            .child(
                constrainer("ranges")
                    .flex()
                    .flex_col()
                    .w_full()
                    .p(px(8.))
                    .gap(px(8.))
                    .child(
                        layer()
                            .flex()
                            .flex_col()
                            .p(px(8.))
                            .w_full()
                            .child(subtitle(tr!(
                                "HORIZONTAL_SLIDERS_TITLE",
                                "Horizontal Sliders"
                            )))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(slider("slider").value(self.slider_value).on_change(
                                        cx.listener(
                                            |this, event: &SliderChangeEvent, &mut _, cx| {
                                                this.slider_value = event.new_value;
                                                cx.notify()
                                            },
                                        ),
                                    ))
                                    .child(
                                        slider("disabled-slider")
                                            .value(self.slider_value)
                                            .disabled(),
                                    )
                                    .child(tr!(
                                        "SLIDER_VALUE_TEXT",
                                        "Slider value: {{value}}",
                                        value = self.slider_value
                                    )),
                            ),
                    ),
            )
    }
}
