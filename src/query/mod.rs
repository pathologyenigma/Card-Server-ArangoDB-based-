pub mod user;
pub mod candidate;
pub mod enterprise;
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct QueryRoot(pub user::UserQuery);