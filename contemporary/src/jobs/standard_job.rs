use crate::components::layer::layer;
use crate::components::progress_bar::progress_bar;
use crate::components::subtitle::subtitle;
use crate::jobs::job::{Job, JobStatus};
use gpui::prelude::FluentBuilder;
use gpui::{AnyElement, IntoElement, ParentElement, SharedString, Styled, px};

pub struct StandardJob {
    progress: u64,
    max_progress: u64,
    title: SharedString,
    description: SharedString,
    status: JobStatus,
    cancellation_callback: Option<Box<dyn Fn()>>,
    transient: bool,
}

impl StandardJob {
    pub fn new(title: SharedString, description: SharedString) -> Self {
        Self {
            progress: 0,
            max_progress: 100,
            title,
            description,
            status: JobStatus::InProgress,
            cancellation_callback: None,
            transient: false,
        }
    }

    pub fn new_indeterminate(title: SharedString, description: SharedString) -> Self {
        Self {
            max_progress: 0,
            ..Self::new(title, description)
        }
    }

    pub fn new_transient(title: SharedString, description: SharedString) -> Self {
        Self {
            transient: true,
            ..Self::new(title, description)
        }
    }

    pub fn update_job_status(&mut self, description: SharedString, status: JobStatus) {
        self.description = description;
        self.status = status;
    }

    pub fn update_job_progress(&mut self, progress: u64, max_progress: u64) {
        self.progress = progress;
        self.max_progress = max_progress;
    }

    pub fn set_job_progress_indeterminate(&mut self) {
        self.max_progress = 0;
    }

    pub fn with_cancellation_callback(mut self, callback: impl Fn() + 'static) -> Self {
        self.cancellation_callback = Some(Box::new(callback));
        self
    }
}

impl Job for StandardJob {
    fn progress(&self) -> f32 {
        match self.status {
            JobStatus::InProgress | JobStatus::RequiresAttention => {
                if self.max_progress == 0 {
                    0.
                } else {
                    self.progress as f32 / self.max_progress as f32
                }
            }
            JobStatus::Completed | JobStatus::Failed => 1.,
        }
    }

    fn progress_indeterminate(&self) -> bool {
        match self.status {
            JobStatus::InProgress | JobStatus::RequiresAttention => self.max_progress == 0,
            JobStatus::Completed | JobStatus::Failed => false,
        }
    }

    fn status(&self) -> JobStatus {
        self.status
    }

    fn transient(&self) -> bool {
        self.transient
    }

    fn element(&self) -> AnyElement {
        layer()
            .flex()
            .flex_col()
            .w_full()
            .p(px(10.))
            .gap(px(6.))
            .child(subtitle(self.title.clone()))
            .child(self.description.clone())
            .when(self.status == JobStatus::InProgress, |david| {
                if self.progress_indeterminate() {
                    david.child(progress_bar().indeterminate("indeterminate-bar"))
                } else {
                    david.child(progress_bar().value(self.progress()))
                }
            })
            .into_any_element()
    }
}
