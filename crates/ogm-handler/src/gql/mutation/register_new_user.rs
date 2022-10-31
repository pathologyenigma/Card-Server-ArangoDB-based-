#![allow(clippy::all, warnings)]
pub struct RegisterNewUser;
pub mod register_new_user {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "RegisterNewUser";
    pub const QUERY : & str = "mutation RegisterNewUser($username: String!, $email: String!, $password: String!, $register_at: String!, $phone_number: String) {\r\n  createUsers(\r\n    input: {username: $username, password: $password, email: $email, register_at: $register_at, phone_number: $phone_number}\r\n  ) {\r\n    info {\r\n      nodesCreated\r\n    }\r\n    users {\r\n      id\r\n    }\r\n  }\r\n}" ;
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
        pub username: String,
        pub email: String,
        pub password: String,
        pub phone_number: Option<String>,
        pub register_at: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "createUsers")]
        pub create_users: RegisterNewUserCreateUsers,
    }
    #[derive(Deserialize)]
    pub struct RegisterNewUserCreateUsers {
        pub info: RegisterNewUserCreateUsersInfo,
        pub users: Vec<RegisterNewUserCreateUsersUsers>,
    }
    #[derive(Deserialize)]
    pub struct RegisterNewUserCreateUsersInfo {
        #[serde(rename = "nodesCreated")]
        pub nodes_created: Int,
    }
    #[derive(Deserialize)]
    pub struct RegisterNewUserCreateUsersUsers {
        pub id: Option<ID>,
    }
}
impl graphql_client::GraphQLQuery for RegisterNewUser {
    type Variables = register_new_user::Variables;
    type ResponseData = register_new_user::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: register_new_user::QUERY,
            operation_name: register_new_user::OPERATION_NAME,
        }
    }
}
pub mod prelude {
    pub use super::RegisterNewUser as Req;
    pub use super::register_new_user::Variables;
    pub use super::register_new_user::ResponseData;
}