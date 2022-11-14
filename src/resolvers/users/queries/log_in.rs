
use crate::utils::ErrorHandlerWithErrorExtensions;
#[derive(Default)]
pub struct LogInQuery;

use aragog::DatabaseConnection;
use async_graphql::{Context, Object, Result};

#[Object]
impl LogInQuery {
    pub async fn log_in(
        &self,
        ctx: &Context<'_>,
        input: gql_types::user::LogInInput,
    ) -> Result<gql_types::common::JWT> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let ogm_res =
            ogm::services::prelude::get_user_by_username_or_email(&input.account, db).await;
        match ogm_res {
            Ok((id, user)) => {
                if let Ok(_) = crate::utils::verify(&input.password, &user.password) {
                    return Ok(gql_types::common::JWT(Some(crate::TokenData {
                        id: gql_types::common::ID(id),
                        username: user.username.clone(),
                        exp: crate::utils::default_exp_for_token(),
                    })));
                } else {
                    return Err(crate::utils::BadInputErrorHandler::default()
                        .append("password".to_owned(), "incorrect password".to_owned())
                        .to_err());
                }
            }
            Err(err) => match err {
                ogm::services::prelude::OGMSeviceError::Internal(msg) => {
                    return Err(crate::utils::new_internal_server_error(msg))
                }
                ogm::services::prelude::OGMSeviceError::NotFound(_, _) => {
                    return Err(crate::utils::new_not_found_error(format!("{}", err)))
                }
                ogm::services::prelude::OGMSeviceError::Conflict(_) => {
                    return Err(crate::utils::new_conflict_error("unkonwn error".to_owned()))
                }
            },
        };
    }
}
