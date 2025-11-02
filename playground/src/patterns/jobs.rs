use cntp_i18n::tr;
use contemporary::components::button::button;
use contemporary::components::constrainer::constrainer;
use contemporary::components::grandstand::grandstand;
use contemporary::components::layer::layer;
use contemporary::components::subtitle::subtitle;
use contemporary::jobs::job::JobStatus;
use contemporary::jobs::job_manager::{JobManager, Jobling};
use contemporary::jobs::standard_job::StandardJob;
use contemporary::styling::theme::ThemeStorage;
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
        let theme = cx.theme();
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
                                                    job_manager.track_job(job_entity, cx);
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
                                                            tr!("STANDARD_JOB_COMPLETE_DESCRIPTION").into(),
                                                            JobStatus::Completed
                                                        );
                                                        cx.notify();
                                                    }).unwrap();
                                                }).detach();
                                                cx.update_global::<JobManager, ()>(|job_manager, cx| {
                                                    job_manager.track_job(job_entity, cx);
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
                                                            tr!("STANDARD_JOB_COMPLETE_DESCRIPTION").into(),
                                                            JobStatus::Completed
                                                        );
                                                        cx.notify();
                                                    }).unwrap();
                                                }).detach();
                                                cx.update_global::<JobManager, ()>(|job_manager, cx| {
                                                    job_manager.track_job(job_entity, cx);
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
                                                    job_manager.track_job(job_entity, cx);
                                                });
                                            })),
                                    ),
                            ),
                    ),
            )
            .child(
                constrainer("delayed-tracking")
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
                            .child(subtitle(tr!("JOB_DELAYED_TRACKING_TITLE", "Delayed Tracking")))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(tr!(
                                            "JOB_DELAYED_TRACKING_DESCRIPTION",
                                            "For jobs that have the potential to finish quickly, delayed tracking ensures the job isn't registered in the job manager if it is finished quickly."
                                        ))
                                    .child(
                                        button("delayed-tracking-quick")
                                            .child(tr!("JOB_DELAYED_TRACKING_QUICK", "Start Quick Job with Delayed Tracking"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                let job = Rc::new(RefCell::new(StandardJob::new(tr!("JOB_DELAYED_TRACKING_QUICK_TITLE", "Quick Delayed Tracking").into(), tr!("JOB_DELAYED_TRACKING_QUICK_IN_PROGRESS_DESCRIPTION", "This job won't show up in the job menu because it finishes too quickly.").into())));
                                                let job_entity_source = job.clone();
                                                let job_entity = cx.new::<Jobling>(|_| {
                                                    job_entity_source
                                                });

                                                let job_clone = job_entity.clone();
                                                cx.spawn(async move |_, cx: &mut AsyncApp| {
                                                    let instant = Instant::now();
                                                    while instant.elapsed().as_secs_f32() < 0.5 {
                                                        job_clone.update(cx, |_, cx| {
                                                            job.borrow_mut().update_job_progress(
                                                                (instant.elapsed().as_secs_f32() * 1000.) as u64,
                                                                500,
                                                            );
                                                            cx.notify();
                                                        }).unwrap();
                                                        cx.background_executor().timer(Duration::from_millis(10)).await;
                                                    }
                                                    job_clone.update(cx, |_, cx| {
                                                        job.borrow_mut().update_job_status(
                                                            tr!("STANDARD_JOB_COMPLETE_DESCRIPTION").into(),
                                                            JobStatus::Completed
                                                        );
                                                        cx.notify();
                                                    }).unwrap();
                                                }).detach();
                                                cx.update_global::<JobManager, ()>(|job_manager, cx| {
                                                    job_manager.track_job_delayed_default(job_entity, cx);
                                                });
                                            })),
                                    )
                                    .child(
                                        button("delayed-tracking-slow")
                                            .child(tr!("JOB_DELAYED_TRACKING_SLOW", "Start Slow Job with Delayed Tracking"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                let job = Rc::new(RefCell::new(StandardJob::new(tr!("JOB_DELAYED_TRACKING_SLOW_TITLE", "Slow Delayed Tracking").into(), tr!("JOB_DELAYED_TRACKING_SLOW_IN_PROGRESS_DESCRIPTION", "This job will show up in the job menu because it takes some time to finish.").into())));
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
                                                            tr!("STANDARD_JOB_COMPLETE_DESCRIPTION").into(),
                                                            JobStatus::Completed
                                                        );
                                                        cx.notify();
                                                    }).unwrap();
                                                }).detach();
                                                cx.update_global::<JobManager, ()>(|job_manager, cx| {
                                                    job_manager.track_job_delayed_default(job_entity, cx);
                                                });
                                            })),
                                    )
                                    .child(
                                        button("delayed-tracking-quick-fail")
                                            .child(tr!("JOB_DELAYED_TRACKING_QUICK_FAIL", "Start Failing Job with Delayed Tracking"))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                let job = Rc::new(RefCell::new(StandardJob::new(tr!("JOB_DELAYED_TRACKING_QUICK_FAIL_TITLE", "Failing Job with Delayed Tracking").into(), tr!("JOB_DELAYED_TRACKING_QUICK_FAIL_IN_PROGRESS_DESCRIPTION", "This job shows up in the job menu before the specified delay because it fails before then.").into())));
                                                let job_entity_source = job.clone();
                                                let job_entity = cx.new::<Jobling>(|_| {
                                                    job_entity_source
                                                });

                                                let job_clone = job_entity.clone();
                                                cx.spawn(async move |_, cx: &mut AsyncApp| {
                                                    let instant = Instant::now();
                                                    while instant.elapsed().as_secs_f32() < 0.5 {
                                                        job_clone.update(cx, |_, cx| {
                                                            job.borrow_mut().update_job_progress(
                                                                (instant.elapsed().as_secs_f32() * 1000.) as u64,
                                                                500,
                                                            );
                                                            cx.notify();
                                                        }).unwrap();
                                                        cx.background_executor().timer(Duration::from_millis(10)).await;
                                                    }
                                                    job_clone.update(cx, |_, cx| {
                                                        job.borrow_mut().update_job_status(
                                                            tr!("FAILING_JOB_COMPLETE_DESCRIPTION").into(),
                                                            JobStatus::Failed
                                                        );
                                                        cx.notify();
                                                    }).unwrap();
                                                }).detach();
                                                cx.update_global::<JobManager, ()>(|job_manager, cx| {
                                                    job_manager.track_job_delayed_default(job_entity, cx);
                                                });
                                            })),
                                    )
                            )
                    )
            )
            .child(
            constrainer("job-menu")
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
                        .child(subtitle(tr!("JOB_MENU_TITLE", "Job Menu")))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(8.))
                                .child(tr!(
                                            "JOB_MENU_DESCRIPTION",
                                            "If you need to, (for example, if a job requires attention soon after it is started) you can programmatically open the job menu."
                                        ))
                                .child(button("open-job-menu").child(tr!("OPEN_JOB_MENU", "Open Job Menu")).on_click(cx.listener(|this, _, _, cx| {
                                    cx.update_global::<JobManager, ()>(|job_manager, cx| {
                                        job_manager.set_job_menu_open(true, cx);
                                    })
                                })))
                        )
                )
        )
    }
}
