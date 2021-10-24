use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FileIdentifier {
    ID(i64),
    Hash(String),
}