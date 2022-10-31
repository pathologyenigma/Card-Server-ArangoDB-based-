#![allow(clippy::all, warnings)]
pub struct GetUserByEmailOrUsername;
pub mod get_user_by_email_or_username {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetUserByEmailOrUsername";
    pub const QUERY : & str = "query GetUserByEmailOrUsername($account: String) {\r\n  users(where: {OR: [{email: $account},{username: $account}]}) {\r\n    id\r\n    username\r\n    password\r\n  }\r\n}" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables {
        pub account: Option<String>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Vec<GetUserByEmailOrUsernameUsers>,
    }
    #[derive(Deserialize)]
    pub struct GetUserByEmailOrUsernameUsers {
        pub id: ID,
        pub username: String,
        pub password: String,
    }
}
impl graphql_client::GraphQLQuery for GetUserByEmailOrUsername {
    type Variables = get_user_by_email_or_username::Variables;
    type ResponseData = get_user_by_email_or_username::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_user_by_email_or_username::QUERY,
            operation_name: get_user_by_email_or_username::OPERATION_NAME,
        }
    }
}

pub mod prelude {
    pub use super::GetUserByEmailOrUsername as Req;
    pub use super::get_user_by_email_or_username::Variables;
    pub use super::get_user_by_email_or_username::ResponseData;
}