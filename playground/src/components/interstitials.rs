use cntp_i18n::tr;
use contemporary::components::button::button;
use contemporary::components::icon_text::icon_text;
use contemporary::components::interstitial::interstitial;
use contemporary::styling::theme::Theme;
use gpui::{App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window};

pub struct Interstitials {}

impl Interstitials {
    pub fn new(cx: &mut App) -> Entity<Interstitials> {
        cx.new(|_| Interstitials {})
    }
}

impl Render for Interstitials {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        interstitial()
            .w_full()
            .h_full()
            .bg(theme.background)
            .icon("sim-card-none".into())
            .title(tr!("INTERSTITIAL_TITLE", "This is an interstitial!").into())
            .message(
                tr!(
                    "INTERSTITIAL_MESSAGE",
                    "Interstitials can be used to welcome the user, or to surface an error."
                )
                .into(),
            )
            .child(button("interstitial-button").child(icon_text(
                "sim-card".into(),
                tr!("SIM_CARD_SETTINGS", "SIM Card Settings").into(),
            )))
    }
}
