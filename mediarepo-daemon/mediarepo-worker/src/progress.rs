use crate::jobs::JobExecutionState;

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
