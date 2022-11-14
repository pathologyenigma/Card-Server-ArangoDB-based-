mod register;
use async_graphql::MergedObject;


#[derive(Default, MergedObject)]
pub struct UserMutationRoot(
    pub register::RegisterMutation,
);
