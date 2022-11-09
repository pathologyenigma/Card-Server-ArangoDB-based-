pub mod resolvers;

pub mod utils {
    use pbkdf2::{
        password_hash::{
            rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, Result, SaltString,
        },
        Pbkdf2,
    };

    pub fn hash(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        Ok(Pbkdf2
            .hash_password(password.as_bytes(), &salt)?
            .to_string())
    }

    pub fn verify(password: &String, hashed_password: &String) -> Result<()> {
        Pbkdf2.verify_password(password.as_bytes(), &PasswordHash::new(&hashed_password)?)?;
        Ok(())
    }
    pub use async_graphql::{Error, ErrorExtensions};
    pub trait ErrorHandlerWithErrorExtensions {
        type ErrorType;
        fn append(self, name: String, value: String) -> Self;
        fn to_err(self) -> Self::ErrorType;
    }

    #[derive(Clone)]
    pub struct BadInputErrorHandler {
        errors: Option<Error>,
    }

    impl ErrorHandlerWithErrorExtensions for BadInputErrorHandler {
        type ErrorType = Error;

        fn append(mut self, name: String, value: String) -> Self{
            self.errors = Some(
                self.errors
                    .clone()
                    .unwrap_or(Error::new("400 Bad Input"))
                    .extend_with(|_, e| e.set(name, value)),
            );
            self
        }

        fn to_err(self) -> Self::ErrorType {
            self.errors.unwrap()
        }
    }

    impl Default for BadInputErrorHandler {
        fn default() -> Self {
            Self { errors: None }
        }
    }
    impl BadInputErrorHandler {
        pub fn is_none(&self) -> bool {
            self.errors.is_none()
        }
    }
    pub fn new_not_authenticated_error(msg: String) -> Error {
        Error::new("401 Not Authenticated").extend_with(|_, e| e.set("token", msg))
    }

    pub fn new_internal_server_error(msg: String) -> Error {
        Error::new("500 Internal Server Error").extend_with(|_, e| e.set("err_msg", msg))
    }
    pub fn new_not_found_error(msg: String) -> Error {
        Error::new("404 Not Found").extend_with(|_, e| e.set("err_msg", msg))
    }
    pub fn new_conflict_error(msg: String) -> Error {
        Error::new("409 Conflict").extend_with(|_, e|e.set("err_msg", msg))
    }
    pub fn default_exp_for_token() -> usize {
        std::time::SystemTime::now()
        .checked_add(std::time::Duration::from_secs(3600))
        .unwrap()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("failed to generate exp")
        .as_secs() as usize
    }
    
}
pub use ogm::set_up_database;

pub use gql_types::common::TokenData;
pub use gql_types::common::JWT;