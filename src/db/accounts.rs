use crate::{ConcaveError, schema::logins};

const BCRYPT_COST: u32 = 12;

use deadpool_diesel::sqlite::Pool;
use diesel::{insert_into, prelude::*};

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = logins)]
pub struct Account {
    pub id: i32,
    pub user: String,
    pub pass: String,
}

pub async fn account_count(pool: &Pool) -> Result<i64, ConcaveError> {
    let conn = pool.get().await.unwrap();
    Ok(conn
        .interact(move |conn| logins::table.distinct().count().get_result::<i64>(conn))
        .await??)
}

pub async fn has_account(pool: &Pool) -> Result<bool, ConcaveError> {
    Ok(account_count(pool).await? > 0)
}

pub async fn add_account(user: String, pass: String, pool: &Pool) -> Result<bool, ConcaveError> {
    let conn = pool.get().await.unwrap();
    let hash = bcrypt::hash(pass, BCRYPT_COST)?;
    Ok(conn
        .interact(move |conn| {
            insert_into(logins::table)
                .values((logins::user.eq(user), logins::pass.eq(hash)))
                .execute(conn)
        })
        .await??
        > 0)
}

pub async fn get_userid(
    user: String,
    pass: String,
    pool: &Pool,
) -> Result<Option<i32>, ConcaveError> {
    let conn = pool.get().await.unwrap();
    let pair: Option<(i32, String)> = conn
        .interact(move |conn| {
            logins::table
                .filter(logins::user.eq(user))
                .select((logins::id, logins::pass))
                .limit(1)
                .get_result(conn)
                .optional()
        })
        .await??;
    // TODO: rewrite the function to be in common time
    match pair {
        None => Ok(None), // user not found
        Some((id, hashed_password)) => {
            let correct_password = bcrypt::verify(pass, &hashed_password)?;
            if correct_password {
                Ok(Some(id))
            } else {
                Ok(None)
            }
        }
    }
}
