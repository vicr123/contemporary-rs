use crate::components::button::button;
use crate::jobs::job_manager::JobManager;
use crate::jobs::jobs_menu::jobs_menu;
use cntp_i18n::tr;
use gpui::{App, Context, Entity, IntoElement, ParentElement, Render, Window};

pub struct JobButton {
    is_first_update: bool,
}

impl JobButton {
    pub fn use_job_button(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let entity = window.use_state(cx, |window, cx| JobButton {
            is_first_update: true,
        });

        entity.update(cx, |job_button, cx| {
            if job_button.is_first_update {
                cx.observe_global::<JobManager>(|_, cx| cx.notify())
                    .detach();
                job_button.is_first_update = false;
            }
        });

        entity
    }
}

impl Render for JobButton {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_open = window.use_state(cx, |window, cx| false);
        let is_open_set = is_open.clone();

        let job_manager = cx.global::<JobManager>();

        button("jobs")
            .flat()
            .child(tr!("JOBS_BUTTON_TEXT", "Jobs"))
            .child(job_manager.job_len().to_string())
            .child(job_manager.aggregate_progress(cx).to_string())
            .child(jobs_menu(
                *is_open.read(cx),
                cx.listener(move |this, should_open, window, cx| {
                    is_open_set.write(cx, *should_open);
                    cx.notify()
                }),
            ))
            .on_click(cx.listener(move |this, _, window, cx| {
                is_open.write(cx, true);
                cx.notify()
            }))
    }
}
