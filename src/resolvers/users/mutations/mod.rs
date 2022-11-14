use std::str::FromStr;

use super::UserMutationRoot;
use aragog::DatabaseConnection;
use async_graphql::{Context, Object, Result};

use tracing::info;

#[Object]
impl UserMutationRoot {
    pub async fn register(
        &self,
        ctx: &Context<'_>,
        input: gql_types::user::RegisterInput,
    ) -> Result<gql_types::common::JWT> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let ogm_res = ogm::services::prelude::new_user(
            ogm::models::prelude::User {
                username: input.username.clone(),
                password: crate::utils::hash(&input.password).expect("failed to hash password"),
                email: input.email,
                registered_at: gql_types::common::DateTimeForGQL::now().0.timestamp() as u64,
            },
            db,
        )
        .await;
        match ogm_res {
            Ok(id) => {
                info!("{}", id);
                return Ok(gql_types::common::JWT(Some(gql_types::common::TokenData {
                    id: gql_types::common::ID::from_str(&id).unwrap(),
                    username: input.username.clone(),
                    exp: crate::utils::default_exp_for_token(),
                })));
            }
            Err(err) => match err {
                ogm::services::prelude::OGMSeviceError::Internal(msg) => {
                    return Err(crate::utils::new_internal_server_error(msg))
                }
                ogm::services::prelude::OGMSeviceError::NotFound(_, _) => {
                    return Err(crate::utils::new_internal_server_error(
                        "unknown error".to_owned(),
                    ))
                }
                ogm::services::prelude::OGMSeviceError::Conflict(msg) => {
                    return Err(crate::utils::new_conflict_error(msg))
                }
            },
        };
    }
}
