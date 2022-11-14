use async_graphql::MergedObject;

mod create_collection;
#[derive(Default, MergedObject)]
pub struct CollectionMutationRoot(pub create_collection::CreateCollectionMutation);