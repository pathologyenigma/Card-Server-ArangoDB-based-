use async_graphql::{Object, Result, Context};
#[derive(Default)]
pub struct UserMutation;
#[Object]
impl UserMutation {
    pub async fn register(&self, _ctx: &Context<'_>, _input: gql_types::user::RegisterInput) -> Result<gql_types::common::JWT> {
        todo!()
    }
}