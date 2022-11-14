use async_graphql::{MergedSubscription};
pub mod mutations;
pub mod queries;
pub mod subscriptions;
pub use queries::UserQueryRoot;
pub use mutations::UserMutationRoot;

#[derive(MergedSubscription)]
pub struct UserSubscriptionRoot();
