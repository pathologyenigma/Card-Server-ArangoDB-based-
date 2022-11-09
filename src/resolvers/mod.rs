use async_graphql::{MergedObject, MergedSubscription};

use users::UserQueryRoot;
use users::UserMutationRoot;
pub mod users;

#[derive(MergedObject, Default)]
pub struct QueryRoot(pub UserQueryRoot);

#[derive(MergedObject, Default)]
pub struct MutationRoot(pub UserMutationRoot);

#[derive(MergedSubscription)]
pub struct SubscriptionRoot();