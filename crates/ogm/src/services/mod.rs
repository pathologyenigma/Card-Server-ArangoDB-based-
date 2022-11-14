use std::fmt;

mod users;
mod collections;
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
impl Error {
    pub fn from_aragog_error(query_or_save: bool, aragog_err: aragog::Error) -> Self {
        if query_or_save {
            match aragog_err {
                aragog::Error::ArangoError(err) => match err.arango_error {
                    aragog::error::ArangoError::ArangoDataSourceNotFound => {
                        return Error::Internal("Table Not exist".to_owned());
                    }
                    _ => return Error::Internal(format!("{}", err)),
                },
                _ => {
                    return Error::Internal(format!("{}", aragog_err));
                }
            }
        } else {
            match aragog_err {
                aragog::Error::Conflict(err) => match err.arango_error {
                    aragog::error::ArangoError::ArangoUniqueConstraintViolated => {
                        return Error::Conflict(err.message)
                    }
                    _ => return Error::Internal(err.message),
                },
                _ => {
                    return Error::Internal(format!("{}", aragog_err));
                }
            }
        }
    }
}
pub mod prelude {
    pub use super::users::*;
    pub use super::collections::*;
    pub use super::Error as OGMSeviceError;
}