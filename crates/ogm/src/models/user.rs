use aragog::Record;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Record, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
    pub registered_at: u64,
}