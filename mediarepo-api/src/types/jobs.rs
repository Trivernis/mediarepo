use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RunJobRequest {
    pub job_type: JobType,
    pub sync: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum JobType {
    MigrateContentDescriptors,
    CalculateSizes,
    CheckIntegrity,
}
