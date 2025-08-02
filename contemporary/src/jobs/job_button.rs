use crate::components::button::button;
use crate::jobs::job_manager::{JobButtonState, JobManager};
use crate::jobs::jobs_menu::jobs_menu;
use cntp_i18n::tr;
use gpui::prelude::FluentBuilder;
use gpui::{
    App, BorrowAppContext, Context, Entity, IntoElement, ParentElement, Render, Window, div,
};

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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let job_manager = cx.global::<JobManager>();

        let job_button_state = job_manager.job_button_state(cx);

        div()
            .child(jobs_menu())
            .when(job_button_state != JobButtonState::Hidden, |david| {
                david.child(
                    button("jobs")
                        .flat()
                        .child(tr!("JOBS_BUTTON_TEXT", "Jobs"))
                        // .child(job_manager.job_len(cx).to_string())
                        // .child(job_manager.aggregate_progress(cx).to_string())
                        .when(
                            job_button_state == JobButtonState::HaveRequiresAttention,
                            |button| button.child("RQA"),
                        )
                        .when(job_button_state == JobButtonState::HaveFailure, |button| {
                            button.child("F")
                        })
                        .when(job_button_state == JobButtonState::HaveSuccess, |button| {
                            button.child("OK")
                        })
                        .on_click(cx.listener(move |_, _, _, cx| {
                            cx.update_global::<JobManager, ()>(|manager, cx| {
                                manager.set_job_menu_open(true, cx);
                            });
                            cx.notify()
                        })),
                )
            })
    }
}
