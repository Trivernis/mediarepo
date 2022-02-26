use mediarepo_database::entities::job_state;

#[derive(Clone, Debug)]
pub struct JobStateDto {
    model: job_state::Model,
}

impl JobStateDto {
    pub(crate) fn new(model: job_state::Model) -> Self {
        Self { model }
    }

    pub fn job_id(&self) -> i64 {
        self.model.job_id
    }

    pub fn key(&self) -> &String {
        &self.model.key
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
    pub job_id: i64,
    pub key: String,
    pub value: Vec<u8>,
}
