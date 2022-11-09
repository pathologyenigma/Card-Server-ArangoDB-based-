use async_graphql::{MergedSubscription};
pub mod mutations;
pub mod queries;
pub mod subscriptions;

#[derive(Default)]
pub struct UserQueryRoot;

#[derive(Default)]
pub struct UserMutationRoot;

#[derive(MergedSubscription)]
pub struct UserSubscriptionRoot();
