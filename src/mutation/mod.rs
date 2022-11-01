use async_graphql::MergedObject;

pub mod user;
pub mod collection;
#[derive(MergedObject, Default)]
pub struct MutationRoot(pub user::UserMutation);