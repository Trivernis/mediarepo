use crate::progress::JobProgressUpdate;
use mediarepo_logic::dto::JobDto;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type JobsTable = Arc<RwLock<HashMap<i64, JobEntry>>>;

#[derive(Clone, Debug)]
pub struct JobEntry {
    pub dto: JobDto,
    pub last_update: Option<JobProgressUpdate>,
}
