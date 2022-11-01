#![allow(clippy::all, warnings)]
pub struct CreateCollection;
pub mod create_collection {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CreateCollection";
    pub const QUERY : & str = "mutation CreateCollection($name: String!,\r\n $is_public: Boolean!,\r\n $cards: CollectionCardsFieldInput,\r\n $owned_by: CollectionOwned_byFieldInput) {\r\n  createCollections(input: [{\r\n    name: $name,\r\n    is_public: $is_public,\r\n    cards: $cards,\r\n    owned_by: $owned_by\r\n  }]) {\r\n    info {\r\n      nodesCreated\r\n    }\r\n    collections {\r\n      id\r\n    }\r\n  }\r\n}\r\nmutation RegisterNewUser(\r\n  $username: String!\r\n  $email: String!\r\n  $password: String!\r\n  $register_at: String!\r\n  $phone_number: String\r\n) {\r\n  createUsers(\r\n    input: {\r\n      username: $username\r\n      password: $password\r\n      email: $email\r\n      register_at: $register_at\r\n      phone_number: $phone_number\r\n    }\r\n  ) {\r\n    info {\r\n      nodesCreated\r\n    }\r\n    users {\r\n      id\r\n    }\r\n  }\r\n}\r\nquery GetUserByEmailOrUsername($account: String) {\r\n  users(where: {OR: [{email: $account},{username: $account}]}) {    id    username    password  }}" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct CardConnectInput {
        pub created_by: Option<CardCreated_byConnectFieldInput>,
    }
    #[derive(Serialize)]
    pub struct CardConnectOrCreateWhere {
        pub node: CardUniqueWhere,
    }
    #[derive(Serialize)]
    pub struct CardConnectWhere {
        pub node: Box<CardWhere>,
    }
    #[derive(Serialize)]
    pub struct CardCreated_byAggregateInput {
        pub count: Option<Int>,
        #[serde(rename = "count_LT")]
        pub count_lt: Option<Int>,
        #[serde(rename = "count_LTE")]
        pub count_lte: Option<Int>,
        #[serde(rename = "count_GT")]
        pub count_gt: Option<Int>,
        #[serde(rename = "count_GTE")]
        pub count_gte: Option<Int>,
        #[serde(rename = "AND")]
        pub and: Option<Vec<CardCreated_byAggregateInput>>,
        #[serde(rename = "OR")]
        pub or: Option<Vec<CardCreated_byAggregateInput>>,
        pub node: Option<CardCreated_byNodeAggregationWhereInput>,
    }
    #[derive(Serialize)]
    pub struct CardCreated_byConnectFieldInput {
        #[serde(rename = "where")]
        pub where_: Option<UserConnectWhere>,
        pub connect: Option<UserConnectInput>,
    }
    #[derive(Serialize)]
    pub struct CardCreated_byConnectionWhere {
        #[serde(rename = "AND")]
        pub and: Box<Option<Vec<CardCreated_byConnectionWhere>>>,
        #[serde(rename = "OR")]
        pub or: Box<Option<Vec<CardCreated_byConnectionWhere>>>,
        pub node: Box<Option<UserWhere>>,
        #[serde(rename = "node_NOT")]
        pub node_not: Box<Option<UserWhere>>,
    }
    #[derive(Serialize)]
    pub struct CardCreated_byConnectOrCreateFieldInput {
        #[serde(rename = "where")]
        pub where_: UserConnectOrCreateWhere,
        #[serde(rename = "onCreate")]
        pub on_create: CardCreated_byConnectOrCreateFieldInputOnCreate,
    }
    #[derive(Serialize)]
    pub struct CardCreated_byConnectOrCreateFieldInputOnCreate {
        pub node: UserOnCreateInput,
    }
    #[derive(Serialize)]
    pub struct CardCreated_byCreateFieldInput {
        pub node: UserCreateInput,
    }
    #[derive(Serialize)]
    pub struct CardCreated_byFieldInput {
        pub create: Option<CardCreated_byCreateFieldInput>,
        pub connect: Option<CardCreated_byConnectFieldInput>,
        #[serde(rename = "connectOrCreate")]
        pub connect_or_create: Option<CardCreated_byConnectOrCreateFieldInput>,
    }
    #[derive(Serialize)]
    pub struct CardCreated_byNodeAggregationWhereInput {
        #[serde(rename = "AND")]
        pub and: Option<Vec<CardCreated_byNodeAggregationWhereInput>>,
        #[serde(rename = "OR")]
        pub or: Option<Vec<CardCreated_byNodeAggregationWhereInput>>,
        #[serde(rename = "id_EQUAL")]
        pub id_equal: Option<ID>,
        #[serde(rename = "username_EQUAL")]
        pub username_equal: Option<String>,
        #[serde(rename = "username_AVERAGE_EQUAL")]
        pub username_average_equal: Option<Float>,
        #[serde(rename = "username_LONGEST_EQUAL")]
        pub username_longest_equal: Option<Int>,
        #[serde(rename = "username_SHORTEST_EQUAL")]
        pub username_shortest_equal: Option<Int>,
        #[serde(rename = "username_GT")]
        pub username_gt: Option<Int>,
        #[serde(rename = "username_AVERAGE_GT")]
        pub username_average_gt: Option<Float>,
        #[serde(rename = "username_LONGEST_GT")]
        pub username_longest_gt: Option<Int>,
        #[serde(rename = "username_SHORTEST_GT")]
        pub username_shortest_gt: Option<Int>,
        #[serde(rename = "username_GTE")]
        pub username_gte: Option<Int>,
        #[serde(rename = "username_AVERAGE_GTE")]
        pub username_average_gte: Option<Float>,
        #[serde(rename = "username_LONGEST_GTE")]
        pub username_longest_gte: Option<Int>,
        #[serde(rename = "username_SHORTEST_GTE")]
        pub username_shortest_gte: Option<Int>,
        #[serde(rename = "username_LT")]
        pub username_lt: Option<Int>,
        #[serde(rename = "username_AVERAGE_LT")]
        pub username_average_lt: Option<Float>,
        #[serde(rename = "username_LONGEST_LT")]
        pub username_longest_lt: Option<Int>,
        #[serde(rename = "username_SHORTEST_LT")]
        pub username_shortest_lt: Option<Int>,
        #[serde(rename = "username_LTE")]
        pub username_lte: Option<Int>,
        #[serde(rename = "username_AVERAGE_LTE")]
        pub username_average_lte: Option<Float>,
        #[serde(rename = "username_LONGEST_LTE")]
        pub username_longest_lte: Option<Int>,
        #[serde(rename = "username_SHORTEST_LTE")]
        pub username_shortest_lte: Option<Int>,
        #[serde(rename = "email_EQUAL")]
        pub email_equal: Option<String>,
        #[serde(rename = "email_AVERAGE_EQUAL")]
        pub email_average_equal: Option<Float>,
        #[serde(rename = "email_LONGEST_EQUAL")]
        pub email_longest_equal: Option<Int>,
        #[serde(rename = "email_SHORTEST_EQUAL")]
        pub email_shortest_equal: Option<Int>,
        #[serde(rename = "email_GT")]
        pub email_gt: Option<Int>,
        #[serde(rename = "email_AVERAGE_GT")]
        pub email_average_gt: Option<Float>,
        #[serde(rename = "email_LONGEST_GT")]
        pub email_longest_gt: Option<Int>,
        #[serde(rename = "email_SHORTEST_GT")]
        pub email_shortest_gt: Option<Int>,
        #[serde(rename = "email_GTE")]
        pub email_gte: Option<Int>,
        #[serde(rename = "email_AVERAGE_GTE")]
        pub email_average_gte: Option<Float>,
        #[serde(rename = "email_LONGEST_GTE")]
        pub email_longest_gte: Option<Int>,
        #[serde(rename = "email_SHORTEST_GTE")]
        pub email_shortest_gte: Option<Int>,
        #[serde(rename = "email_LT")]
        pub email_lt: Option<Int>,
        #[serde(rename = "email_AVERAGE_LT")]
        pub email_average_lt: Option<Float>,
        #[serde(rename = "email_LONGEST_LT")]
        pub email_longest_lt: Option<Int>,
        #[serde(rename = "email_SHORTEST_LT")]
        pub email_shortest_lt: Option<Int>,
        #[serde(rename = "email_LTE")]
        pub email_lte: Option<Int>,
        #[serde(rename = "email_AVERAGE_LTE")]
        pub email_average_lte: Option<Float>,
        #[serde(rename = "email_LONGEST_LTE")]
        pub email_longest_lte: Option<Int>,
        #[serde(rename = "email_SHORTEST_LTE")]
        pub email_shortest_lte: Option<Int>,
        #[serde(rename = "image_EQUAL")]
        pub image_equal: Option<String>,
        #[serde(rename = "image_AVERAGE_EQUAL")]
        pub image_average_equal: Option<Float>,
        #[serde(rename = "image_LONGEST_EQUAL")]
        pub image_longest_equal: Option<Int>,
        #[serde(rename = "image_SHORTEST_EQUAL")]
        pub image_shortest_equal: Option<Int>,
        #[serde(rename = "image_GT")]
        pub image_gt: Option<Int>,
        #[serde(rename = "image_AVERAGE_GT")]
        pub image_average_gt: Option<Float>,
        #[serde(rename = "image_LONGEST_GT")]
        pub image_longest_gt: Option<Int>,
        #[serde(rename = "image_SHORTEST_GT")]
        pub image_shortest_gt: Option<Int>,
        #[serde(rename = "image_GTE")]
        pub image_gte: Option<Int>,
        #[serde(rename = "image_AVERAGE_GTE")]
        pub image_average_gte: Option<Float>,
        #[serde(rename = "image_LONGEST_GTE")]
        pub image_longest_gte: Option<Int>,
        #[serde(rename = "image_SHORTEST_GTE")]
        pub image_shortest_gte: Option<Int>,
        #[serde(rename = "image_LT")]
        pub image_lt: Option<Int>,
        #[serde(rename = "image_AVERAGE_LT")]
        pub image_average_lt: Option<Float>,
        #[serde(rename = "image_LONGEST_LT")]
        pub image_longest_lt: Option<Int>,
        #[serde(rename = "image_SHORTEST_LT")]
        pub image_shortest_lt: Option<Int>,
        #[serde(rename = "image_LTE")]
        pub image_lte: Option<Int>,
        #[serde(rename = "image_AVERAGE_LTE")]
        pub image_average_lte: Option<Float>,
        #[serde(rename = "image_LONGEST_LTE")]
        pub image_longest_lte: Option<Int>,
        #[serde(rename = "image_SHORTEST_LTE")]
        pub image_shortest_lte: Option<Int>,
        #[serde(rename = "phone_number_EQUAL")]
        pub phone_number_equal: Option<String>,
        #[serde(rename = "phone_number_AVERAGE_EQUAL")]
        pub phone_number_average_equal: Option<Float>,
        #[serde(rename = "phone_number_LONGEST_EQUAL")]
        pub phone_number_longest_equal: Option<Int>,
        #[serde(rename = "phone_number_SHORTEST_EQUAL")]
        pub phone_number_shortest_equal: Option<Int>,
        #[serde(rename = "phone_number_GT")]
        pub phone_number_gt: Option<Int>,
        #[serde(rename = "phone_number_AVERAGE_GT")]
        pub phone_number_average_gt: Option<Float>,
        #[serde(rename = "phone_number_LONGEST_GT")]
        pub phone_number_longest_gt: Option<Int>,
        #[serde(rename = "phone_number_SHORTEST_GT")]
        pub phone_number_shortest_gt: Option<Int>,
        #[serde(rename = "phone_number_GTE")]
        pub phone_number_gte: Option<Int>,
        #[serde(rename = "phone_number_AVERAGE_GTE")]
        pub phone_number_average_gte: Option<Float>,
        #[serde(rename = "phone_number_LONGEST_GTE")]
        pub phone_number_longest_gte: Option<Int>,
        #[serde(rename = "phone_number_SHORTEST_GTE")]
        pub phone_number_shortest_gte: Option<Int>,
        #[serde(rename = "phone_number_LT")]
        pub phone_number_lt: Option<Int>,
        #[serde(rename = "phone_number_AVERAGE_LT")]
        pub phone_number_average_lt: Option<Float>,
        #[serde(rename = "phone_number_LONGEST_LT")]
        pub phone_number_longest_lt: Option<Int>,
        #[serde(rename = "phone_number_SHORTEST_LT")]
        pub phone_number_shortest_lt: Option<Int>,
        #[serde(rename = "phone_number_LTE")]
        pub phone_number_lte: Option<Int>,
        #[serde(rename = "phone_number_AVERAGE_LTE")]
        pub phone_number_average_lte: Option<Float>,
        #[serde(rename = "phone_number_LONGEST_LTE")]
        pub phone_number_longest_lte: Option<Int>,
        #[serde(rename = "phone_number_SHORTEST_LTE")]
        pub phone_number_shortest_lte: Option<Int>,
        #[serde(rename = "password_EQUAL")]
        pub password_equal: Option<String>,
        #[serde(rename = "password_AVERAGE_EQUAL")]
        pub password_average_equal: Option<Float>,
        #[serde(rename = "password_LONGEST_EQUAL")]
        pub password_longest_equal: Option<Int>,
        #[serde(rename = "password_SHORTEST_EQUAL")]
        pub password_shortest_equal: Option<Int>,
        #[serde(rename = "password_GT")]
        pub password_gt: Option<Int>,
        #[serde(rename = "password_AVERAGE_GT")]
        pub password_average_gt: Option<Float>,
        #[serde(rename = "password_LONGEST_GT")]
        pub password_longest_gt: Option<Int>,
        #[serde(rename = "password_SHORTEST_GT")]
        pub password_shortest_gt: Option<Int>,
        #[serde(rename = "password_GTE")]
        pub password_gte: Option<Int>,
        #[serde(rename = "password_AVERAGE_GTE")]
        pub password_average_gte: Option<Float>,
        #[serde(rename = "password_LONGEST_GTE")]
        pub password_longest_gte: Option<Int>,
        #[serde(rename = "password_SHORTEST_GTE")]
        pub password_shortest_gte: Option<Int>,
        #[serde(rename = "password_LT")]
        pub password_lt: Option<Int>,
        #[serde(rename = "password_AVERAGE_LT")]
        pub password_average_lt: Option<Float>,
        #[serde(rename = "password_LONGEST_LT")]
        pub password_longest_lt: Option<Int>,
        #[serde(rename = "password_SHORTEST_LT")]
        pub password_shortest_lt: Option<Int>,
        #[serde(rename = "password_LTE")]
        pub password_lte: Option<Int>,
        #[serde(rename = "password_AVERAGE_LTE")]
        pub password_average_lte: Option<Float>,
        #[serde(rename = "password_LONGEST_LTE")]
        pub password_longest_lte: Option<Int>,
        #[serde(rename = "password_SHORTEST_LTE")]
        pub password_shortest_lte: Option<Int>,
        #[serde(rename = "register_at_EQUAL")]
        pub register_at_equal: Option<String>,
        #[serde(rename = "register_at_AVERAGE_EQUAL")]
        pub register_at_average_equal: Option<Float>,
        #[serde(rename = "register_at_LONGEST_EQUAL")]
        pub register_at_longest_equal: Option<Int>,
        #[serde(rename = "register_at_SHORTEST_EQUAL")]
        pub register_at_shortest_equal: Option<Int>,
        #[serde(rename = "register_at_GT")]
        pub register_at_gt: Option<Int>,
        #[serde(rename = "register_at_AVERAGE_GT")]
        pub register_at_average_gt: Option<Float>,
        #[serde(rename = "register_at_LONGEST_GT")]
        pub register_at_longest_gt: Option<Int>,
        #[serde(rename = "register_at_SHORTEST_GT")]
        pub register_at_shortest_gt: Option<Int>,
        #[serde(rename = "register_at_GTE")]
        pub register_at_gte: Option<Int>,
        #[serde(rename = "register_at_AVERAGE_GTE")]
        pub register_at_average_gte: Option<Float>,
        #[serde(rename = "register_at_LONGEST_GTE")]
        pub register_at_longest_gte: Option<Int>,
        #[serde(rename = "register_at_SHORTEST_GTE")]
        pub register_at_shortest_gte: Option<Int>,
        #[serde(rename = "register_at_LT")]
        pub register_at_lt: Option<Int>,
        #[serde(rename = "register_at_AVERAGE_LT")]
        pub register_at_average_lt: Option<Float>,
        #[serde(rename = "register_at_LONGEST_LT")]
        pub register_at_longest_lt: Option<Int>,
        #[serde(rename = "register_at_SHORTEST_LT")]
        pub register_at_shortest_lt: Option<Int>,
        #[serde(rename = "register_at_LTE")]
        pub register_at_lte: Option<Int>,
        #[serde(rename = "register_at_AVERAGE_LTE")]
        pub register_at_average_lte: Option<Float>,
        #[serde(rename = "register_at_LONGEST_LTE")]
        pub register_at_longest_lte: Option<Int>,
        #[serde(rename = "register_at_SHORTEST_LTE")]
        pub register_at_shortest_lte: Option<Int>,
    }
    #[derive(Serialize)]
    pub struct CardCreateInput {
        pub name: String,
        pub description: Option<String>,
        pub view: Option<String>,
        pub effects: Option<Vec<Option<String>>>,
        pub duplicate_from: Option<ID>,
        pub is_public: Boolean,
        pub created_by: Option<CardCreated_byFieldInput>,
    }
    #[derive(Serialize)]
    pub struct CardOnCreateInput {
        pub name: String,
        pub description: Option<String>,
        pub view: Option<String>,
        pub effects: Option<Vec<Option<String>>>,
        pub duplicate_from: Option<ID>,
        pub is_public: Boolean,
    }
    #[derive(Serialize)]
    pub struct CardUniqueWhere {
        pub id: Option<ID>,
    }
    #[derive(Serialize)]
    pub struct CardWhere {
        #[serde(rename = "OR")]
        pub or: Box<Option<Vec<CardWhere>>>,
        #[serde(rename = "AND")]
        pub and: Box<Option<Vec<CardWhere>>>,
        pub id: Option<ID>,
        #[serde(rename = "id_NOT")]
        pub id_not: Option<ID>,
        #[serde(rename = "id_IN")]
        pub id_in: Option<Vec<ID>>,
        #[serde(rename = "id_NOT_IN")]
        pub id_not_in: Option<Vec<ID>>,
        #[serde(rename = "id_CONTAINS")]
        pub id_contains: Option<ID>,
        #[serde(rename = "id_NOT_CONTAINS")]
        pub id_not_contains: Option<ID>,
        #[serde(rename = "id_STARTS_WITH")]
        pub id_starts_with: Option<ID>,
        #[serde(rename = "id_NOT_STARTS_WITH")]
        pub id_not_starts_with: Option<ID>,
        #[serde(rename = "id_ENDS_WITH")]
        pub id_ends_with: Option<ID>,
        #[serde(rename = "id_NOT_ENDS_WITH")]
        pub id_not_ends_with: Option<ID>,
        pub name: Option<String>,
        #[serde(rename = "name_NOT")]
        pub name_not: Option<String>,
        #[serde(rename = "name_IN")]
        pub name_in: Option<Vec<String>>,
        #[serde(rename = "name_NOT_IN")]
        pub name_not_in: Option<Vec<String>>,
        #[serde(rename = "name_CONTAINS")]
        pub name_contains: Option<String>,
        #[serde(rename = "name_NOT_CONTAINS")]
        pub name_not_contains: Option<String>,
        #[serde(rename = "name_STARTS_WITH")]
        pub name_starts_with: Option<String>,
        #[serde(rename = "name_NOT_STARTS_WITH")]
        pub name_not_starts_with: Option<String>,
        #[serde(rename = "name_ENDS_WITH")]
        pub name_ends_with: Option<String>,
        #[serde(rename = "name_NOT_ENDS_WITH")]
        pub name_not_ends_with: Option<String>,
        pub description: Option<String>,
        #[serde(rename = "description_NOT")]
        pub description_not: Option<String>,
        #[serde(rename = "description_IN")]
        pub description_in: Option<Vec<Option<String>>>,
        #[serde(rename = "description_NOT_IN")]
        pub description_not_in: Option<Vec<Option<String>>>,
        #[serde(rename = "description_CONTAINS")]
        pub description_contains: Option<String>,
        #[serde(rename = "description_NOT_CONTAINS")]
        pub description_not_contains: Option<String>,
        #[serde(rename = "description_STARTS_WITH")]
        pub description_starts_with: Option<String>,
        #[serde(rename = "description_NOT_STARTS_WITH")]
        pub description_not_starts_with: Option<String>,
        #[serde(rename = "description_ENDS_WITH")]
        pub description_ends_with: Option<String>,
        #[serde(rename = "description_NOT_ENDS_WITH")]
        pub description_not_ends_with: Option<String>,
        pub view: Option<String>,
        #[serde(rename = "view_NOT")]
        pub view_not: Option<String>,
        #[serde(rename = "view_IN")]
        pub view_in: Option<Vec<Option<String>>>,
        #[serde(rename = "view_NOT_IN")]
        pub view_not_in: Option<Vec<Option<String>>>,
        #[serde(rename = "view_CONTAINS")]
        pub view_contains: Option<String>,
        #[serde(rename = "view_NOT_CONTAINS")]
        pub view_not_contains: Option<String>,
        #[serde(rename = "view_STARTS_WITH")]
        pub view_starts_with: Option<String>,
        #[serde(rename = "view_NOT_STARTS_WITH")]
        pub view_not_starts_with: Option<String>,
        #[serde(rename = "view_ENDS_WITH")]
        pub view_ends_with: Option<String>,
        #[serde(rename = "view_NOT_ENDS_WITH")]
        pub view_not_ends_with: Option<String>,
        pub effects: Option<Vec<Option<String>>>,
        #[serde(rename = "effects_NOT")]
        pub effects_not: Option<Vec<Option<String>>>,
        #[serde(rename = "effects_INCLUDES")]
        pub effects_includes: Option<String>,
        #[serde(rename = "effects_NOT_INCLUDES")]
        pub effects_not_includes: Option<String>,
        pub duplicate_from: Option<ID>,
        #[serde(rename = "duplicate_from_NOT")]
        pub duplicate_from_not: Option<ID>,
        #[serde(rename = "duplicate_from_IN")]
        pub duplicate_from_in: Option<Vec<Option<ID>>>,
        #[serde(rename = "duplicate_from_NOT_IN")]
        pub duplicate_from_not_in: Option<Vec<Option<ID>>>,
        #[serde(rename = "duplicate_from_CONTAINS")]
        pub duplicate_from_contains: Option<ID>,
        #[serde(rename = "duplicate_from_NOT_CONTAINS")]
        pub duplicate_from_not_contains: Option<ID>,
        #[serde(rename = "duplicate_from_STARTS_WITH")]
        pub duplicate_from_starts_with: Option<ID>,
        #[serde(rename = "duplicate_from_NOT_STARTS_WITH")]
        pub duplicate_from_not_starts_with: Option<ID>,
        #[serde(rename = "duplicate_from_ENDS_WITH")]
        pub duplicate_from_ends_with: Option<ID>,
        #[serde(rename = "duplicate_from_NOT_ENDS_WITH")]
        pub duplicate_from_not_ends_with: Option<ID>,
        pub is_public: Option<Boolean>,
        #[serde(rename = "is_public_NOT")]
        pub is_public_not: Option<Boolean>,
        pub created_by: Box<Option<UserWhere>>,
        #[serde(rename = "created_by_NOT")]
        pub created_by_not: Box<Option<UserWhere>>,
        #[serde(rename = "created_byAggregate")]
        pub created_by_aggregate: Option<CardCreated_byAggregateInput>,
        #[serde(rename = "created_byConnection")]
        pub created_by_connection: Box<Option<CardCreated_byConnectionWhere>>,
        #[serde(rename = "created_byConnection_NOT")]
        pub created_by_connection_not: Box<Option<CardCreated_byConnectionWhere>>,
    }
    #[derive(Serialize)]
    pub struct CollectionCardsAggregateInput {
        pub count: Option<Int>,
        #[serde(rename = "count_LT")]
        pub count_lt: Option<Int>,
        #[serde(rename = "count_LTE")]
        pub count_lte: Option<Int>,
        #[serde(rename = "count_GT")]
        pub count_gt: Option<Int>,
        #[serde(rename = "count_GTE")]
        pub count_gte: Option<Int>,
        #[serde(rename = "AND")]
        pub and: Option<Vec<CollectionCardsAggregateInput>>,
        #[serde(rename = "OR")]
        pub or: Option<Vec<CollectionCardsAggregateInput>>,
        pub node: Option<CollectionCardsNodeAggregationWhereInput>,
    }
    #[derive(Serialize)]
    pub struct CollectionCardsConnectFieldInput {
        #[serde(rename = "where")]
        pub where_: Option<CardConnectWhere>,
        pub connect: Option<Vec<CardConnectInput>>,
    }
    #[derive(Serialize)]
    pub struct CollectionCardsConnectionWhere {
        #[serde(rename = "AND")]
        pub and: Box<Option<Vec<CollectionCardsConnectionWhere>>>,
        #[serde(rename = "OR")]
        pub or: Box<Option<Vec<CollectionCardsConnectionWhere>>>,
        pub node: Box<Option<CardWhere>>,
        #[serde(rename = "node_NOT")]
        pub node_not: Box<Option<CardWhere>>,
    }
    #[derive(Serialize)]
    pub struct CollectionCardsConnectOrCreateFieldInput {
        #[serde(rename = "where")]
        pub where_: CardConnectOrCreateWhere,
        #[serde(rename = "onCreate")]
        pub on_create: CollectionCardsConnectOrCreateFieldInputOnCreate,
    }
    #[derive(Serialize)]
    pub struct CollectionCardsConnectOrCreateFieldInputOnCreate {
        pub node: CardOnCreateInput,
    }
    #[derive(Serialize)]
    pub struct CollectionCardsCreateFieldInput {
        pub node: CardCreateInput,
    }
    #[derive(Serialize)]
    pub struct CollectionCardsFieldInput {
        pub create: Option<Vec<CollectionCardsCreateFieldInput>>,
        pub connect: Option<Vec<CollectionCardsConnectFieldInput>>,
        #[serde(rename = "connectOrCreate")]
        pub connect_or_create: Option<Vec<CollectionCardsConnectOrCreateFieldInput>>,
    }
    #[derive(Serialize)]
    pub struct CollectionCardsNodeAggregationWhereInput {
        #[serde(rename = "AND")]
        pub and: Option<Vec<CollectionCardsNodeAggregationWhereInput>>,
        #[serde(rename = "OR")]
        pub or: Option<Vec<CollectionCardsNodeAggregationWhereInput>>,
        #[serde(rename = "id_EQUAL")]
        pub id_equal: Option<ID>,
        #[serde(rename = "duplicate_from_EQUAL")]
        pub duplicate_from_equal: Option<ID>,
        #[serde(rename = "name_EQUAL")]
        pub name_equal: Option<String>,
        #[serde(rename = "name_AVERAGE_EQUAL")]
        pub name_average_equal: Option<Float>,
        #[serde(rename = "name_LONGEST_EQUAL")]
        pub name_longest_equal: Option<Int>,
        #[serde(rename = "name_SHORTEST_EQUAL")]
        pub name_shortest_equal: Option<Int>,
        #[serde(rename = "name_GT")]
        pub name_gt: Option<Int>,
        #[serde(rename = "name_AVERAGE_GT")]
        pub name_average_gt: Option<Float>,
        #[serde(rename = "name_LONGEST_GT")]
        pub name_longest_gt: Option<Int>,
        #[serde(rename = "name_SHORTEST_GT")]
        pub name_shortest_gt: Option<Int>,
        #[serde(rename = "name_GTE")]
        pub name_gte: Option<Int>,
        #[serde(rename = "name_AVERAGE_GTE")]
        pub name_average_gte: Option<Float>,
        #[serde(rename = "name_LONGEST_GTE")]
        pub name_longest_gte: Option<Int>,
        #[serde(rename = "name_SHORTEST_GTE")]
        pub name_shortest_gte: Option<Int>,
        #[serde(rename = "name_LT")]
        pub name_lt: Option<Int>,
        #[serde(rename = "name_AVERAGE_LT")]
        pub name_average_lt: Option<Float>,
        #[serde(rename = "name_LONGEST_LT")]
        pub name_longest_lt: Option<Int>,
        #[serde(rename = "name_SHORTEST_LT")]
        pub name_shortest_lt: Option<Int>,
        #[serde(rename = "name_LTE")]
        pub name_lte: Option<Int>,
        #[serde(rename = "name_AVERAGE_LTE")]
        pub name_average_lte: Option<Float>,
        #[serde(rename = "name_LONGEST_LTE")]
        pub name_longest_lte: Option<Int>,
        #[serde(rename = "name_SHORTEST_LTE")]
        pub name_shortest_lte: Option<Int>,
        #[serde(rename = "description_EQUAL")]
        pub description_equal: Option<String>,
        #[serde(rename = "description_AVERAGE_EQUAL")]
        pub description_average_equal: Option<Float>,
        #[serde(rename = "description_LONGEST_EQUAL")]
        pub description_longest_equal: Option<Int>,
        #[serde(rename = "description_SHORTEST_EQUAL")]
        pub description_shortest_equal: Option<Int>,
        #[serde(rename = "description_GT")]
        pub description_gt: Option<Int>,
        #[serde(rename = "description_AVERAGE_GT")]
        pub description_average_gt: Option<Float>,
        #[serde(rename = "description_LONGEST_GT")]
        pub description_longest_gt: Option<Int>,
        #[serde(rename = "description_SHORTEST_GT")]
        pub description_shortest_gt: Option<Int>,
        #[serde(rename = "description_GTE")]
        pub description_gte: Option<Int>,
        #[serde(rename = "description_AVERAGE_GTE")]
        pub description_average_gte: Option<Float>,
        #[serde(rename = "description_LONGEST_GTE")]
        pub description_longest_gte: Option<Int>,
        #[serde(rename = "description_SHORTEST_GTE")]
        pub description_shortest_gte: Option<Int>,
        #[serde(rename = "description_LT")]
        pub description_lt: Option<Int>,
        #[serde(rename = "description_AVERAGE_LT")]
        pub description_average_lt: Option<Float>,
        #[serde(rename = "description_LONGEST_LT")]
        pub description_longest_lt: Option<Int>,
        #[serde(rename = "description_SHORTEST_LT")]
        pub description_shortest_lt: Option<Int>,
        #[serde(rename = "description_LTE")]
        pub description_lte: Option<Int>,
        #[serde(rename = "description_AVERAGE_LTE")]
        pub description_average_lte: Option<Float>,
        #[serde(rename = "description_LONGEST_LTE")]
        pub description_longest_lte: Option<Int>,
        #[serde(rename = "description_SHORTEST_LTE")]
        pub description_shortest_lte: Option<Int>,
        #[serde(rename = "view_EQUAL")]
        pub view_equal: Option<String>,
        #[serde(rename = "view_AVERAGE_EQUAL")]
        pub view_average_equal: Option<Float>,
        #[serde(rename = "view_LONGEST_EQUAL")]
        pub view_longest_equal: Option<Int>,
        #[serde(rename = "view_SHORTEST_EQUAL")]
        pub view_shortest_equal: Option<Int>,
        #[serde(rename = "view_GT")]
        pub view_gt: Option<Int>,
        #[serde(rename = "view_AVERAGE_GT")]
        pub view_average_gt: Option<Float>,
        #[serde(rename = "view_LONGEST_GT")]
        pub view_longest_gt: Option<Int>,
        #[serde(rename = "view_SHORTEST_GT")]
        pub view_shortest_gt: Option<Int>,
        #[serde(rename = "view_GTE")]
        pub view_gte: Option<Int>,
        #[serde(rename = "view_AVERAGE_GTE")]
        pub view_average_gte: Option<Float>,
        #[serde(rename = "view_LONGEST_GTE")]
        pub view_longest_gte: Option<Int>,
        #[serde(rename = "view_SHORTEST_GTE")]
        pub view_shortest_gte: Option<Int>,
        #[serde(rename = "view_LT")]
        pub view_lt: Option<Int>,
        #[serde(rename = "view_AVERAGE_LT")]
        pub view_average_lt: Option<Float>,
        #[serde(rename = "view_LONGEST_LT")]
        pub view_longest_lt: Option<Int>,
        #[serde(rename = "view_SHORTEST_LT")]
        pub view_shortest_lt: Option<Int>,
        #[serde(rename = "view_LTE")]
        pub view_lte: Option<Int>,
        #[serde(rename = "view_AVERAGE_LTE")]
        pub view_average_lte: Option<Float>,
        #[serde(rename = "view_LONGEST_LTE")]
        pub view_longest_lte: Option<Int>,
        #[serde(rename = "view_SHORTEST_LTE")]
        pub view_shortest_lte: Option<Int>,
    }
    #[derive(Serialize)]
    pub struct CollectionConnectInput {
        pub cards: Option<Vec<CollectionCardsConnectFieldInput>>,
        pub owned_by: Option<CollectionOwned_byConnectFieldInput>,
    }
    #[derive(Serialize)]
    pub struct CollectionConnectOrCreateWhere {
        pub node: CollectionUniqueWhere,
    }
    #[derive(Serialize)]
    pub struct CollectionConnectWhere {
        pub node: Box<CollectionWhere>,
    }
    #[derive(Serialize)]
    pub struct CollectionCreateInput {
        pub name: String,
        pub is_public: Boolean,
        pub cards: Option<CollectionCardsFieldInput>,
        pub owned_by: Option<CollectionOwned_byFieldInput>,
    }
    #[derive(Serialize)]
    pub struct CollectionOnCreateInput {
        pub name: String,
        pub is_public: Boolean,
    }
    #[derive(Serialize)]
    pub struct CollectionOwned_byAggregateInput {
        pub count: Option<Int>,
        #[serde(rename = "count_LT")]
        pub count_lt: Option<Int>,
        #[serde(rename = "count_LTE")]
        pub count_lte: Option<Int>,
        #[serde(rename = "count_GT")]
        pub count_gt: Option<Int>,
        #[serde(rename = "count_GTE")]
        pub count_gte: Option<Int>,
        #[serde(rename = "AND")]
        pub and: Option<Vec<CollectionOwned_byAggregateInput>>,
        #[serde(rename = "OR")]
        pub or: Option<Vec<CollectionOwned_byAggregateInput>>,
        pub node: Option<CollectionOwned_byNodeAggregationWhereInput>,
    }
    #[derive(Serialize)]
    pub struct CollectionOwned_byConnectFieldInput {
        #[serde(rename = "where")]
        pub where_: Option<UserConnectWhere>,
        pub connect: Option<UserConnectInput>,
    }
    #[derive(Serialize)]
    pub struct CollectionOwned_byConnectionWhere {
        #[serde(rename = "AND")]
        pub and: Box<Option<Vec<CollectionOwned_byConnectionWhere>>>,
        #[serde(rename = "OR")]
        pub or: Box<Option<Vec<CollectionOwned_byConnectionWhere>>>,
        pub node: Box<Option<UserWhere>>,
        #[serde(rename = "node_NOT")]
        pub node_not: Box<Option<UserWhere>>,
    }
    #[derive(Serialize)]
    pub struct CollectionOwned_byConnectOrCreateFieldInput {
        #[serde(rename = "where")]
        pub where_: UserConnectOrCreateWhere,
        #[serde(rename = "onCreate")]
        pub on_create: CollectionOwned_byConnectOrCreateFieldInputOnCreate,
    }
    #[derive(Serialize)]
    pub struct CollectionOwned_byConnectOrCreateFieldInputOnCreate {
        pub node: UserOnCreateInput,
    }
    #[derive(Serialize)]
    pub struct CollectionOwned_byCreateFieldInput {
        pub node: UserCreateInput,
    }
    #[derive(Serialize)]
    pub struct CollectionOwned_byFieldInput {
        pub create: Option<CollectionOwned_byCreateFieldInput>,
        pub connect: Option<CollectionOwned_byConnectFieldInput>,
        #[serde(rename = "connectOrCreate")]
        pub connect_or_create: Option<CollectionOwned_byConnectOrCreateFieldInput>,
    }
    #[derive(Serialize)]
    pub struct CollectionOwned_byNodeAggregationWhereInput {
        #[serde(rename = "AND")]
        pub and: Option<Vec<CollectionOwned_byNodeAggregationWhereInput>>,
        #[serde(rename = "OR")]
        pub or: Option<Vec<CollectionOwned_byNodeAggregationWhereInput>>,
        #[serde(rename = "id_EQUAL")]
        pub id_equal: Option<ID>,
        #[serde(rename = "username_EQUAL")]
        pub username_equal: Option<String>,
        #[serde(rename = "username_AVERAGE_EQUAL")]
        pub username_average_equal: Option<Float>,
        #[serde(rename = "username_LONGEST_EQUAL")]
        pub username_longest_equal: Option<Int>,
        #[serde(rename = "username_SHORTEST_EQUAL")]
        pub username_shortest_equal: Option<Int>,
        #[serde(rename = "username_GT")]
        pub username_gt: Option<Int>,
        #[serde(rename = "username_AVERAGE_GT")]
        pub username_average_gt: Option<Float>,
        #[serde(rename = "username_LONGEST_GT")]
        pub username_longest_gt: Option<Int>,
        #[serde(rename = "username_SHORTEST_GT")]
        pub username_shortest_gt: Option<Int>,
        #[serde(rename = "username_GTE")]
        pub username_gte: Option<Int>,
        #[serde(rename = "username_AVERAGE_GTE")]
        pub username_average_gte: Option<Float>,
        #[serde(rename = "username_LONGEST_GTE")]
        pub username_longest_gte: Option<Int>,
        #[serde(rename = "username_SHORTEST_GTE")]
        pub username_shortest_gte: Option<Int>,
        #[serde(rename = "username_LT")]
        pub username_lt: Option<Int>,
        #[serde(rename = "username_AVERAGE_LT")]
        pub username_average_lt: Option<Float>,
        #[serde(rename = "username_LONGEST_LT")]
        pub username_longest_lt: Option<Int>,
        #[serde(rename = "username_SHORTEST_LT")]
        pub username_shortest_lt: Option<Int>,
        #[serde(rename = "username_LTE")]
        pub username_lte: Option<Int>,
        #[serde(rename = "username_AVERAGE_LTE")]
        pub username_average_lte: Option<Float>,
        #[serde(rename = "username_LONGEST_LTE")]
        pub username_longest_lte: Option<Int>,
        #[serde(rename = "username_SHORTEST_LTE")]
        pub username_shortest_lte: Option<Int>,
        #[serde(rename = "email_EQUAL")]
        pub email_equal: Option<String>,
        #[serde(rename = "email_AVERAGE_EQUAL")]
        pub email_average_equal: Option<Float>,
        #[serde(rename = "email_LONGEST_EQUAL")]
        pub email_longest_equal: Option<Int>,
        #[serde(rename = "email_SHORTEST_EQUAL")]
        pub email_shortest_equal: Option<Int>,
        #[serde(rename = "email_GT")]
        pub email_gt: Option<Int>,
        #[serde(rename = "email_AVERAGE_GT")]
        pub email_average_gt: Option<Float>,
        #[serde(rename = "email_LONGEST_GT")]
        pub email_longest_gt: Option<Int>,
        #[serde(rename = "email_SHORTEST_GT")]
        pub email_shortest_gt: Option<Int>,
        #[serde(rename = "email_GTE")]
        pub email_gte: Option<Int>,
        #[serde(rename = "email_AVERAGE_GTE")]
        pub email_average_gte: Option<Float>,
        #[serde(rename = "email_LONGEST_GTE")]
        pub email_longest_gte: Option<Int>,
        #[serde(rename = "email_SHORTEST_GTE")]
        pub email_shortest_gte: Option<Int>,
        #[serde(rename = "email_LT")]
        pub email_lt: Option<Int>,
        #[serde(rename = "email_AVERAGE_LT")]
        pub email_average_lt: Option<Float>,
        #[serde(rename = "email_LONGEST_LT")]
        pub email_longest_lt: Option<Int>,
        #[serde(rename = "email_SHORTEST_LT")]
        pub email_shortest_lt: Option<Int>,
        #[serde(rename = "email_LTE")]
        pub email_lte: Option<Int>,
        #[serde(rename = "email_AVERAGE_LTE")]
        pub email_average_lte: Option<Float>,
        #[serde(rename = "email_LONGEST_LTE")]
        pub email_longest_lte: Option<Int>,
        #[serde(rename = "email_SHORTEST_LTE")]
        pub email_shortest_lte: Option<Int>,
        #[serde(rename = "image_EQUAL")]
        pub image_equal: Option<String>,
        #[serde(rename = "image_AVERAGE_EQUAL")]
        pub image_average_equal: Option<Float>,
        #[serde(rename = "image_LONGEST_EQUAL")]
        pub image_longest_equal: Option<Int>,
        #[serde(rename = "image_SHORTEST_EQUAL")]
        pub image_shortest_equal: Option<Int>,
        #[serde(rename = "image_GT")]
        pub image_gt: Option<Int>,
        #[serde(rename = "image_AVERAGE_GT")]
        pub image_average_gt: Option<Float>,
        #[serde(rename = "image_LONGEST_GT")]
        pub image_longest_gt: Option<Int>,
        #[serde(rename = "image_SHORTEST_GT")]
        pub image_shortest_gt: Option<Int>,
        #[serde(rename = "image_GTE")]
        pub image_gte: Option<Int>,
        #[serde(rename = "image_AVERAGE_GTE")]
        pub image_average_gte: Option<Float>,
        #[serde(rename = "image_LONGEST_GTE")]
        pub image_longest_gte: Option<Int>,
        #[serde(rename = "image_SHORTEST_GTE")]
        pub image_shortest_gte: Option<Int>,
        #[serde(rename = "image_LT")]
        pub image_lt: Option<Int>,
        #[serde(rename = "image_AVERAGE_LT")]
        pub image_average_lt: Option<Float>,
        #[serde(rename = "image_LONGEST_LT")]
        pub image_longest_lt: Option<Int>,
        #[serde(rename = "image_SHORTEST_LT")]
        pub image_shortest_lt: Option<Int>,
        #[serde(rename = "image_LTE")]
        pub image_lte: Option<Int>,
        #[serde(rename = "image_AVERAGE_LTE")]
        pub image_average_lte: Option<Float>,
        #[serde(rename = "image_LONGEST_LTE")]
        pub image_longest_lte: Option<Int>,
        #[serde(rename = "image_SHORTEST_LTE")]
        pub image_shortest_lte: Option<Int>,
        #[serde(rename = "phone_number_EQUAL")]
        pub phone_number_equal: Option<String>,
        #[serde(rename = "phone_number_AVERAGE_EQUAL")]
        pub phone_number_average_equal: Option<Float>,
        #[serde(rename = "phone_number_LONGEST_EQUAL")]
        pub phone_number_longest_equal: Option<Int>,
        #[serde(rename = "phone_number_SHORTEST_EQUAL")]
        pub phone_number_shortest_equal: Option<Int>,
        #[serde(rename = "phone_number_GT")]
        pub phone_number_gt: Option<Int>,
        #[serde(rename = "phone_number_AVERAGE_GT")]
        pub phone_number_average_gt: Option<Float>,
        #[serde(rename = "phone_number_LONGEST_GT")]
        pub phone_number_longest_gt: Option<Int>,
        #[serde(rename = "phone_number_SHORTEST_GT")]
        pub phone_number_shortest_gt: Option<Int>,
        #[serde(rename = "phone_number_GTE")]
        pub phone_number_gte: Option<Int>,
        #[serde(rename = "phone_number_AVERAGE_GTE")]
        pub phone_number_average_gte: Option<Float>,
        #[serde(rename = "phone_number_LONGEST_GTE")]
        pub phone_number_longest_gte: Option<Int>,
        #[serde(rename = "phone_number_SHORTEST_GTE")]
        pub phone_number_shortest_gte: Option<Int>,
        #[serde(rename = "phone_number_LT")]
        pub phone_number_lt: Option<Int>,
        #[serde(rename = "phone_number_AVERAGE_LT")]
        pub phone_number_average_lt: Option<Float>,
        #[serde(rename = "phone_number_LONGEST_LT")]
        pub phone_number_longest_lt: Option<Int>,
        #[serde(rename = "phone_number_SHORTEST_LT")]
        pub phone_number_shortest_lt: Option<Int>,
        #[serde(rename = "phone_number_LTE")]
        pub phone_number_lte: Option<Int>,
        #[serde(rename = "phone_number_AVERAGE_LTE")]
        pub phone_number_average_lte: Option<Float>,
        #[serde(rename = "phone_number_LONGEST_LTE")]
        pub phone_number_longest_lte: Option<Int>,
        #[serde(rename = "phone_number_SHORTEST_LTE")]
        pub phone_number_shortest_lte: Option<Int>,
        #[serde(rename = "password_EQUAL")]
        pub password_equal: Option<String>,
        #[serde(rename = "password_AVERAGE_EQUAL")]
        pub password_average_equal: Option<Float>,
        #[serde(rename = "password_LONGEST_EQUAL")]
        pub password_longest_equal: Option<Int>,
        #[serde(rename = "password_SHORTEST_EQUAL")]
        pub password_shortest_equal: Option<Int>,
        #[serde(rename = "password_GT")]
        pub password_gt: Option<Int>,
        #[serde(rename = "password_AVERAGE_GT")]
        pub password_average_gt: Option<Float>,
        #[serde(rename = "password_LONGEST_GT")]
        pub password_longest_gt: Option<Int>,
        #[serde(rename = "password_SHORTEST_GT")]
        pub password_shortest_gt: Option<Int>,
        #[serde(rename = "password_GTE")]
        pub password_gte: Option<Int>,
        #[serde(rename = "password_AVERAGE_GTE")]
        pub password_average_gte: Option<Float>,
        #[serde(rename = "password_LONGEST_GTE")]
        pub password_longest_gte: Option<Int>,
        #[serde(rename = "password_SHORTEST_GTE")]
        pub password_shortest_gte: Option<Int>,
        #[serde(rename = "password_LT")]
        pub password_lt: Option<Int>,
        #[serde(rename = "password_AVERAGE_LT")]
        pub password_average_lt: Option<Float>,
        #[serde(rename = "password_LONGEST_LT")]
        pub password_longest_lt: Option<Int>,
        #[serde(rename = "password_SHORTEST_LT")]
        pub password_shortest_lt: Option<Int>,
        #[serde(rename = "password_LTE")]
        pub password_lte: Option<Int>,
        #[serde(rename = "password_AVERAGE_LTE")]
        pub password_average_lte: Option<Float>,
        #[serde(rename = "password_LONGEST_LTE")]
        pub password_longest_lte: Option<Int>,
        #[serde(rename = "password_SHORTEST_LTE")]
        pub password_shortest_lte: Option<Int>,
        #[serde(rename = "register_at_EQUAL")]
        pub register_at_equal: Option<String>,
        #[serde(rename = "register_at_AVERAGE_EQUAL")]
        pub register_at_average_equal: Option<Float>,
        #[serde(rename = "register_at_LONGEST_EQUAL")]
        pub register_at_longest_equal: Option<Int>,
        #[serde(rename = "register_at_SHORTEST_EQUAL")]
        pub register_at_shortest_equal: Option<Int>,
        #[serde(rename = "register_at_GT")]
        pub register_at_gt: Option<Int>,
        #[serde(rename = "register_at_AVERAGE_GT")]
        pub register_at_average_gt: Option<Float>,
        #[serde(rename = "register_at_LONGEST_GT")]
        pub register_at_longest_gt: Option<Int>,
        #[serde(rename = "register_at_SHORTEST_GT")]
        pub register_at_shortest_gt: Option<Int>,
        #[serde(rename = "register_at_GTE")]
        pub register_at_gte: Option<Int>,
        #[serde(rename = "register_at_AVERAGE_GTE")]
        pub register_at_average_gte: Option<Float>,
        #[serde(rename = "register_at_LONGEST_GTE")]
        pub register_at_longest_gte: Option<Int>,
        #[serde(rename = "register_at_SHORTEST_GTE")]
        pub register_at_shortest_gte: Option<Int>,
        #[serde(rename = "register_at_LT")]
        pub register_at_lt: Option<Int>,
        #[serde(rename = "register_at_AVERAGE_LT")]
        pub register_at_average_lt: Option<Float>,
        #[serde(rename = "register_at_LONGEST_LT")]
        pub register_at_longest_lt: Option<Int>,
        #[serde(rename = "register_at_SHORTEST_LT")]
        pub register_at_shortest_lt: Option<Int>,
        #[serde(rename = "register_at_LTE")]
        pub register_at_lte: Option<Int>,
        #[serde(rename = "register_at_AVERAGE_LTE")]
        pub register_at_average_lte: Option<Float>,
        #[serde(rename = "register_at_LONGEST_LTE")]
        pub register_at_longest_lte: Option<Int>,
        #[serde(rename = "register_at_SHORTEST_LTE")]
        pub register_at_shortest_lte: Option<Int>,
    }
    #[derive(Serialize)]
    pub struct CollectionUniqueWhere {
        pub id: Option<ID>,
    }
    #[derive(Serialize)]
    pub struct CollectionWhere {
        #[serde(rename = "OR")]
        pub or: Box<Option<Vec<CollectionWhere>>>,
        #[serde(rename = "AND")]
        pub and: Box<Option<Vec<CollectionWhere>>>,
        pub id: Option<ID>,
        #[serde(rename = "id_NOT")]
        pub id_not: Option<ID>,
        #[serde(rename = "id_IN")]
        pub id_in: Option<Vec<ID>>,
        #[serde(rename = "id_NOT_IN")]
        pub id_not_in: Option<Vec<ID>>,
        #[serde(rename = "id_CONTAINS")]
        pub id_contains: Option<ID>,
        #[serde(rename = "id_NOT_CONTAINS")]
        pub id_not_contains: Option<ID>,
        #[serde(rename = "id_STARTS_WITH")]
        pub id_starts_with: Option<ID>,
        #[serde(rename = "id_NOT_STARTS_WITH")]
        pub id_not_starts_with: Option<ID>,
        #[serde(rename = "id_ENDS_WITH")]
        pub id_ends_with: Option<ID>,
        #[serde(rename = "id_NOT_ENDS_WITH")]
        pub id_not_ends_with: Option<ID>,
        pub name: Option<String>,
        #[serde(rename = "name_NOT")]
        pub name_not: Option<String>,
        #[serde(rename = "name_IN")]
        pub name_in: Option<Vec<String>>,
        #[serde(rename = "name_NOT_IN")]
        pub name_not_in: Option<Vec<String>>,
        #[serde(rename = "name_CONTAINS")]
        pub name_contains: Option<String>,
        #[serde(rename = "name_NOT_CONTAINS")]
        pub name_not_contains: Option<String>,
        #[serde(rename = "name_STARTS_WITH")]
        pub name_starts_with: Option<String>,
        #[serde(rename = "name_NOT_STARTS_WITH")]
        pub name_not_starts_with: Option<String>,
        #[serde(rename = "name_ENDS_WITH")]
        pub name_ends_with: Option<String>,
        #[serde(rename = "name_NOT_ENDS_WITH")]
        pub name_not_ends_with: Option<String>,
        pub is_public: Option<Boolean>,
        #[serde(rename = "is_public_NOT")]
        pub is_public_not: Option<Boolean>,
        #[serde(rename = "cardsAggregate")]
        pub cards_aggregate: Option<CollectionCardsAggregateInput>,
        #[serde(rename = "cards_ALL")]
        pub cards_all: Box<Option<CardWhere>>,
        #[serde(rename = "cards_NONE")]
        pub cards_none: Box<Option<CardWhere>>,
        #[serde(rename = "cards_SINGLE")]
        pub cards_single: Box<Option<CardWhere>>,
        #[serde(rename = "cards_SOME")]
        pub cards_some: Box<Option<CardWhere>>,
        pub owned_by: Box<Option<UserWhere>>,
        #[serde(rename = "owned_by_NOT")]
        pub owned_by_not: Box<Option<UserWhere>>,
        #[serde(rename = "owned_byAggregate")]
        pub owned_by_aggregate: Option<CollectionOwned_byAggregateInput>,
        #[serde(rename = "cardsConnection_ALL")]
        pub cards_connection_all: Box<Option<CollectionCardsConnectionWhere>>,
        #[serde(rename = "cardsConnection_NONE")]
        pub cards_connection_none: Box<Option<CollectionCardsConnectionWhere>>,
        #[serde(rename = "cardsConnection_SINGLE")]
        pub cards_connection_single: Box<Option<CollectionCardsConnectionWhere>>,
        #[serde(rename = "cardsConnection_SOME")]
        pub cards_connection_some: Box<Option<CollectionCardsConnectionWhere>>,
        #[serde(rename = "owned_byConnection")]
        pub owned_by_connection: Box<Option<CollectionOwned_byConnectionWhere>>,
        #[serde(rename = "owned_byConnection_NOT")]
        pub owned_by_connection_not: Box<Option<CollectionOwned_byConnectionWhere>>,
    }
    #[derive(Serialize)]
    pub struct UserCardsAggregateInput {
        pub count: Option<Int>,
        #[serde(rename = "count_LT")]
        pub count_lt: Option<Int>,
        #[serde(rename = "count_LTE")]
        pub count_lte: Option<Int>,
        #[serde(rename = "count_GT")]
        pub count_gt: Option<Int>,
        #[serde(rename = "count_GTE")]
        pub count_gte: Option<Int>,
        #[serde(rename = "AND")]
        pub and: Option<Vec<UserCardsAggregateInput>>,
        #[serde(rename = "OR")]
        pub or: Option<Vec<UserCardsAggregateInput>>,
        pub node: Option<UserCardsNodeAggregationWhereInput>,
    }
    #[derive(Serialize)]
    pub struct UserCardsConnectFieldInput {
        #[serde(rename = "where")]
        pub where_: Option<CardConnectWhere>,
        pub connect: Option<Vec<CardConnectInput>>,
    }
    #[derive(Serialize)]
    pub struct UserCardsConnectionWhere {
        #[serde(rename = "AND")]
        pub and: Box<Option<Vec<UserCardsConnectionWhere>>>,
        #[serde(rename = "OR")]
        pub or: Box<Option<Vec<UserCardsConnectionWhere>>>,
        pub node: Box<Option<CardWhere>>,
        #[serde(rename = "node_NOT")]
        pub node_not: Box<Option<CardWhere>>,
    }
    #[derive(Serialize)]
    pub struct UserCardsConnectOrCreateFieldInput {
        #[serde(rename = "where")]
        pub where_: CardConnectOrCreateWhere,
        #[serde(rename = "onCreate")]
        pub on_create: UserCardsConnectOrCreateFieldInputOnCreate,
    }
    #[derive(Serialize)]
    pub struct UserCardsConnectOrCreateFieldInputOnCreate {
        pub node: CardOnCreateInput,
    }
    #[derive(Serialize)]
    pub struct UserCardsCreateFieldInput {
        pub node: CardCreateInput,
    }
    #[derive(Serialize)]
    pub struct UserCardsFieldInput {
        pub create: Option<Vec<UserCardsCreateFieldInput>>,
        pub connect: Option<Vec<UserCardsConnectFieldInput>>,
        #[serde(rename = "connectOrCreate")]
        pub connect_or_create: Option<Vec<UserCardsConnectOrCreateFieldInput>>,
    }
    #[derive(Serialize)]
    pub struct UserCardsNodeAggregationWhereInput {
        #[serde(rename = "AND")]
        pub and: Option<Vec<UserCardsNodeAggregationWhereInput>>,
        #[serde(rename = "OR")]
        pub or: Option<Vec<UserCardsNodeAggregationWhereInput>>,
        #[serde(rename = "id_EQUAL")]
        pub id_equal: Option<ID>,
        #[serde(rename = "duplicate_from_EQUAL")]
        pub duplicate_from_equal: Option<ID>,
        #[serde(rename = "name_EQUAL")]
        pub name_equal: Option<String>,
        #[serde(rename = "name_AVERAGE_EQUAL")]
        pub name_average_equal: Option<Float>,
        #[serde(rename = "name_LONGEST_EQUAL")]
        pub name_longest_equal: Option<Int>,
        #[serde(rename = "name_SHORTEST_EQUAL")]
        pub name_shortest_equal: Option<Int>,
        #[serde(rename = "name_GT")]
        pub name_gt: Option<Int>,
        #[serde(rename = "name_AVERAGE_GT")]
        pub name_average_gt: Option<Float>,
        #[serde(rename = "name_LONGEST_GT")]
        pub name_longest_gt: Option<Int>,
        #[serde(rename = "name_SHORTEST_GT")]
        pub name_shortest_gt: Option<Int>,
        #[serde(rename = "name_GTE")]
        pub name_gte: Option<Int>,
        #[serde(rename = "name_AVERAGE_GTE")]
        pub name_average_gte: Option<Float>,
        #[serde(rename = "name_LONGEST_GTE")]
        pub name_longest_gte: Option<Int>,
        #[serde(rename = "name_SHORTEST_GTE")]
        pub name_shortest_gte: Option<Int>,
        #[serde(rename = "name_LT")]
        pub name_lt: Option<Int>,
        #[serde(rename = "name_AVERAGE_LT")]
        pub name_average_lt: Option<Float>,
        #[serde(rename = "name_LONGEST_LT")]
        pub name_longest_lt: Option<Int>,
        #[serde(rename = "name_SHORTEST_LT")]
        pub name_shortest_lt: Option<Int>,
        #[serde(rename = "name_LTE")]
        pub name_lte: Option<Int>,
        #[serde(rename = "name_AVERAGE_LTE")]
        pub name_average_lte: Option<Float>,
        #[serde(rename = "name_LONGEST_LTE")]
        pub name_longest_lte: Option<Int>,
        #[serde(rename = "name_SHORTEST_LTE")]
        pub name_shortest_lte: Option<Int>,
        #[serde(rename = "description_EQUAL")]
        pub description_equal: Option<String>,
        #[serde(rename = "description_AVERAGE_EQUAL")]
        pub description_average_equal: Option<Float>,
        #[serde(rename = "description_LONGEST_EQUAL")]
        pub description_longest_equal: Option<Int>,
        #[serde(rename = "description_SHORTEST_EQUAL")]
        pub description_shortest_equal: Option<Int>,
        #[serde(rename = "description_GT")]
        pub description_gt: Option<Int>,
        #[serde(rename = "description_AVERAGE_GT")]
        pub description_average_gt: Option<Float>,
        #[serde(rename = "description_LONGEST_GT")]
        pub description_longest_gt: Option<Int>,
        #[serde(rename = "description_SHORTEST_GT")]
        pub description_shortest_gt: Option<Int>,
        #[serde(rename = "description_GTE")]
        pub description_gte: Option<Int>,
        #[serde(rename = "description_AVERAGE_GTE")]
        pub description_average_gte: Option<Float>,
        #[serde(rename = "description_LONGEST_GTE")]
        pub description_longest_gte: Option<Int>,
        #[serde(rename = "description_SHORTEST_GTE")]
        pub description_shortest_gte: Option<Int>,
        #[serde(rename = "description_LT")]
        pub description_lt: Option<Int>,
        #[serde(rename = "description_AVERAGE_LT")]
        pub description_average_lt: Option<Float>,
        #[serde(rename = "description_LONGEST_LT")]
        pub description_longest_lt: Option<Int>,
        #[serde(rename = "description_SHORTEST_LT")]
        pub description_shortest_lt: Option<Int>,
        #[serde(rename = "description_LTE")]
        pub description_lte: Option<Int>,
        #[serde(rename = "description_AVERAGE_LTE")]
        pub description_average_lte: Option<Float>,
        #[serde(rename = "description_LONGEST_LTE")]
        pub description_longest_lte: Option<Int>,
        #[serde(rename = "description_SHORTEST_LTE")]
        pub description_shortest_lte: Option<Int>,
        #[serde(rename = "view_EQUAL")]
        pub view_equal: Option<String>,
        #[serde(rename = "view_AVERAGE_EQUAL")]
        pub view_average_equal: Option<Float>,
        #[serde(rename = "view_LONGEST_EQUAL")]
        pub view_longest_equal: Option<Int>,
        #[serde(rename = "view_SHORTEST_EQUAL")]
        pub view_shortest_equal: Option<Int>,
        #[serde(rename = "view_GT")]
        pub view_gt: Option<Int>,
        #[serde(rename = "view_AVERAGE_GT")]
        pub view_average_gt: Option<Float>,
        #[serde(rename = "view_LONGEST_GT")]
        pub view_longest_gt: Option<Int>,
        #[serde(rename = "view_SHORTEST_GT")]
        pub view_shortest_gt: Option<Int>,
        #[serde(rename = "view_GTE")]
        pub view_gte: Option<Int>,
        #[serde(rename = "view_AVERAGE_GTE")]
        pub view_average_gte: Option<Float>,
        #[serde(rename = "view_LONGEST_GTE")]
        pub view_longest_gte: Option<Int>,
        #[serde(rename = "view_SHORTEST_GTE")]
        pub view_shortest_gte: Option<Int>,
        #[serde(rename = "view_LT")]
        pub view_lt: Option<Int>,
        #[serde(rename = "view_AVERAGE_LT")]
        pub view_average_lt: Option<Float>,
        #[serde(rename = "view_LONGEST_LT")]
        pub view_longest_lt: Option<Int>,
        #[serde(rename = "view_SHORTEST_LT")]
        pub view_shortest_lt: Option<Int>,
        #[serde(rename = "view_LTE")]
        pub view_lte: Option<Int>,
        #[serde(rename = "view_AVERAGE_LTE")]
        pub view_average_lte: Option<Float>,
        #[serde(rename = "view_LONGEST_LTE")]
        pub view_longest_lte: Option<Int>,
        #[serde(rename = "view_SHORTEST_LTE")]
        pub view_shortest_lte: Option<Int>,
    }
    #[derive(Serialize)]
    pub struct UserCollectionsAggregateInput {
        pub count: Option<Int>,
        #[serde(rename = "count_LT")]
        pub count_lt: Option<Int>,
        #[serde(rename = "count_LTE")]
        pub count_lte: Option<Int>,
        #[serde(rename = "count_GT")]
        pub count_gt: Option<Int>,
        #[serde(rename = "count_GTE")]
        pub count_gte: Option<Int>,
        #[serde(rename = "AND")]
        pub and: Option<Vec<UserCollectionsAggregateInput>>,
        #[serde(rename = "OR")]
        pub or: Option<Vec<UserCollectionsAggregateInput>>,
        pub node: Option<UserCollectionsNodeAggregationWhereInput>,
    }
    #[derive(Serialize)]
    pub struct UserCollectionsConnectFieldInput {
        #[serde(rename = "where")]
        pub where_: Option<CollectionConnectWhere>,
        pub connect: Option<Vec<CollectionConnectInput>>,
    }
    #[derive(Serialize)]
    pub struct UserCollectionsConnectionWhere {
        #[serde(rename = "AND")]
        pub and: Box<Option<Vec<UserCollectionsConnectionWhere>>>,
        #[serde(rename = "OR")]
        pub or: Box<Option<Vec<UserCollectionsConnectionWhere>>>,
        pub node: Box<Option<CollectionWhere>>,
        #[serde(rename = "node_NOT")]
        pub node_not: Box<Option<CollectionWhere>>,
    }
    #[derive(Serialize)]
    pub struct UserCollectionsConnectOrCreateFieldInput {
        #[serde(rename = "where")]
        pub where_: CollectionConnectOrCreateWhere,
        #[serde(rename = "onCreate")]
        pub on_create: UserCollectionsConnectOrCreateFieldInputOnCreate,
    }
    #[derive(Serialize)]
    pub struct UserCollectionsConnectOrCreateFieldInputOnCreate {
        pub node: CollectionOnCreateInput,
    }
    #[derive(Serialize)]
    pub struct UserCollectionsCreateFieldInput {
        pub node: CollectionCreateInput,
    }
    #[derive(Serialize)]
    pub struct UserCollectionsFieldInput {
        pub create: Option<Vec<UserCollectionsCreateFieldInput>>,
        pub connect: Option<Vec<UserCollectionsConnectFieldInput>>,
        #[serde(rename = "connectOrCreate")]
        pub connect_or_create: Option<Vec<UserCollectionsConnectOrCreateFieldInput>>,
    }
    #[derive(Serialize)]
    pub struct UserCollectionsNodeAggregationWhereInput {
        #[serde(rename = "AND")]
        pub and: Option<Vec<UserCollectionsNodeAggregationWhereInput>>,
        #[serde(rename = "OR")]
        pub or: Option<Vec<UserCollectionsNodeAggregationWhereInput>>,
        #[serde(rename = "id_EQUAL")]
        pub id_equal: Option<ID>,
        #[serde(rename = "name_EQUAL")]
        pub name_equal: Option<String>,
        #[serde(rename = "name_AVERAGE_EQUAL")]
        pub name_average_equal: Option<Float>,
        #[serde(rename = "name_LONGEST_EQUAL")]
        pub name_longest_equal: Option<Int>,
        #[serde(rename = "name_SHORTEST_EQUAL")]
        pub name_shortest_equal: Option<Int>,
        #[serde(rename = "name_GT")]
        pub name_gt: Option<Int>,
        #[serde(rename = "name_AVERAGE_GT")]
        pub name_average_gt: Option<Float>,
        #[serde(rename = "name_LONGEST_GT")]
        pub name_longest_gt: Option<Int>,
        #[serde(rename = "name_SHORTEST_GT")]
        pub name_shortest_gt: Option<Int>,
        #[serde(rename = "name_GTE")]
        pub name_gte: Option<Int>,
        #[serde(rename = "name_AVERAGE_GTE")]
        pub name_average_gte: Option<Float>,
        #[serde(rename = "name_LONGEST_GTE")]
        pub name_longest_gte: Option<Int>,
        #[serde(rename = "name_SHORTEST_GTE")]
        pub name_shortest_gte: Option<Int>,
        #[serde(rename = "name_LT")]
        pub name_lt: Option<Int>,
        #[serde(rename = "name_AVERAGE_LT")]
        pub name_average_lt: Option<Float>,
        #[serde(rename = "name_LONGEST_LT")]
        pub name_longest_lt: Option<Int>,
        #[serde(rename = "name_SHORTEST_LT")]
        pub name_shortest_lt: Option<Int>,
        #[serde(rename = "name_LTE")]
        pub name_lte: Option<Int>,
        #[serde(rename = "name_AVERAGE_LTE")]
        pub name_average_lte: Option<Float>,
        #[serde(rename = "name_LONGEST_LTE")]
        pub name_longest_lte: Option<Int>,
        #[serde(rename = "name_SHORTEST_LTE")]
        pub name_shortest_lte: Option<Int>,
    }
    #[derive(Serialize)]
    pub struct UserConnectInput {
        pub collections: Option<Vec<UserCollectionsConnectFieldInput>>,
        pub cards: Option<Vec<UserCardsConnectFieldInput>>,
    }
    #[derive(Serialize)]
    pub struct UserConnectOrCreateWhere {
        pub node: UserUniqueWhere,
    }
    #[derive(Serialize)]
    pub struct UserConnectWhere {
        pub node: Box<UserWhere>,
    }
    #[derive(Serialize)]
    pub struct UserCreateInput {
        pub username: String,
        pub email: String,
        pub image: Option<String>,
        pub phone_number: Option<String>,
        pub password: String,
        pub register_at: String,
        pub collections: Option<UserCollectionsFieldInput>,
        pub cards: Option<UserCardsFieldInput>,
    }
    #[derive(Serialize)]
    pub struct UserOnCreateInput {
        pub username: String,
        pub email: String,
        pub image: Option<String>,
        pub phone_number: Option<String>,
        pub password: String,
        pub register_at: String,
    }
    #[derive(Serialize)]
    pub struct UserUniqueWhere {
        pub id: Option<ID>,
        pub username: Option<String>,
        pub email: Option<String>,
        pub phone_number: Option<String>,
    }
    #[derive(Serialize)]
    pub struct UserWhere {
        #[serde(rename = "OR")]
        pub or: Box<Option<Vec<UserWhere>>>,
        #[serde(rename = "AND")]
        pub and: Box<Option<Vec<UserWhere>>>,
        pub id: Option<ID>,
        #[serde(rename = "id_NOT")]
        pub id_not: Option<ID>,
        #[serde(rename = "id_IN")]
        pub id_in: Option<Vec<ID>>,
        #[serde(rename = "id_NOT_IN")]
        pub id_not_in: Option<Vec<ID>>,
        #[serde(rename = "id_CONTAINS")]
        pub id_contains: Option<ID>,
        #[serde(rename = "id_NOT_CONTAINS")]
        pub id_not_contains: Option<ID>,
        #[serde(rename = "id_STARTS_WITH")]
        pub id_starts_with: Option<ID>,
        #[serde(rename = "id_NOT_STARTS_WITH")]
        pub id_not_starts_with: Option<ID>,
        #[serde(rename = "id_ENDS_WITH")]
        pub id_ends_with: Option<ID>,
        #[serde(rename = "id_NOT_ENDS_WITH")]
        pub id_not_ends_with: Option<ID>,
        pub username: Option<String>,
        #[serde(rename = "username_NOT")]
        pub username_not: Option<String>,
        #[serde(rename = "username_IN")]
        pub username_in: Option<Vec<String>>,
        #[serde(rename = "username_NOT_IN")]
        pub username_not_in: Option<Vec<String>>,
        #[serde(rename = "username_CONTAINS")]
        pub username_contains: Option<String>,
        #[serde(rename = "username_NOT_CONTAINS")]
        pub username_not_contains: Option<String>,
        #[serde(rename = "username_STARTS_WITH")]
        pub username_starts_with: Option<String>,
        #[serde(rename = "username_NOT_STARTS_WITH")]
        pub username_not_starts_with: Option<String>,
        #[serde(rename = "username_ENDS_WITH")]
        pub username_ends_with: Option<String>,
        #[serde(rename = "username_NOT_ENDS_WITH")]
        pub username_not_ends_with: Option<String>,
        pub email: Option<String>,
        #[serde(rename = "email_NOT")]
        pub email_not: Option<String>,
        #[serde(rename = "email_IN")]
        pub email_in: Option<Vec<String>>,
        #[serde(rename = "email_NOT_IN")]
        pub email_not_in: Option<Vec<String>>,
        #[serde(rename = "email_CONTAINS")]
        pub email_contains: Option<String>,
        #[serde(rename = "email_NOT_CONTAINS")]
        pub email_not_contains: Option<String>,
        #[serde(rename = "email_STARTS_WITH")]
        pub email_starts_with: Option<String>,
        #[serde(rename = "email_NOT_STARTS_WITH")]
        pub email_not_starts_with: Option<String>,
        #[serde(rename = "email_ENDS_WITH")]
        pub email_ends_with: Option<String>,
        #[serde(rename = "email_NOT_ENDS_WITH")]
        pub email_not_ends_with: Option<String>,
        pub image: Option<String>,
        #[serde(rename = "image_NOT")]
        pub image_not: Option<String>,
        #[serde(rename = "image_IN")]
        pub image_in: Option<Vec<Option<String>>>,
        #[serde(rename = "image_NOT_IN")]
        pub image_not_in: Option<Vec<Option<String>>>,
        #[serde(rename = "image_CONTAINS")]
        pub image_contains: Option<String>,
        #[serde(rename = "image_NOT_CONTAINS")]
        pub image_not_contains: Option<String>,
        #[serde(rename = "image_STARTS_WITH")]
        pub image_starts_with: Option<String>,
        #[serde(rename = "image_NOT_STARTS_WITH")]
        pub image_not_starts_with: Option<String>,
        #[serde(rename = "image_ENDS_WITH")]
        pub image_ends_with: Option<String>,
        #[serde(rename = "image_NOT_ENDS_WITH")]
        pub image_not_ends_with: Option<String>,
        pub phone_number: Option<String>,
        #[serde(rename = "phone_number_NOT")]
        pub phone_number_not: Option<String>,
        #[serde(rename = "phone_number_IN")]
        pub phone_number_in: Option<Vec<Option<String>>>,
        #[serde(rename = "phone_number_NOT_IN")]
        pub phone_number_not_in: Option<Vec<Option<String>>>,
        #[serde(rename = "phone_number_CONTAINS")]
        pub phone_number_contains: Option<String>,
        #[serde(rename = "phone_number_NOT_CONTAINS")]
        pub phone_number_not_contains: Option<String>,
        #[serde(rename = "phone_number_STARTS_WITH")]
        pub phone_number_starts_with: Option<String>,
        #[serde(rename = "phone_number_NOT_STARTS_WITH")]
        pub phone_number_not_starts_with: Option<String>,
        #[serde(rename = "phone_number_ENDS_WITH")]
        pub phone_number_ends_with: Option<String>,
        #[serde(rename = "phone_number_NOT_ENDS_WITH")]
        pub phone_number_not_ends_with: Option<String>,
        pub password: Option<String>,
        #[serde(rename = "password_NOT")]
        pub password_not: Option<String>,
        #[serde(rename = "password_IN")]
        pub password_in: Option<Vec<String>>,
        #[serde(rename = "password_NOT_IN")]
        pub password_not_in: Option<Vec<String>>,
        #[serde(rename = "password_CONTAINS")]
        pub password_contains: Option<String>,
        #[serde(rename = "password_NOT_CONTAINS")]
        pub password_not_contains: Option<String>,
        #[serde(rename = "password_STARTS_WITH")]
        pub password_starts_with: Option<String>,
        #[serde(rename = "password_NOT_STARTS_WITH")]
        pub password_not_starts_with: Option<String>,
        #[serde(rename = "password_ENDS_WITH")]
        pub password_ends_with: Option<String>,
        #[serde(rename = "password_NOT_ENDS_WITH")]
        pub password_not_ends_with: Option<String>,
        pub register_at: Option<String>,
        #[serde(rename = "register_at_NOT")]
        pub register_at_not: Option<String>,
        #[serde(rename = "register_at_IN")]
        pub register_at_in: Option<Vec<String>>,
        #[serde(rename = "register_at_NOT_IN")]
        pub register_at_not_in: Option<Vec<String>>,
        #[serde(rename = "register_at_CONTAINS")]
        pub register_at_contains: Option<String>,
        #[serde(rename = "register_at_NOT_CONTAINS")]
        pub register_at_not_contains: Option<String>,
        #[serde(rename = "register_at_STARTS_WITH")]
        pub register_at_starts_with: Option<String>,
        #[serde(rename = "register_at_NOT_STARTS_WITH")]
        pub register_at_not_starts_with: Option<String>,
        #[serde(rename = "register_at_ENDS_WITH")]
        pub register_at_ends_with: Option<String>,
        #[serde(rename = "register_at_NOT_ENDS_WITH")]
        pub register_at_not_ends_with: Option<String>,
        #[serde(rename = "collectionsAggregate")]
        pub collections_aggregate: Option<UserCollectionsAggregateInput>,
        #[serde(rename = "collections_ALL")]
        pub collections_all: Box<Option<CollectionWhere>>,
        #[serde(rename = "collections_NONE")]
        pub collections_none: Box<Option<CollectionWhere>>,
        #[serde(rename = "collections_SINGLE")]
        pub collections_single: Box<Option<CollectionWhere>>,
        #[serde(rename = "collections_SOME")]
        pub collections_some: Box<Option<CollectionWhere>>,
        #[serde(rename = "cardsAggregate")]
        pub cards_aggregate: Option<UserCardsAggregateInput>,
        #[serde(rename = "cards_ALL")]
        pub cards_all: Box<Option<CardWhere>>,
        #[serde(rename = "cards_NONE")]
        pub cards_none: Box<Option<CardWhere>>,
        #[serde(rename = "cards_SINGLE")]
        pub cards_single: Box<Option<CardWhere>>,
        #[serde(rename = "cards_SOME")]
        pub cards_some: Box<Option<CardWhere>>,
        #[serde(rename = "collectionsConnection_ALL")]
        pub collections_connection_all: Box<Option<UserCollectionsConnectionWhere>>,
        #[serde(rename = "collectionsConnection_NONE")]
        pub collections_connection_none: Box<Option<UserCollectionsConnectionWhere>>,
        #[serde(rename = "collectionsConnection_SINGLE")]
        pub collections_connection_single: Box<Option<UserCollectionsConnectionWhere>>,
        #[serde(rename = "collectionsConnection_SOME")]
        pub collections_connection_some: Box<Option<UserCollectionsConnectionWhere>>,
        #[serde(rename = "cardsConnection_ALL")]
        pub cards_connection_all: Box<Option<UserCardsConnectionWhere>>,
        #[serde(rename = "cardsConnection_NONE")]
        pub cards_connection_none: Box<Option<UserCardsConnectionWhere>>,
        #[serde(rename = "cardsConnection_SINGLE")]
        pub cards_connection_single: Box<Option<UserCardsConnectionWhere>>,
        #[serde(rename = "cardsConnection_SOME")]
        pub cards_connection_some: Box<Option<UserCardsConnectionWhere>>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub name: String,
        pub is_public: Boolean,
        pub cards: Option<CollectionCardsFieldInput>,
        pub owned_by: Option<CollectionOwned_byFieldInput>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "createCollections")]
        pub create_collections: CreateCollectionCreateCollections,
    }
    #[derive(Deserialize)]
    pub struct CreateCollectionCreateCollections {
        pub info: CreateCollectionCreateCollectionsInfo,
        pub collections: Vec<CreateCollectionCreateCollectionsCollections>,
    }
    #[derive(Deserialize)]
    pub struct CreateCollectionCreateCollectionsInfo {
        #[serde(rename = "nodesCreated")]
        pub nodes_created: Int,
    }
    #[derive(Deserialize)]
    pub struct CreateCollectionCreateCollectionsCollections {
        pub id: ID,
    }
}
impl graphql_client::GraphQLQuery for CreateCollection {
    type Variables = create_collection::Variables;
    type ResponseData = create_collection::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: create_collection::QUERY,
            operation_name: create_collection::OPERATION_NAME,
        }
    }
}
pub struct RegisterNewUser;
pub mod register_new_user {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "RegisterNewUser";
    pub const QUERY : & str = "mutation CreateCollection($name: String!,\r\n $is_public: Boolean!,\r\n $cards: CollectionCardsFieldInput,\r\n $owned_by: CollectionOwned_byFieldInput) {\r\n  createCollections(input: [{\r\n    name: $name,\r\n    is_public: $is_public,\r\n    cards: $cards,\r\n    owned_by: $owned_by\r\n  }]) {\r\n    info {\r\n      nodesCreated\r\n    }\r\n    collections {\r\n      id\r\n    }\r\n  }\r\n}\r\nmutation RegisterNewUser(\r\n  $username: String!\r\n  $email: String!\r\n  $password: String!\r\n  $register_at: String!\r\n  $phone_number: String\r\n) {\r\n  createUsers(\r\n    input: {\r\n      username: $username\r\n      password: $password\r\n      email: $email\r\n      register_at: $register_at\r\n      phone_number: $phone_number\r\n    }\r\n  ) {\r\n    info {\r\n      nodesCreated\r\n    }\r\n    users {\r\n      id\r\n    }\r\n  }\r\n}\r\nquery GetUserByEmailOrUsername($account: String) {\r\n  users(where: {OR: [{email: $account},{username: $account}]}) {    id    username    password  }}" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize, Deserialize)]
    pub struct Variables {
        pub username: String,
        pub email: String,
        pub password: String,
        pub register_at: String,
        pub phone_number: Option<String>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "createUsers")]
        pub create_users: RegisterNewUserCreateUsers,
    }
    #[derive(Deserialize)]
    pub struct RegisterNewUserCreateUsers {
        pub info: RegisterNewUserCreateUsersInfo,
        pub users: Vec<RegisterNewUserCreateUsersUsers>,
    }
    #[derive(Deserialize)]
    pub struct RegisterNewUserCreateUsersInfo {
        #[serde(rename = "nodesCreated")]
        pub nodes_created: Int,
    }
    #[derive(Deserialize)]
    pub struct RegisterNewUserCreateUsersUsers {
        pub id: ID,
    }
}
impl graphql_client::GraphQLQuery for RegisterNewUser {
    type Variables = register_new_user::Variables;
    type ResponseData = register_new_user::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: register_new_user::QUERY,
            operation_name: register_new_user::OPERATION_NAME,
        }
    }
}
pub struct GetUserByEmailOrUsername;
pub mod get_user_by_email_or_username {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetUserByEmailOrUsername";
    pub const QUERY : & str = "mutation CreateCollection($name: String!,\r\n $is_public: Boolean!,\r\n $cards: CollectionCardsFieldInput,\r\n $owned_by: CollectionOwned_byFieldInput) {\r\n  createCollections(input: [{\r\n    name: $name,\r\n    is_public: $is_public,\r\n    cards: $cards,\r\n    owned_by: $owned_by\r\n  }]) {\r\n    info {\r\n      nodesCreated\r\n    }\r\n    collections {\r\n      id\r\n    }\r\n  }\r\n}\r\nmutation RegisterNewUser(\r\n  $username: String!\r\n  $email: String!\r\n  $password: String!\r\n  $register_at: String!\r\n  $phone_number: String\r\n) {\r\n  createUsers(\r\n    input: {\r\n      username: $username\r\n      password: $password\r\n      email: $email\r\n      register_at: $register_at\r\n      phone_number: $phone_number\r\n    }\r\n  ) {\r\n    info {\r\n      nodesCreated\r\n    }\r\n    users {\r\n      id\r\n    }\r\n  }\r\n}\r\nquery GetUserByEmailOrUsername($account: String) {\r\n  users(where: {OR: [{email: $account},{username: $account}]}) {    id    username    password  }}" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize, Deserialize)]
    pub struct Variables {
        pub account: Option<String>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Vec<GetUserByEmailOrUsernameUsers>,
    }
    #[derive(Deserialize)]
    pub struct GetUserByEmailOrUsernameUsers {
        pub id: ID,
        pub username: String,
        pub password: String,
    }
}
impl graphql_client::GraphQLQuery for GetUserByEmailOrUsername {
    type Variables = get_user_by_email_or_username::Variables;
    type ResponseData = get_user_by_email_or_username::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_user_by_email_or_username::QUERY,
            operation_name: get_user_by_email_or_username::OPERATION_NAME,
        }
    }
}
