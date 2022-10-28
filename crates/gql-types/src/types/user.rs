use async_graphql::{SimpleObject, InputObject};

#[derive(SimpleObject)]
pub struct User {
    id: super::common::UUID,
    username: String,
    password: String,
    email: Option<super::common::Email>,
    phone_email: super::common::PhoneNumber,
    image_url: Option<super::common::URL>,
    register_at: super::common::DateTimeForGQL,
}
#[derive(InputObject)]
pub struct RegisterInput {
    #[graphql(validator(min_length = 5, max_length = 10))]
    username: String,
    #[graphql(validator(min_length = 5, max_length = 5))]
    verify_code: String,
    #[graphql(validator(min_length = 1))]
    phone_number: String,
}

#[derive(InputObject)]
pub struct LogInInput {
    #[graphql(validator(min_length = 1))]
    account: String,
    #[graphql(validator(min_length = 1))]
    verify: String,
    is_verify_code: bool,
}