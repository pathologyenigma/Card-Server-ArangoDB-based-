mod log_in;
use async_graphql::MergedObject;


#[derive(Default, MergedObject)]
pub struct UserQueryRoot(
    pub log_in::LogInQuery,
);
