use crate::jobs::{Job, JobTypeKey};
use mediarepo_core::tokio_graceful_shutdown::SubsystemHandle;
use mediarepo_core::typemap_rev::{TypeMap, TypeMapKey};
use mediarepo_logic::dao::repo::Repo;
use mediarepo_logic::dao::DaoProvider;
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::Instant;

#[derive(Clone)]
pub struct JobDispatcher {
    subsystem: Arc<UnsafeCell<SubsystemHandle>>,
    job_status_map: Arc<RwLock<TypeMap>>,
    repo: Arc<Repo>,
}

impl JobDispatcher {
    pub fn new(subsystem: SubsystemHandle, repo: Repo) -> Self {
        Self {
            job_status_map: Default::default(),
            subsystem: Arc::new(UnsafeCell::new(subsystem)),
            repo: Arc::new(repo),
        }
    }

    pub async fn dispatch<T: 'static + Job>(&self, job: T) -> Arc<RwLock<T::JobStatus>> {
        self._dispatch(job, None).await
    }

    pub async fn dispatch_periodically<T: 'static + Job>(
        &self,
        job: T,
        interval: Duration,
    ) -> Arc<RwLock<T::JobStatus>> {
        self._dispatch(job, Some(interval)).await
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn _dispatch<T: 'static + Job>(
        &self,
        job: T,
        interval: Option<Duration>,
    ) -> Arc<RwLock<T::JobStatus>> {
        let status = job.status();
        self.add_status::<JobTypeKey<T>>(status.clone()).await;

        let subsystem = unsafe {
            // SAFETY: the subsystem requires a mutable borrow for the start method
            // the implementation of start doesn't need that mutability. So until that's
            // changed we have to do some trickery.
            &mut *self.subsystem.get()
        };

        let repo = self.repo.clone();

        subsystem.start("worker-job", move |subsystem| async move {
            loop {
                let start = Instant::now();
                let job_2 = job.clone();
                let result = tokio::select! {
                    _ = subsystem.on_shutdown_requested() => {
                        job_2.save_status(repo.job()).await
                    }
                    r = job.run(repo.clone()) => {

                        if let Err(e) = r {
                            Err(e)
                        } else {
                            job.save_status(repo.job()).await
                        }
                    }
                };
                if let Err(e) = result {
                    tracing::error!("job failed with error: {}", e);
                }
                if let Some(interval) = interval {
                    let sleep_duration = interval - start.elapsed();
                    tokio::select! {
                        _  =  tokio::time::sleep(sleep_duration) => {},
                        _ = subsystem.on_shutdown_requested() => {break}
                    }
                } else {
                    break;
                }
            }

            Ok(())
        });

        status
    }

    #[inline]
    async fn add_status<T: TypeMapKey>(&self, status: T::Value) {
        let mut status_map = self.job_status_map.write().await;
        status_map.insert::<T>(status);
    }
}

pub struct DispatcherKey;

impl TypeMapKey for DispatcherKey {
    type Value = JobDispatcher;
}

unsafe impl Send for JobDispatcher {}
unsafe impl Sync for JobDispatcher {}
