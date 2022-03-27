use mediarepo_database::entities::job_state;
use mediarepo_database::entities::job_state::JobType;

#[derive(Clone, Debug)]
pub struct JobStateDto {
    model: job_state::Model,
}

impl JobStateDto {
    pub(crate) fn new(model: job_state::Model) -> Self {
        Self { model }
    }

    pub fn job_type(&self) -> JobType {
        self.model.job_type
    }

    pub fn value(&self) -> &[u8] {
        &self.model.value
    }

    pub fn into_value(self) -> Vec<u8> {
        self.model.value
    }
}

#[derive(Clone, Debug)]
pub struct UpsertJobStateDto {
    pub job_type: JobType,
    pub value: Vec<u8>,
}
