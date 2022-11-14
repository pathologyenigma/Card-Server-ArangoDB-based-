
use aragog::Record;
use serde::{Deserialize, Serialize};



#[derive(Clone, Serialize, Deserialize, Record, Debug)]
pub struct Collection {
    pub name: String,
    // make same to be using by different user
    // but only once
    pub created_by: String,
    pub is_public: bool,
    pub cards: Vec<String>,
}
