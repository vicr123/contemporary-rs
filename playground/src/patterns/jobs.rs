use cntp_i18n::tr;
use contemporary::components::button::button;
use contemporary::components::constrainer::constrainer;
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::subtitle::subtitle;
use contemporary::jobs::job::JobStatus;
use contemporary::jobs::job_manager::{JobManager, Jobling};
use contemporary::jobs::standard_job::StandardJob;
use contemporary::styling::theme::Theme;
use gpui::{
    App, AppContext, AsyncApp, BorrowAppContext, Context, Entity, IntoElement, ParentElement,
    Render, Styled, Window, div, px,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

pub struct Jobs {}

impl Jobs {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Jobs {})
    }
}

impl Render for Jobs {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .bg(theme.background)
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(
                grandstand("jobs-grandstand")
                    .text(tr!("JOBS_TITLE", "Jobs"))
                    .pt(px(36.)),
            )
            .child(
                constrainer("jobs")
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
                            .child(subtitle(tr!("JOBS_TITLE")))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(tr!(
                                        "JOBS_DESCRIPTION",
                                        "Jobs represent long running processes that are currently running in the background. When a job is added, it shows in the Jobs pane in the top right corner of the window."
                                    ))
                                    .child(
                                        button("job-normal")
                                            .child(tr!("JOB_NORMAL_START", "Start Normal Job"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                let job = Rc::new(RefCell::new(StandardJob::new(tr!("STANDARD_JOB_TITLE", "Standard Job").into(), tr!("STANDARD_JOB_IN_PROGRESS_DESCRIPTION", "A description of the current task can go here.").into())));
                                                let job_entity_source = job.clone();
                                                let job_entity = cx.new::<Jobling>(|_| {
                                                    job_entity_source
                                                });

                                                let job_clone = job_entity.clone();
                                                cx.spawn(async move |_, cx: &mut AsyncApp| {
                                                    let instant = Instant::now();
                                                    while instant.elapsed().as_secs_f32() < 10. {
                                                        job_clone.update(cx, |_, cx| {
                                                            job.borrow_mut().update_job_progress(
                                                                (instant.elapsed().as_secs_f32() * 1000.) as u64,
                                                                10000,
                                                            );
                                                            cx.notify();
                                                        }).unwrap();
                                                        cx.background_executor().timer(Duration::from_millis(10)).await;
                                                    }
                                                    job_clone.update(cx, |_, cx| {
                                                        job.borrow_mut().update_job_status(
                                                            tr!("STANDARD_JOB_COMPLETE_DESCRIPTION", "This job is now complete.").into(),
                                                            JobStatus::Completed
                                                        );
                                                        cx.notify();
                                                    }).unwrap();
                                                }).detach();
                                                cx.update_global::<JobManager, ()>(|job_manager, cx| {
                                                    job_manager.add_job(job_entity, cx);
                                                });
                                            })),
                                    )
                                    .child(
                                        button("job-indeterminate")
                                            .child(tr!("JOB_INDETERMINATE_START", "Start Indeterminate Processing Job"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                let job = Rc::new(RefCell::new(StandardJob::new_indeterminate(tr!("INDETERMINATE_JOB_TITLE", "Indeterminate Job").into(), tr!("INDETERMINATE_JOB_IN_PROGRESS_DESCRIPTION", "This job has an indeterminate progress bar.").into())));
                                                let job_entity_source = job.clone();
                                                let job_entity = cx.new::<Jobling>(|_| {
                                                    job_entity_source
                                                });

                                                let job_clone = job_entity.clone();
                                                cx.spawn(async move |_, cx: &mut AsyncApp| {
                                                    cx.background_executor().timer(Duration::from_secs(10)).await;
                                                    job_clone.update(cx, |_, cx| {
                                                        job.borrow_mut().update_job_status(
                                                            tr!("STANDARD_JOB_COMPLETE_DESCRIPTION", "This job is now complete.").into(),
                                                            JobStatus::Completed
                                                        );
                                                        cx.notify();
                                                    }).unwrap();
                                                }).detach();
                                                cx.update_global::<JobManager, ()>(|job_manager, cx| {
                                                    job_manager.add_job(job_entity, cx);
                                                });
                                            })),
                                    )
                                    .child(
                                        button("job-transient")
                                            .child(tr!("JOB_TRANSIENT_START", "Start Transient Job"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                let job = Rc::new(RefCell::new(StandardJob::new_transient(tr!("TRANSIENT_JOB_TITLE", "Transient Job").into(), tr!("TRANSIENT_JOB_IN_PROGRESS_DESCRIPTION", "This job is transient and so will disappear automatically once it is complete.").into())));
                                                let job_entity_source = job.clone();
                                                let job_entity = cx.new::<Jobling>(|_| {
                                                    job_entity_source
                                                });

                                                let job_clone = job_entity.clone();
                                                cx.spawn(async move |_, cx: &mut AsyncApp| {
                                                    let instant = Instant::now();
                                                    while instant.elapsed().as_secs_f32() < 10. {
                                                        job_clone.update(cx, |_, cx| {
                                                            job.borrow_mut().update_job_progress(
                                                                (instant.elapsed().as_secs_f32() * 1000.) as u64,
                                                                10000,
                                                            );
                                                            cx.notify();
                                                        }).unwrap();
                                                        cx.background_executor().timer(Duration::from_millis(10)).await;
                                                    }
                                                    job_clone.update(cx, |_, cx| {
                                                        job.borrow_mut().update_job_status(
                                                            tr!("STANDARD_JOB_COMPLETE_DESCRIPTION", "This job is now complete.").into(),
                                                            JobStatus::Completed
                                                        );
                                                        cx.notify();
                                                    }).unwrap();
                                                }).detach();
                                                cx.update_global::<JobManager, ()>(|job_manager, cx| {
                                                    job_manager.add_job(job_entity, cx);
                                                });
                                            })),
                                    )
                                    .child(
                                        button("job-failing")
                                            .child(tr!("JOB_FAILING_START", "Start Failing Job"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                let job = Rc::new(RefCell::new(StandardJob::new(tr!("FAILING_JOB_TITLE", "Failing Job").into(), tr!("FAILING_JOB_IN_PROGRESS_DESCRIPTION", "This job will fail halfway through processing.").into())));
                                                let job_entity_source = job.clone();
                                                let job_entity = cx.new::<Jobling>(|_| {
                                                    job_entity_source
                                                });

                                                let job_clone = job_entity.clone();
                                                cx.spawn(async move |_, cx: &mut AsyncApp| {
                                                    let instant = Instant::now();
                                                    while instant.elapsed().as_secs_f32() < 5. {
                                                        job_clone.update(cx, |_, cx| {
                                                            job.borrow_mut().update_job_progress(
                                                                (instant.elapsed().as_secs_f32() * 1000.) as u64,
                                                                10000,
                                                            );
                                                            cx.notify();
                                                        }).unwrap();
                                                        cx.background_executor().timer(Duration::from_millis(10)).await;
                                                    }
                                                    job_clone.update(cx, |_, cx| {
                                                        job.borrow_mut().update_job_status(
                                                            tr!("FAILING_JOB_COMPLETE_DESCRIPTION", "This job has failed.").into(),
                                                            JobStatus::Failed
                                                        );
                                                        cx.notify();
                                                    }).unwrap();
                                                }).detach();
                                                cx.update_global::<JobManager, ()>(|job_manager, cx| {
                                                    job_manager.add_job(job_entity, cx);
                                                });
                                            })),
                                    ),
                            ),
                    ),
            )
    }
}
