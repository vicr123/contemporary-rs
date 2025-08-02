use crate::jobs::job::{Job, JobStatus};
use gpui::{App, BorrowAppContext, Entity, Global};

use std::cell::RefCell;
use std::rc::Rc;

pub type Jobling = Rc<RefCell<dyn Job>>;
pub type JoblingEntity = Entity<Jobling>;

pub struct JobManager {
    jobs: Vec<JoblingEntity>,
    pub is_job_menu_open: bool,
    unfinished_jobs: Vec<JoblingEntity>,
}

#[derive(PartialEq)]
pub enum JobButtonState {
    Hidden,
    Idle,
    HaveRequiresAttention,
    HaveFailure,
    HaveSuccess,
}

impl JobManager {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            is_job_menu_open: false,
            unfinished_jobs: Vec::new(),
        }
    }

    pub fn add_job(&mut self, job: JoblingEntity, cx: &mut App) {
        cx.observe(&job, |job_entity, cx| {
            cx.update_global::<Self, ()>(|this, cx| {
                let job = job_entity.read(cx);
                let job_borrow = job.borrow();

                if job_borrow.transient()
                    && job_borrow.status().is_complete()
                    && this.unfinished_jobs.contains(&job_entity)
                {
                    this.unfinished_jobs.retain(|job| *job != job_entity);
                }
            })
        })
        .detach();
        self.jobs.push(job.clone());
        self.unfinished_jobs.push(job);
    }

    fn tracked_jobs(&self, cx: &App) -> impl Iterator<Item = &JoblingEntity> {
        self.jobs.iter().filter(|job| {
            let job = job.read(cx);
            let job_borrow = job.borrow();
            !job_borrow.transient()
                || matches!(
                    job_borrow.status(),
                    JobStatus::InProgress | JobStatus::RequiresAttention
                )
        })
    }

    pub fn job_len(&self, cx: &App) -> usize {
        self.tracked_jobs(cx).count()
    }

    pub fn job(&self, index: usize, cx: &App) -> Option<&JoblingEntity> {
        self.tracked_jobs(cx).skip(index).next()
    }

    pub fn aggregate_progress(&self, cx: &App) -> f32 {
        self.jobs
            .iter()
            .map(|job| job.read(cx).borrow().progress())
            .sum::<f32>()
            / self.jobs.len() as f32
    }

    pub fn set_job_menu_open(&mut self, open: bool, cx: &App) {
        self.is_job_menu_open = open;
        if open {
            let complete_jobs: Vec<_> = self
                .tracked_jobs(cx)
                .filter(|job| job.read(cx).borrow().status().is_complete())
                .cloned()
                .collect();
            self.unfinished_jobs
                .retain(|job| !complete_jobs.iter().any(|complete_job| complete_job == job));
        }
    }

    pub fn job_button_state(&self, cx: &App) -> JobButtonState {
        let unfinished_jobs: Vec<_> = self
            .unfinished_jobs
            .iter()
            .map(|job| job.read(cx).borrow())
            .collect();
        if unfinished_jobs.is_empty() {
            JobButtonState::Hidden
        } else if unfinished_jobs
            .iter()
            .any(|job| job.status() == JobStatus::RequiresAttention)
        {
            JobButtonState::HaveRequiresAttention
        } else if unfinished_jobs
            .iter()
            .any(|job| job.status() == JobStatus::Failed)
        {
            JobButtonState::HaveFailure
        } else if unfinished_jobs
            .iter()
            .any(|job| job.status() == JobStatus::Completed)
        {
            JobButtonState::HaveSuccess
        } else {
            JobButtonState::Idle
        }
    }
}

impl Global for JobManager {}
