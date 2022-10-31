//! card is the core of the project
//! it could be just a name and description
//! it could have image and even spritesheet
//! also it could have effects following some set-up rule(WIP)

use async_graphql::{SimpleObject, OneofObject, InputObject};
#[derive(SimpleObject)]
/// the card type definition
/// every card is unique by the card id
/// you can duplicate a card or refrence it directly
pub struct Card {
    pub id: super::common::UUID,
    pub name: String,
    pub description: Option<String>,
    /// what this card is looks like, could be just a image or spritesheet or live2d
    /// could be empty
    /// in this early version, this project might not support sprites or live2d
    /// and still finding a way to support those things at server-side
    pub view: Option<super::common::URL>,
    /// effects are something to playable
    /// like attack or defense
    /// and special effects like kill a unit or draw a card
    /// just like the card games you played
    /// but in this version
    /// the effects is not settled yet
    /// still thinking about how to set up the input rule
    /// and makes the rule to support multiple languages
    pub effects: Option<Vec<String>>,
    /// if the card is duplicate from another card
    /// this will be the uuid of the original card
    /// because the original card may not always be available
    /// so this will not returns a card reference
    pub duplicate_from: Option<super::common::UUID>,
    /// a public card could be access by anyone
    /// could be duplicate by anyone
    /// could be reference by anyone
    /// but the private card could only be seen by others
    pub is_public: bool,
    /// this representing the creator of the card
    /// none only because the creator's account is destoryed and this is duplicate one
    pub created_by: Option<super::user::User>,
}

#[derive(InputObject)]
pub struct CardCreateInput {
    pub name: String,
    /// the description is not required
    /// but if you create a card without any effects
    /// that you better tells others about your card
    /// but you can still choose not to add this
    pub description: Option<String>,
    pub view: Option<CardViewInput>,
    pub effects: Option<Vec<String>>,
    pub duplicate_from: Option<super::common::UUID>,
    pub is_public: bool,
}
#[derive(InputObject)]
pub struct CardUpdateInput {
    pub id: super::common::UUID,
    pub name: Option<String>,
    /// the description is not required
    /// but if you create a card without any effects
    /// that you better tells others about your card
    /// but you can still choose not to add this
    pub description: Option<String>,
    pub view: Option<CardViewInput>,
    pub effects: Option<Vec<String>>,
    pub duplicate_from: Option<super::common::UUID>,
    pub is_public: Option<bool>,
}
#[derive(OneofObject)]
pub enum CardViewInput {
    StaticImage(super::common::URL),
    /// not implemented yet
    /// the String might not be the final argument for this
    /// spritesheet need client side support
    SpriteSheet(String),
    /// not implemented yet
    /// the String might not be the final argument for this
    /// live2d also need client side support
    Live2D(String),
}
#[derive(OneofObject)]
pub enum CardInput {
    Create(CardCreateInput),
    Update(CardUpdateInput),
    Remove(super::common::UUID)
}