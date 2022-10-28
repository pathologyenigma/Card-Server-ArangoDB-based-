use async_graphql::MergedObject;

pub mod user;
#[derive(MergedObject, Default)]
pub struct MutationRoot(pub user::UserMutation);