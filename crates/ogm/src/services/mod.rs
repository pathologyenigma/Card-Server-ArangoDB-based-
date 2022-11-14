use std::fmt;

mod users;
#[derive(Debug)]
pub enum Error {
    Internal(String),
    NotFound(String, String),
    Conflict(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Error::Internal(msg) => {
                format!("{}", msg)
            }
            Error::NotFound(field_name, field_value) => {
                format!("{} {} is not found", field_name, field_value)
            }
            Error::Conflict(msg) => {
                format!("{}", msg)
            }
        };
        write!(f, "{}", msg)
    }
}
pub mod prelude {
    pub use super::users::*;
    pub use super::Error as OGMSeviceError;
}