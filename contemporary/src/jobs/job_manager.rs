use crate::jobs::job::Job;
use gpui::{App, BorrowAppContext, Entity, Global};
use std::cell::RefCell;
use std::rc::Rc;

pub type Jobling = Rc<RefCell<dyn Job>>;
pub type JoblingEntity = Entity<Jobling>;

pub struct JobManager {
    jobs: Vec<JoblingEntity>,
}

impl JobManager {
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    pub fn add_job(&mut self, job: JoblingEntity, cx: &mut App) {
        cx.observe(&job, |_, cx| cx.update_global::<Self, ()>(|_, _| {}))
            .detach();
        self.jobs.push(job);
    }

    pub fn job_len(&self) -> usize {
        self.jobs.len()
    }

    pub fn job(&self, index: usize) -> JoblingEntity {
        self.jobs[index].clone()
    }

    pub fn aggregate_progress(&self, cx: &App) -> f32 {
        self.jobs
            .iter()
            .map(|job| job.read(cx).borrow().progress())
            .sum::<f32>()
            / self.jobs.len() as f32
    }
}

impl Global for JobManager {}
