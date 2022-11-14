

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
        .map_err(|err| match err {
            aragog::Error::ArangoError(err) => match err.arango_error {
                aragog::error::ArangoError::ArangoDataSourceNotFound => {
                    return Error::Internal("Table Not exist".to_owned());
                }
                _ => return Error::Internal(format!("{}", err)),
            },
            _ => {
                return Error::Internal(format!("{}", err));
            }
        })?;
    let mut users = res
        .0
        .into_iter()
        .map(|data| (data.key().clone(), data.record))
        .take(1)
        .collect::<Vec<(String, User)>>();
    if users.len() < 1 {
        return Err(Error::NotFound("account".to_owned(), account.clone()));
    }
    Ok(users.pop().unwrap())
}

pub async fn new_user(user: User, db: &DatabaseConnection) -> Result<String, Error> {
    let res = User::create(user, db).await.map_err(|err| match err {
        aragog::Error::Conflict(err) => match err.arango_error {
            aragog::error::ArangoError::ArangoUniqueConstraintViolated => {
                return Error::Conflict(err.message)
            }
            _ => return Error::Internal(err.message),
        },
        _ => {
            return Error::Internal(format!("{}", err));
        }
    })?;
    Ok(res.id().clone())
}
