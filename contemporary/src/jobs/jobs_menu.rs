use crate::components::grandstand::grandstand;
use crate::components::scrim::scrim;
use crate::jobs::job_manager::JobManager;
use crate::styling::theme::Theme;
use cntp_i18n::tr;
use gpui::prelude::FluentBuilder;
use gpui::{
    App, InteractiveElement, IntoElement, ListAlignment, ListState, ParentElement, RenderOnce,
    Styled, Window, div, list, px,
};
use std::rc::Rc;

#[derive(IntoElement)]
pub struct JobsMenu {
    is_open: bool,
    set_open: Rc<dyn Fn(&bool, &mut Window, &mut App)>,
}

pub fn jobs_menu(
    is_open: bool,
    set_open: impl Fn(&bool, &mut Window, &mut App) + 'static,
) -> JobsMenu {
    JobsMenu {
        is_open,
        set_open: Rc::new(set_open),
    }
}

impl RenderOnce for JobsMenu {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let job_manager = cx.global::<JobManager>();

        let set_open = self.set_open.clone();

        let list_state = ListState::new(
            job_manager.job_len(),
            ListAlignment::Top,
            px(0.),
            |index, window, cx| {
                let job_manager = cx.global::<JobManager>();
                let job = job_manager.job(index).read(cx);
                job.borrow().element()
            },
        );

        scrim("jobs-menu")
            .visible(self.is_open)
            .on_click(move |_, window, cx| {
                set_open(&false, window, cx);
            })
            .when(self.is_open, |david| {
                david.child(
                    div()
                        .bg(theme.background)
                        .border_color(theme.border_color)
                        .border(px(1.))
                        .rounded(theme.border_radius)
                        .occlude()
                        .flex()
                        .flex_col()
                        .w(px(300.))
                        .h(px(500.))
                        .max_h(window.viewport_size().height)
                        .left(window.viewport_size().width - px(300.))
                        .child(grandstand("jobs-grandstand").text(tr!("JOBS_MENU_TITLE", "Jobs")))
                        .child(list(list_state).p(px(10.)).flex_grow().w_full()),
                )
            })
    }
}
