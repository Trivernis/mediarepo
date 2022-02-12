use crate::tauri_plugin::error::{PluginError, PluginResult};
use futures::future;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use std::{mem, thread};
use tokio::sync::{Mutex, RwLock};

#[derive(Clone, Debug)]
pub struct TaskContext {
    tasks: Arc<RwLock<HashMap<String, AsyncTask>>>,
}

impl TaskContext {
    pub fn new() -> Self {
        Self {
            tasks: Default::default(),
        }
    }

    pub async fn add_task<S: ToString, F: 'static + Future<Output = PluginResult<()>>>(
        &self,
        name: S,
        task: F,
    ) {
        self.tasks
            .write()
            .await
            .insert(name.to_string(), AsyncTask::new(task));
    }

    pub async fn task_state<S: AsRef<str>>(&self, name: S) -> Option<TaskState> {
        let state = {
            let tasks = self.tasks.read().await;

            if let Some(task) = tasks.get(name.as_ref()) {
                Some(task.state().await)
            } else {
                None
            }
        };
        if let Some(TaskState::Finished) = state {
            self.tasks.write().await.remove(name.as_ref());
        }

        state
    }

    /// Returns all tasks queued for execution
    async fn queued_tasks(&self) -> Vec<AsyncTask> {
        let task_map = self.tasks.read().await;
        let mut tasks = Vec::new();

        for task in task_map.values() {
            if task.state().await == TaskState::Queued {
                tasks.push(task.clone());
            }
        }

        tasks
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum TaskState {
    Queued,
    Running,
    Finished,
    Error,
}

impl TaskState {
    pub fn error(&self) -> bool {
        *self == TaskState::Error
    }
}

#[derive(Clone)]
pub struct AsyncTask {
    state: Arc<RwLock<TaskState>>,
    inner: Arc<Mutex<Option<Pin<Box<dyn Future<Output = PluginResult<()>>>>>>>,
    error: Arc<RwLock<Option<PluginError>>>,
}

impl Debug for AsyncTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AsyncTask (state: {:?})", self.state)
    }
}

impl AsyncTask {
    pub fn new<F: 'static + Future<Output = PluginResult<()>>>(inner: F) -> Self {
        Self {
            state: Arc::new(RwLock::new(TaskState::Queued)),
            inner: Arc::new(Mutex::new(Some(Box::pin(inner)))),
            error: Default::default(),
        }
    }

    pub async fn exec(&self) {
        self.set_state(TaskState::Running).await;

        let inner = self.inner.lock().await.take();
        if let Some(task) = inner {
            if let Err(e) = task.await {
                let _ = mem::replace(&mut *self.error.write().await, Some(e));
                self.set_state(TaskState::Error).await;
            } else {
                self.set_state(TaskState::Finished).await;
            }
        } else {
            self.set_state(TaskState::Finished).await;
        }
    }

    pub async fn state(&self) -> TaskState {
        self.state.read().await.clone()
    }

    async fn set_state(&self, state: TaskState) {
        let _ = mem::replace(&mut *self.state.write().await, state);
    }
}

unsafe impl Send for AsyncTask {}
unsafe impl Sync for AsyncTask {}

pub fn start_background_task_runtime(ctx: TaskContext) {
    thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .thread_name("background_tasks")
            .enable_time()
            .build()
            .expect("failed to build background task runtime")
            .block_on(async move {
                tracing::debug!("background task listener ready");
                loop {
                    let tasks = ctx.queued_tasks().await;

                    if tasks.len() > 0 {
                        tracing::debug!("executing {} async background tasks", tasks.len());
                        let start = SystemTime::now();
                        future::join_all(tasks.iter().map(|t| t.exec())).await;
                        tracing::debug!(
                            "background tasks executed in {} ms",
                            start.elapsed().unwrap().as_millis()
                        );
                    } else {
                        tokio::time::sleep(Duration::from_millis(100)).await;
                    }
                }
            });
        tracing::error!("background task executor exited!");
    });
}
