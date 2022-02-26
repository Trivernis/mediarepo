use chrono::NaiveDateTime;
use mediarepo_database::entities::job;
use mediarepo_database::entities::job::JobType;

#[derive(Clone, Debug)]
pub struct JobDto {
    model: job::Model,
}

impl JobDto {
    pub(crate) fn new(model: job::Model) -> Self {
        Self { model }
    }

    pub fn id(&self) -> i64 {
        self.model.id
    }

    pub fn job_type(&self) -> JobType {
        self.model.job_type
    }

    pub fn name(&self) -> Option<&String> {
        self.model.name.as_ref()
    }

    pub fn next_run(&self) -> Option<NaiveDateTime> {
        self.model.next_run
    }

    pub fn interval(&self) -> Option<i64> {
        self.model.interval
    }
}
