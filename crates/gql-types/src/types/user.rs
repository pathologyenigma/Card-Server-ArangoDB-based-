use async_graphql::{SimpleObject, InputObject};
use serde::Serialize;

#[derive(SimpleObject)]
pub struct User {
    pub id: super::common::ID,
    pub username: String,
    pub password: String,
    pub email: super::common::Email,
    pub phone_email: Option<super::common::PhoneNumber>,
    pub image_url: Option<super::common::URL>,
    pub register_at: super::common::DateTimeForGQL,
}
#[derive(InputObject, Serialize)]
pub struct RegisterInput {
    #[graphql(validator(min_length = 5, max_length = 10))]
    pub username: String,
    #[graphql(validator(min_length = 8, max_length = 16))]
    pub password: String,
    #[graphql(validator(email))]
    pub email: String,
    pub phone_number: Option<super::common::PhoneNumber>,
}

#[derive(InputObject, Serialize)]
pub struct LogInInput {
    #[graphql(validator(min_length = 1))]
    /// could be an email address or your username
    pub account: String,
    #[graphql(validator(min_length = 1))]
    pub password: String,
}