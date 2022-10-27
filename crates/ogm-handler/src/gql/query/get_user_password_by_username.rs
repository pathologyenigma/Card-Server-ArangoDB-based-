#![allow(clippy::all, warnings)]
pub struct GetUserPasswordByUsername;
pub mod get_user_password_by_username {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetUserPasswordByUsername";
    pub const QUERY : & str = "query GetUserPasswordByUsername($username: String) {\r\n  users(where: {username: $username}) {\r\n    password\r\n  }\r\n}" ;
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
        pub username: Option<String>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Vec<GetUserPasswordByUsernameUsers>,
    }
    #[derive(Deserialize)]
    pub struct GetUserPasswordByUsernameUsers {
        pub password: String,
    }
}
impl graphql_client::GraphQLQuery for GetUserPasswordByUsername {
    type Variables = get_user_password_by_username::Variables;
    type ResponseData = get_user_password_by_username::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_user_password_by_username::QUERY,
            operation_name: get_user_password_by_username::OPERATION_NAME,
        }
    }
}
