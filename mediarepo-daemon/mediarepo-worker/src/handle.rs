use mediarepo_core::error::{RepoError, RepoResult};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::RwLock;

pub struct JobHandle<T: Send + Sync, R: Send + Sync> {
    status: Arc<RwLock<T>>,
    state: Arc<RwLock<JobState>>,
    result_receiver: CloneableReceiver<Arc<RwLock<Option<RepoResult<R>>>>>,
}

impl<T: Send + Sync, R: Send + Sync> Clone for JobHandle<T, R> {
    fn clone(&self) -> Self {
        Self {
            status: self.status.clone(),
            state: self.state.clone(),
            result_receiver: self.result_receiver.clone(),
        }
    }
}

impl<T: Send + Sync, R: Send + Sync> JobHandle<T, R> {
    pub fn new(
        status: Arc<RwLock<T>>,
        state: Arc<RwLock<JobState>>,
        result_receiver: CloneableReceiver<Arc<RwLock<Option<RepoResult<R>>>>>,
    ) -> Self {
        Self {
            status,
            state,
            result_receiver,
        }
    }

    pub async fn state(&self) -> JobState {
        *self.state.read().await
    }

    pub fn status(&self) -> &Arc<RwLock<T>> {
        &self.status
    }

    pub async fn result(&mut self) -> Arc<RwLock<Option<RepoResult<R>>>> {
        match self.result_receiver.recv().await {
            Ok(v) => v,
            Err(e) => Arc::new(RwLock::new(Some(Err(RepoError::from(&*e.to_string()))))),
        }
    }

    pub async fn take_result(&mut self) -> Option<RepoResult<R>> {
        let shared_result = self.result().await;
        let mut result = shared_result.write().await;
        result.take()
    }
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum JobState {
    Queued,
    Scheduled,
    Running,
    Finished,
}

pub struct CloneableReceiver<T: Clone> {
    receiver: Receiver<T>,
    sender: Sender<T>,
}

impl<T: Clone> CloneableReceiver<T> {
    pub fn new(sender: Sender<T>) -> Self {
        Self {
            receiver: sender.subscribe(),
            sender,
        }
    }
}

impl<T: Clone> Clone for CloneableReceiver<T> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            receiver: self.sender.subscribe(),
        }
    }
}

impl<T: Clone> Deref for CloneableReceiver<T> {
    type Target = Receiver<T>;

    fn deref(&self) -> &Self::Target {
        &self.receiver
    }
}

impl<T: Clone> DerefMut for CloneableReceiver<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.receiver
    }
}
