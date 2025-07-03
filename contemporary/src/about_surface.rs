use crate::application::{ApplicationLink, Details, Versions};
use crate::components::button::button;
use crate::components::constrainer::constrainer;
use crate::components::grandstand::{Grandstand, grandstand};
use crate::components::layer::layer;
use crate::components::subtitle::subtitle;
use crate::styling::theme::Theme;
use crate::surface::surface;
use contemporary_i18n::tr;
use gpui::{
    App, AppContext, ClickEvent, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px,
};

#[derive(IntoElement)]
struct AboutSurfaceButtons;

impl RenderOnce for AboutSurfaceButtons {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let details = cx.global::<Details>();

        div()
            .flex()
            .bg(theme.button_background)
            .gap(px(2.))
            .rounded(theme.border_radius)
            .children(
                details
                    .links
                    .iter()
                    .filter(|link| *link.0 != ApplicationLink::HelpContents)
                    .enumerate()
                    .map(|(idx, link)| {
                        button(("help-link", idx))
                            .child(link.0.get_name())
                            .on_click(|_, _, cx| cx.open_url(link.1))
                            .into_any_element()
                    }),
            )
    }
}

#[derive(IntoElement)]
pub struct AboutSurface {
    grandstand: Grandstand,
}

pub fn about_surface() -> AboutSurface {
    AboutSurface {
        grandstand: grandstand("about-grandstand"),
    }
}

impl AboutSurface {
    pub fn on_back_click(
        mut self,
        fun: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.grandstand = self
            .grandstand
            .on_click(move |event, window, cx| fun(event, window, cx));
        self
    }
}

impl RenderOnce for AboutSurface {
    fn render(self, _window: &mut Window, cx: &mut gpui::App) -> impl IntoElement {
        let details = cx.global::<Details>();
        let versions = cx.global::<Versions>();

        surface().child(
            div()
                .flex()
                .flex_col()
                .w_full()
                .h_full()
                .child(
                    self.grandstand
                        .text(tr!(
                            "ABOUT_TITLE",
                            "About {{application}}",
                            application = details.application_name
                        ))
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
                                .child(
                                    div()
                                        .flex()
                                        .gap(px(6.))
                                        .child(div().w(px(48.))) // TODO: Icon goes here
                                        .child(
                                            div()
                                                .text_size(px(35.))
                                                .child(details.application_name),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .gap(px(6.))
                                        .child(div().w(px(48.)))
                                        .child(div().child(details.application_generic_name)),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .gap(px(6.))
                                        .child(div().w(px(48.)))
                                        .child(AboutSurfaceButtons),
                                ),
                        )
                        .child(
                            layer("software-layer").child(
                                div()
                                    .p(px(4.))
                                    .flex()
                                    .flex_col()
                                    .child(subtitle(tr!("ABOUT_SOFTWARE", "Software")))
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
                                            .child(tr!("ABOUT_CONTEMPORARY", "Contemporary"))
                                            .child(versions.contemporary_version),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .justify_between()
                                            .child(tr!("ABOUT_PLATFORM", "Platform"))
                                            .child(std::env::consts::OS),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .justify_between()
                                            .child(tr!("ABOUT_ARCH", "Architecture"))
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
                                    .child(subtitle(tr!("ABOUT_COPYRIGHT", "Copyright")))
                                    .child(format!(
                                        "Copyright © {} {}",
                                        details.copyright_holder, details.copyright_year
                                    )),
                            ),
                        ),
                ),
        )
    }
}
