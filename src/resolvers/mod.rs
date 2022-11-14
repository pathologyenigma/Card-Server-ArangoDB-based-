use async_graphql::{MergedObject, MergedSubscription};

use users::UserQueryRoot;
use users::UserMutationRoot;
use collections::CollectionMutationRoot;
pub mod users;
pub mod collections;
#[derive(MergedObject, Default)]
pub struct QueryRoot(pub UserQueryRoot);

#[derive(MergedObject, Default)]
pub struct MutationRoot(pub UserMutationRoot, pub CollectionMutationRoot);

#[derive(MergedSubscription)]
pub struct SubscriptionRoot();