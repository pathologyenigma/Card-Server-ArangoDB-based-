pub mod user;
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct QueryRoot(pub user::UserQuery);