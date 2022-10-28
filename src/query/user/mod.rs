use async_graphql::{Object, Result, Context};
#[derive(Default)]
pub struct UserQuery;
#[Object]
impl UserQuery {
    pub async fn log_in(&self, _ctx: &Context<'_>, _input: gql_types::user::RegisterInput) -> Result<gql_types::common::JWT> {
        todo!()
    }
}
