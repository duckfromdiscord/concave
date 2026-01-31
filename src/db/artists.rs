use deadpool_diesel::sqlite::Pool;
use diesel::{
    ExpressionMethods, QueryDsl, RunQueryDsl, Selectable, delete, dsl::insert_into, prelude::*,
};

use crate::{ConcaveError, core::artist, schema::artists};

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = artists)]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub mbid: Option<String>,
}

pub async fn remove_artist(artist_id: i32, pool: &Pool) -> Result<usize, ConcaveError> {
    let conn = pool.get().await.unwrap();
    Ok(conn
        .interact(move |conn| {
            delete(artists::table.filter(artists::id.eq(artist_id))).execute(conn)
        })
        .await??)
}

pub async fn add_artist_by_mbid(artist_mbid: String, pool: &Pool) -> Result<usize, ConcaveError> {
    let artist = artist::get_by_mbid(artist_mbid).await?;
    add_artist(artist.name, artist.mbid, pool).await
}

pub async fn add_artist(
    name: String,
    mbid: Option<String>,
    pool: &Pool,
) -> Result<usize, ConcaveError> {
    let conn = pool.get().await.unwrap();
    Ok(conn
        .interact(move |conn| {
            insert_into(artists::table)
                .values((
                    artists::name.eq(name),
                    artists::mbid.eq::<Option<String>>(mbid),
                ))
                .returning(artists::id)
                .execute(conn)
        })
        .await??)
}

pub async fn list_artists(pool: &Pool) -> Result<Vec<Artist>, ConcaveError> {
    let conn = pool.get().await.unwrap();
    Ok(conn
        .interact(move |conn| {
            artists::table
                .select(Artist::as_select())
                .distinct()
                .get_results::<Artist>(conn)
        })
        .await??)
}

pub async fn get_artist_by_id(artist_id: i32, pool: &Pool) -> Result<Option<Artist>, ConcaveError> {
    let conn = pool.get().await.unwrap();
    Ok(conn
        .interact(move |conn| {
            artists::table
                .select(Artist::as_select())
                .filter(artists::id.eq(artist_id))
                .first::<Artist>(conn)
                .optional()
        })
        .await??)
}
