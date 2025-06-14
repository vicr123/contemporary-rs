use crate::application::{Details, Versions};
use crate::constrainer::constrainer;
use crate::grandstand::grandstand;
use crate::layer::layer;
use crate::window::{ContemporaryWindow, PushPop};
use gpui::{
    App, AppContext, Context, Entity, FontWeight, IntoElement, ParentElement, Render, Styled,
    WeakEntity, Window, div, px,
};

pub struct AboutSurface<T>
where
    T: Render,
{
    window: WeakEntity<ContemporaryWindow<T>>,
}

impl<T> AboutSurface<T>
where
    T: Render,
{
    pub fn new(cx: &mut App, window: WeakEntity<ContemporaryWindow<T>>) -> Entity<Self> {
        cx.new(|_| AboutSurface { window })
    }
}

impl<T> Render for AboutSurface<T>
where
    T: Render,
{
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let window = self.window.clone();
        let details = cx.global::<Details>();
        let versions = cx.global::<Versions>();

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .child(
                grandstand("about-grandstand")
                    .text(format!("About {}", details.application_name))
                    .on_click(move |_, _, cx| {
                        window.upgrade().unwrap().pop(cx);
                    })
                    .pt(px(36.)),
            )
            .child(
                constrainer("about-constrainer")
                    .flex()
                    .flex_col()
                    .gap(px(6.))
                    .child(
                        div()
                            .pt(px(64.))
                            .pb(px(64.))
                            .child(div().text_size(px(35.)).child(details.application_name))
                            .child(div().child(details.application_generic_name)),
                    )
                    .child(
                        layer("software-layer").child(
                            div()
                                .p(px(4.))
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .font_weight(FontWeight::BLACK)
                                        .child("Software".to_uppercase()),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .justify_between()
                                        .child(details.application_name)
                                        .child(details.application_version),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .justify_between()
                                        .child("Contemporary")
                                        .child(versions.contemporary_version),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .justify_between()
                                        .child("Platform")
                                        .child(std::env::consts::OS),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .justify_between()
                                        .child("Architecture")
                                        .child(std::env::consts::ARCH),
                                ),
                        ),
                    )
                    .child(
                        layer("copyright-layer").child(
                            div()
                                .p(px(4.))
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .font_weight(FontWeight::BOLD)
                                        .child("Copyright".to_uppercase()),
                                )
                                .child(format!(
                                    "Copyright (c) {} {}",
                                    details.copyright_holder, details.copyright_year
                                )),
                        ),
                    ),
            )
    }
}
