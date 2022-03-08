use crate::execution_state::{ExecutionStateSynchronizer, JobExecutionState, RunningHandle};
use tokio::sync::mpsc::Sender;

#[derive(Clone, Debug)]
pub struct JobProgressUpdate {
    id: i64,
    state: JobExecutionState,
    progress: Option<u64>,
    total: Option<u64>,
}

impl JobProgressUpdate {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            state: JobExecutionState::Scheduled,
            progress: None,
            total: None,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn state(&self) -> JobExecutionState {
        self.state
    }

    pub fn set_state(&mut self, state: JobExecutionState) {
        self.state = state;
    }

    pub fn progress(&self) -> Option<u64> {
        self.progress
    }

    pub fn set_progress(&mut self, progress: u64) {
        self.progress = Some(progress);
    }

    pub fn total(&self) -> Option<u64> {
        self.total
    }

    pub fn set_total(&mut self, total: u64) {
        self.total = Some(total)
    }
}

pub struct ProgressSender {
    job_id: i64,
    execution_state_sync: ExecutionStateSynchronizer,
    pub inner: Sender<JobProgressUpdate>,
}

impl ProgressSender {
    pub fn new(job_id: i64, sender: Sender<JobProgressUpdate>) -> Self {
        Self {
            job_id,
            inner: sender,
            execution_state_sync: ExecutionStateSynchronizer::default(),
        }
    }

    pub fn send_progress(&self, progress: u64, total: u64) {
        let _ = self.inner.send(JobProgressUpdate {
            id: self.job_id,
            state: JobExecutionState::Running,
            progress: Some(progress),
            total: Some(total),
        });
    }

    pub fn send_progress_percent(&self, percent: f64) {
        self.send_progress((percent * 100.0) as u64, 100);
    }
}
