use crate::handle::{CloneableReceiver, JobHandle, JobState};
use crate::jobs::{Job, JobTypeKey};
use mediarepo_core::tokio_graceful_shutdown::SubsystemHandle;
use mediarepo_core::trait_bound_typemap::{SendSyncTypeMap, TypeMap, TypeMapKey};
use mediarepo_logic::dao::repo::Repo;
use mediarepo_logic::dao::DaoProvider;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast::channel;
use tokio::sync::RwLock;
use tokio::time::Instant;

#[derive(Clone)]
pub struct JobDispatcher {
    subsystem: SubsystemHandle,
    job_handle_map: Arc<RwLock<SendSyncTypeMap>>,
    repo: Arc<Repo>,
}

impl JobDispatcher {
    pub fn new(subsystem: SubsystemHandle, repo: Repo) -> Self {
        Self {
            job_handle_map: Arc::new(RwLock::new(SendSyncTypeMap::new())),
            subsystem,
            repo: Arc::new(repo),
        }
    }

    pub async fn dispatch<T: 'static + Job>(&self, job: T) -> JobHandle<T::JobStatus, T::Result> {
        self._dispatch(job, None).await
    }

    pub async fn dispatch_periodically<T: 'static + Job>(
        &self,
        job: T,
        interval: Duration,
    ) -> JobHandle<T::JobStatus, T::Result> {
        self._dispatch(job, Some(interval)).await
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn _dispatch<T: 'static + Job>(
        &self,
        job: T,
        interval: Option<Duration>,
    ) -> JobHandle<T::JobStatus, T::Result> {
        let status = job.status();
        let state = Arc::new(RwLock::new(JobState::Queued));
        let (sender, mut receiver) = channel(1);
        self.subsystem
            .start("channel-consumer", move |subsystem| async move {
                tokio::select! {
                    _ = receiver.recv() => (),
                    _ = subsystem.on_shutdown_requested() => (),
                }
                Ok(())
            });
        let receiver = CloneableReceiver::new(sender.clone());
        let handle = JobHandle::new(status.clone(), state.clone(), receiver);
        self.add_handle::<JobTypeKey<T>>(handle.clone()).await;

        let repo = self.repo.clone();

        self.subsystem
            .start("worker-job", move |subsystem| async move {
                loop {
                    let start = Instant::now();
                    let job_2 = job.clone();
                    {
                        let mut state = state.write().await;
                        *state = JobState::Running;
                    }
                    let result = tokio::select! {
                        _ = subsystem.on_shutdown_requested() => {
                            job_2.save_state(repo.job()).await
                        }
                        r = job.run(repo.clone()) => {
                            match r {
                                Err(e) => Err(e),
                                Ok(v) => {
                                    let _ = sender.send(Arc::new(RwLock::new(Ok(v))));
                                    job.save_state(repo.job()).await
                                }
                            }
                        }
                    };
                    if let Err(e) = result {
                        tracing::error!("job failed with error: {}", e);
                    }
                    if let Some(interval) = interval {
                        {
                            let mut state = state.write().await;
                            *state = JobState::Scheduled;
                        }
                        let sleep_duration = interval - start.elapsed();
                        tokio::select! {
                            _  =  tokio::time::sleep(sleep_duration) => {},
                            _ = subsystem.on_shutdown_requested() => {break}
                        }
                    } else {
                        let mut state = state.write().await;
                        *state = JobState::Finished;
                        break;
                    }
                }

                Ok(())
            });

        handle
    }

    #[inline]
    async fn add_handle<T: TypeMapKey>(&self, status: T::Value)
    where
        <T as TypeMapKey>::Value: Send + Sync,
    {
        let mut status_map = self.job_handle_map.write().await;
        status_map.insert::<T>(status);
    }
}

pub struct DispatcherKey;

impl TypeMapKey for DispatcherKey {
    type Value = JobDispatcher;
}

unsafe impl Send for JobDispatcher {}
unsafe impl Sync for JobDispatcher {}
