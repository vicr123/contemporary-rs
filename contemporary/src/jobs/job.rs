use gpui::AnyElement;

#[derive(Clone, Copy, PartialEq)]
pub enum JobStatus {
    InProgress,
    Completed,
    Failed,
    RequiresAttention,
}

pub trait Job {
    /// Returns a number between 0.0 and 1.0 representing the progress of this job
    fn progress(&self) -> f32;

    /// Returns true if progress is unknown, false otherwise
    fn progress_indeterminate(&self) -> bool {
        false
    }

    /// Returns the status of this job
    fn status(&self) -> JobStatus;

    /// Returns true if the job stays around after it is complete, false otherwise
    fn transient(&self) -> bool {
        false
    }

    /// Returns an element to be rendered in the Jobs pane
    fn element(&self) -> AnyElement;
}
