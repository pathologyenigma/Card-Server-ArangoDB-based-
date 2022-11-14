use aragog::{
    query::{Comparison, Filter},
    DatabaseConnection, Record,
};

use crate::models::prelude::User;

use super::Error;

pub async fn get_user_by_username_or_email(
    account: &String,
    db: &DatabaseConnection,
) -> Result<(String, User), Error> {
    let res = User::query()
        .filter(
            Filter::new(Comparison::field("username").equals_str(&account))
                .or(Comparison::field("email").equals_str(&account)),
        )
        .call::<DatabaseConnection, User>(db)
        .await
        .map_err(|err| Error::from_aragog_error(true, err))?;

    if res.len() > 1 {
        return Err(Error::Internal(
            "found multiple users with the same username, this should not be happen".to_owned(),
        ));
    }
    let user = res
        .first_record()
        .map(|data| (data.id().clone(), data.record));
    match user {
        Some(user) => return Ok(user),
        None => {
            return Err(Error::NotFound("account".to_owned(), account.clone()));
        }
    }
}

pub async fn new_user(user: User, db: &DatabaseConnection) -> Result<String, Error> {
    let res = User::create(user, db)
        .await
        .map_err(|err| Error::from_aragog_error(false, err))?;
    Ok(res.id().clone())
}
