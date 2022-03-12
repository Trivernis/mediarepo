use crate::job_dispatcher::JobDispatcher;
use crate::jobs::VacuumJob;
use mediarepo_core::error::RepoError;
use mediarepo_core::tokio_graceful_shutdown::Toplevel;
use mediarepo_logic::dao::repo::Repo;
use tokio::sync::oneshot::channel;

pub mod job_dispatcher;
pub mod jobs;

pub async fn start(top_level: Toplevel, repo: Repo) -> (Toplevel, JobDispatcher) {
    let (tx, rx) = channel();

    let top_level = top_level.start("mediarepo-worker", |subsystem| async move {
        let dispatcher = JobDispatcher::new(subsystem, repo);
        tx.send(dispatcher.clone())
            .map_err(|_| RepoError::from("failed to send dispatcher"))?;
        dispatcher.dispatch(VacuumJob::default()).await;

        Ok(())
    });
    let receiver = rx
        .await
        .expect("failed to create background job dispatcher");

    (top_level, receiver)
}
