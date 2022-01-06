use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FileIdentifier {
    ID(i64),
    CD(String),
}
