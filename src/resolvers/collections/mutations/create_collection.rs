use aragog::DatabaseConnection;
use async_graphql::{Context, Object, Result};
use ogm::models::prelude::Collection;

#[derive(Default)]
pub struct CreateCollectionMutation;

#[Object]
impl CreateCollectionMutation {
    pub async fn create_collection(
        &self,
        ctx: &Context<'_>,
        input: gql_types::collection::CollectionCreateInput,
    ) -> Result<String> {
        let token = ctx.data_opt::<crate::TokenData>();
        if let Some(token) = token {
            let cards = if let Some(_cards) = &input.cards {
                todo!("cards record is not ready yet");
            } else {
                Vec::<String>::new()
            };
            let db = ctx.data_unchecked::<DatabaseConnection>();
            let res = ogm::services::prelude::create_collection( Collection{
                name: input.name,
                is_public: input.is_public,
                cards,
                created_by: format!("User/{}", &token.id.0)
            }, db, &token.id.0).await;
            match res {
                Ok(res) => Ok(res),
                Err(err) => match err {
                    ogm::services::Error::Internal(msg) => Err(crate::utils::new_internal_server_error(msg)),
                    ogm::services::Error::NotFound(_, _) => Err(crate::utils::new_internal_server_error("unknown error".to_owned())),
                    ogm::services::Error::Conflict(msg) => Err(crate::utils::new_conflict_error(msg)),
                },
            }
            
        } else {
            Err(crate::utils::new_not_authenticated_error("log in before create collection".to_owned()))
        }
    }
}
