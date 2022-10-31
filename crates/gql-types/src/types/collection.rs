//! collection is a container created by user
//! one user can create multiple collections
//! user can add card into collection
//! card pool generation will based on collections
//! there could be only one refrence to a card in a collection

use async_graphql::{SimpleObject, InputObject, OneofObject};
#[derive(SimpleObject)]
/// the collection type definition
/// it is directly connected with user
/// each collection will have a unique id
/// and the name is unique for the same user
pub struct Collection {
    pub id: super::common::UUID,
    pub name: String,
    /// public collection could be access from anyone
    pub is_public: bool,
    pub cards: Vec<super::card::Card>,
    /// collection is owned only by one user
    /// and could not be duplicate or reference
    /// will be destoryed when the user's account is destroyed
    pub owned_by: super::user::User,
}
#[derive(InputObject)]
/// you only need to provide name and is public to create a collection
pub struct CollectionCreateInput {
    pub name: String,
    pub is_public: bool,
    /// you can provide cards when creating a collection
    /// but most of time you don't
    pub cards: Option<Vec<super::card::CardCreateInput>>,
}
#[derive(InputObject)]
pub struct CollectionUpdateInput {
    pub id: super::common::UUID,
    pub name: String,
    pub is_public: bool,
    /// you can just update or create card right here or using the card's create/update api
    pub cards: Option<Vec<super::card::CardInput>>
}

#[derive(OneofObject)]
pub enum CollectionInput {
    Create(CollectionCreateInput),
    Update(CollectionUpdateInput),
    Remove(super::common::UUID),
}