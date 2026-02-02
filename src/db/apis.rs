use crate::{
    ConcaveError,
    db::artists::Artist,
    schema::{apiinfo, apis, artistdata},
};
use deadpool_diesel::sqlite::Pool;
use diesel::{delete, insert_into, prelude::*};

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = apis)]
pub struct Api {
    pub id: i32,
    pub sitename: String,
}

#[derive(Queryable, Selectable, Associations, Identifiable)]
#[diesel(table_name = apiinfo)]
#[diesel(belongs_to(Api))]
pub struct ApiInfo {
    pub id: i32,
    pub api_id: i32,
    pub key: String,
    pub value: String,
}

#[derive(Queryable, Selectable, Associations, Identifiable)]
#[diesel(table_name = artistdata)]
#[diesel(belongs_to(Artist))]
#[diesel(belongs_to(Api))]
//#[diesel(primary_key(id))]
pub struct ArtistData {
    pub id: i32,
    pub artist_id: i32,
    pub api_id: i32,
    pub key: String,
    pub value: Option<String>,
}

pub async fn add_api(sitename: String, pool: &Pool) -> Result<i32, ConcaveError> {
    let conn = pool.get().await.unwrap();
    Ok(conn
        .interact(move |conn| {
            insert_into(apis::table)
                .values(apis::sitename.eq(sitename))
                .returning(apis::id)
                .get_result(conn)
        })
        .await??)
}

pub async fn add_api_kv(
    id: i32,
    key: String,
    value: String,
    pool: &Pool,
) -> Result<bool, ConcaveError> {
    let conn = pool.get().await.unwrap();
    Ok(conn
        .interact(move |conn| {
            insert_into(apiinfo::table)
                .values((
                    apiinfo::api_id.eq(id),
                    apiinfo::key.eq(key),
                    apiinfo::value.eq(value),
                ))
                .execute(conn)
                .map(|x| x != 0)
        })
        .await??)
}

pub async fn add_artist_kv_for_api(
    artist_id: i32,
    api_id: i32,
    key: String,
    value: String,
    pool: &Pool,
) -> Result<bool, ConcaveError> {
    let conn = pool.get().await.unwrap();
    Ok(conn
        .interact(move |conn| {
            insert_into(artistdata::table)
                .values((
                    artistdata::artist_id.eq(artist_id),
                    artistdata::api_id.eq(api_id),
                    artistdata::key.eq(key),
                    artistdata::value.eq(value),
                ))
                .execute(conn)
                .map(|x| x != 0)
        })
        .await??)
}

pub async fn get_api(api_id: i32, pool: &Pool) -> Result<Option<Api>, ConcaveError> {
    let conn = pool.get().await.unwrap();
    Ok(conn
        .interact(move |conn| {
            apis::table
                .filter(apis::id.eq(api_id))
                .first::<Api>(conn)
                .optional()
        })
        .await??)
}

pub async fn get_api_info(
    api_id: i32,
    key: String,
    pool: &Pool,
) -> Result<Option<String>, ConcaveError> {
    let conn = pool.get().await.unwrap();
    Ok(conn
        .interact(move |conn| {
            apiinfo::table
                .filter(apiinfo::api_id.eq(api_id))
                .filter(apiinfo::key.eq(key))
                .select(apiinfo::value)
                .first::<String>(conn)
                .optional()
        })
        .await??)
}

pub async fn get_api_infos(
    api_id: i32,
    keys: Vec<String>,
    pool: &Pool,
) -> Result<Option<Vec<(String, String)>>, ConcaveError> {
    let conn = pool.get().await.unwrap();
    if keys.len() == 0 {
        return Ok(Some(vec![]));
    }
    Ok(conn
        .interact(move |conn| {
            apiinfo::table
                .filter(apiinfo::api_id.eq(api_id))
                .filter(apiinfo::key.eq_any(keys))
                .select((apiinfo::key, apiinfo::value))
                .get_results::<(String, String)>(conn)
                .optional()
        })
        .await??)
}

pub async fn get_all_api_infos(
    api_id: i32,
    pool: &Pool,
) -> Result<Option<Vec<(String, String)>>, ConcaveError> {
    let conn = pool.get().await.unwrap();
    Ok(conn
        .interact(move |conn| {
            apiinfo::table
                .filter(apiinfo::api_id.eq(api_id))
                .select((apiinfo::key, apiinfo::value))
                .get_results::<(String, String)>(conn)
                .optional()
        })
        .await??)
}

pub async fn get_api_artist_data(
    api_id: i32,
    artist_id: i32,
    key: String,
    pool: &Pool,
) -> Result<Option<String>, ConcaveError> {
    let conn = pool.get().await.unwrap();
    Ok(conn
        .interact(move |conn| {
            artistdata::table
                .filter(artistdata::api_id.eq(api_id))
                .filter(artistdata::artist_id.eq(artist_id))
                .filter(artistdata::key.eq(key))
                .select(artistdata::value)
                .first::<String>(conn)
                .optional()
        })
        .await??)
}

pub async fn get_api_artist_data_all(
    artist_id: i32,
    pool: &Pool,
) -> Result<Option<Vec<(String, String)>>, ConcaveError> {
    let conn = pool.get().await.unwrap();
    Ok(conn
        .interact(move |conn| {
            artistdata::table
                .filter(artistdata::artist_id.eq(artist_id))
                .select((artistdata::key, artistdata::value))
                .load::<(String, String)>(conn)
                .optional()
        })
        .await??)
}

pub async fn list_apis(pool: &Pool) -> Result<Vec<Api>, ConcaveError> {
    let conn = pool.get().await.unwrap();
    Ok(conn
        .interact(move |conn| {
            apis::table
                .select(Api::as_select())
                .distinct()
                .get_results::<Api>(conn)
        })
        .await??)
}

pub async fn remove_api(api_id: i32, pool: &Pool) -> Result<usize, ConcaveError> {
    let conn = pool.get().await.unwrap();
    // remove KVs first - this will cause DB error if you don't do it (dependency)
    conn.interact(move |conn| {
        delete(apiinfo::table.filter(apiinfo::api_id.eq(api_id))).execute(conn)
    })
    .await??;
    Ok(conn
        .interact(move |conn| delete(apis::table.filter(apis::id.eq(api_id))).execute(conn))
        .await??)
}

pub async fn remove_kv(artist_id: i32, api_id: i32, key: String, pool: &Pool) -> Result<usize, ConcaveError> {
    let conn = pool.get().await.unwrap();
    Ok(conn
        .interact(move |conn| {
            delete(
                artistdata::table.filter(
                    artistdata::artist_id.eq(artist_id)
                )
                .filter(
                    artistdata::api_id.eq(api_id)
                )
                .filter(
                    artistdata::key.eq(key)
                )
            ).execute(conn)
        })
        .await??)
}