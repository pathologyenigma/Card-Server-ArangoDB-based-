use std::str::FromStr;

use async_graphql::{Context, Object, Result};
use ogm_handler::gql::{
    json_to_vars,
    querys::{get_user_by_email_or_username::Variables, GetUserByEmailOrUsername},
};

use crate::{utils::ErrorHandlerWithErrorExtensions, OGM_URL};
#[derive(Default)]
pub struct UserQuery;
#[Object]
impl UserQuery {
    pub async fn log_in(
        &self,
        _ctx: &Context<'_>,
        input: gql_types::user::LogInInput,
    ) -> Result<gql_types::common::JWT> {
        let json_arg = format!("{{\"account\": {}}}", &input.account);
        let ogm_res = ogm_handler::Handler::query::<GetUserByEmailOrUsername>(
            OGM_URL,
            json_to_vars::<Variables>(&json_arg).expect("Invalid json"),
        )
        .await
        .expect("failed to fetch data from OGM");
        if ogm_res.errors.is_some() {
            return Err(crate::utils::new_internal_server_error(format!(
                "error from ogm: {:?}",
                ogm_res.errors
            )));
        }
        let data = ogm_res.data.unwrap();
        if data.users.len() == 0 {
            return Err(crate::utils::new_not_found_error(
                "user not exist".to_owned(),
            ));
        }
        if let Ok(_) = crate::utils::verify(&input.password, &data.users[0].password) {
            return Ok(gql_types::common::JWT(gql_types::common::TokenData {
                id: gql_types::common::UUID::from_str(&data.users[0].id).unwrap(),
                username: data.users[0].username.clone(),
            }));
        } else {
            return Err(crate::utils::BadInputErrorHandler::default()
                .append("password".to_owned(), "incorrect password".to_owned())
                .to_err());
        }
    }
}
