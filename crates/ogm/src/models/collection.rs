use aragog::Record;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Record, Debug)]
pub struct Collection {
    pub name: String,
    // make same to be using by different user
    // but only once
    pub created_by: String,
    pub is_public: bool
}