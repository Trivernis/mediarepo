use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum JobExecutionState {
    Scheduled,
    Running,
    Finished,
}

pub struct ExecutionStateSynchronizer {
    state: Arc<AtomicU8>,
}

impl Default for ExecutionStateSynchronizer {
    fn default() -> Self {
        Self {
            state: Arc::new(AtomicU8::new(0)),
        }
    }
}

impl ExecutionStateSynchronizer {
    pub fn set_scheduled(&self) {
        self.state.store(0, Ordering::Relaxed);
    }

    #[must_use]
    pub fn set_running(&self) -> RunningHandle {
        self.state.store(1, Ordering::Relaxed);
        RunningHandle {
            state: Arc::clone(&self.state),
        }
    }

    pub fn set_finished(&self) {
        self.state.store(2, Ordering::SeqCst)
    }

    pub fn state(&self) -> JobExecutionState {
        match self.state.load(Ordering::SeqCst) {
            0 => JobExecutionState::Scheduled,
            1 => JobExecutionState::Running,
            2 => JobExecutionState::Scheduled,
            _ => JobExecutionState::Finished,
        }
    }
}

pub struct RunningHandle {
    state: Arc<AtomicU8>,
}

impl Drop for RunningHandle {
    fn drop(&mut self) {
        self.state.store(2, Ordering::SeqCst);
    }
}
