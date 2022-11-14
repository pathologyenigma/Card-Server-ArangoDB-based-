use aragog::{query::Query, DatabaseConnection, Record};

use crate::{models::prelude::Collection, prelude::User};

use super::Error;

pub struct CollectionWithOwner {
    pub name: String,
    // make same to be using by different user
    // but only once
    pub created_by: User,
    pub is_public: bool,
    // cards is not implemented yet
    // pub cards: Vec<String>,
}

pub async fn create_collection(
    collection: Collection,
    db: &DatabaseConnection,
    user_id: &String,
) -> Result<String, Error> {
    let res = Collection::create(collection, db)
        .await
        .map_err(|err| Error::from_aragog_error(false, err))?;
    // add this collection's id into the User who owns this
    let mut owner = User::find(user_id, db)
        .await
        .map_err(|err| Error::from_aragog_error(true, err))?;
    owner.collections.push(res.id().clone());
    owner.save(db).await.map_err(|err| Error::from_aragog_error(false, err))?;
    Ok(res.id().clone())
}

pub async fn search_collection_by_name(
    phrase: &String,
    db: &DatabaseConnection,
) -> Result<Vec<(String, Collection)>, Error> {
    let res = Query::new(&format!("FULLTEXT(Collection, \"name\", \"{}\")", phrase))
        .call::<DatabaseConnection, Collection>(db)
        .await
        .map_err(|err| Error::from_aragog_error(true, err))?;
    let res = res
        .0
        .into_iter()
        .map(|data| (data.id().clone(), data.record))
        .collect::<Vec<(String, Collection)>>();
    Ok(res)
}
