use crate::application::{ApplicationLink, Details, Versions};
use crate::components::button::button;
use crate::components::constrainer::constrainer;
use crate::components::grandstand::grandstand;
use crate::components::layer::layer;
use crate::components::subtitle::subtitle;
use crate::styling::theme::Theme;
use crate::window::{ContemporaryWindow, PushPop};
use contemporary_i18n::tr;
use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, RenderOnce, Styled,
    WeakEntity, Window, div, px,
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
                    .text(tr!(
                        "ABOUT_TITLE",
                        "About {{application}}",
                        application = details.application_name
                    ))
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
                            .child(
                                div()
                                    .flex()
                                    .gap(px(6.))
                                    .child(div().w(px(48.))) // TODO: Icon goes here
                                    .child(
                                        div().text_size(px(35.)).child(details.application_name),
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
            )
    }
}
