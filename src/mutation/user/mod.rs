use std::str::FromStr;

use async_graphql::{Context, Object, Result};
use gql_types::common::TokenData;

use crate::OGM_URL;
#[derive(Default)]
pub struct UserMutation;
#[Object]
impl UserMutation {
    pub async fn register(
        &self,
        _ctx: &Context<'_>,
        input: gql_types::user::RegisterInput,
    ) -> Result<gql_types::common::JWT> {
        let ogm_res =
            ogm_handler::Handler::query::<ogm_handler::gql::querys::RegisterNewUser>(
                OGM_URL,
                ogm_handler::gql::json_to_vars::<
                    ogm_handler::gql::querys::register_new_user::Variables,
                >(&serde_json::to_string(&input).expect("Invalid json"))
                .expect("Invalid json")
            )
            .await
            .expect("failed to query ogm");
        if ogm_res.errors.is_some() {
            let errors = ogm_res.errors.unwrap();
            if errors[0].message.contains("Constraint validation failed") {
                return Err(crate::utils::new_conflict_error(
                    "username phone number or email is taken".to_owned(),
                ));
            }
            return Err(crate::utils::new_internal_server_error(format!(
                "error from ogm: {:?}",
                errors
            )));
        }
        if let Some(res) = ogm_res.data {
            let id = &res.create_users.users[0].id;
            return Ok(gql_types::common::JWT(TokenData {
                id: gql_types::common::UUID::from_str(&id)
                    .expect("failed to parse the id from database"),
                username: input.username,
            }));
        } else {
            return Err(crate::utils::new_internal_server_error(
                "unknown error".to_owned(),
            ));
        }
    }
}
