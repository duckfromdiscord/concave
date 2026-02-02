// https://developer.ticketmaster.com/
pub mod json;

use crate::{ConcaveError, core::artist::get_mbartist_by_mbid, db};
use deadpool_diesel::sqlite::Pool;
use json::{AttractionSearchResponse, EventSearchResponse};
use reqwest::Client;

pub fn get_required_data() -> Vec<(String, String)> {
    vec![("att_id".to_string(), "Ticketmaster ID".to_string())]
}

pub async fn search_events(
    keyword: Option<String>,
    event_id: Option<String>,
    attraction_id: Option<String>,
    api_key: String,
    client: Client,
) -> Result<EventSearchResponse, ConcaveError> {
    let mut query = vec![];
    if let Some(keyword) = keyword {
        query.push(("keyword", keyword));
    }
    if let Some(event_id) = event_id {
        query.push(("id", event_id));
    }
    if let Some(attraction_id) = attraction_id {
        query.push(("attractionId", attraction_id));
    }
    query.push(("apikey", api_key));
    let req = client
        .get("https://app.ticketmaster.com/discovery/v2/events.json")
        .query(&query)
        .build()?;
    let res = client.execute(req).await?;
    Ok(res.json::<EventSearchResponse>().await?)
}

pub async fn search_artist(
    keyword: Option<String>,
    id: Option<String>,
    api_key: String,
    client: Client,
) -> Result<AttractionSearchResponse, ConcaveError> {
    let mut query = vec![];
    if let Some(keyword) = keyword {
        query.push(("keyword", keyword));
    }
    if let Some(id) = id {
        query.push(("id", id));
    }
    query.push(("apikey", api_key));
    query.push(("segmentId", "KZFzniwnSyZfZ7v7nJ".to_string())); // only search for musicians
    let req = client
        .get("https://app.ticketmaster.com/discovery/v2/attractions.json")
        .query(&query)
        .build()?;
    let res = client.execute(req).await?;
    Ok(res.json::<AttractionSearchResponse>().await?)
}

pub async fn search_artist_by_mbid(
    mbid: String,
    api_key: String,
    client: Client,
) -> Result<Option<String>, ConcaveError> {
    let artist = get_mbartist_by_mbid(&mbid).await?;
    let name = artist.name;

    let mut query = vec![];
    query.push(("keyword", &name));
    query.push(("apikey", &api_key));
    let found_artists = match search_artist(Some(name.clone()), None, api_key, client)
        .await?
        .embedded
    {
        None => return Err(ConcaveError::ArtistDoesNotExist),
        Some(emb) => emb.attractions,
    };
    for found in found_artists {
        match found.external_links {
            Some(links) => {
                match links.musicbrainz {
                    Some(mb) => {
                    if let Some(mb) = mb.first() {
                        let id = mb.clone().id.unwrap();
                        if id == mbid {
                            return Ok(Some(found.id.unwrap()));
                        }
                    };
                }
                    None => continue,
                }
            },
            None => {
                if found.name.is_some_and(|maybe_name| maybe_name == name) {
                    return Ok(Some(found.id.unwrap()));
                }
            }
        }
    }
    Ok(None)
}

pub async fn store_id(
    artist_id: i32,
    api_id: i32,
    tm_att_id: String,
    pool: &Pool,
) -> Result<bool, ConcaveError> {
    Ok(
        db::apis::add_artist_kv_for_api(artist_id, api_id, "att_id".to_string(), tm_att_id, pool)
            .await?,
    )
}

pub async fn get_id(artist_id: i32, api_id: i32, pool: &Pool) -> Result<String, ConcaveError> {
    Ok(
        db::apis::get_api_artist_data(api_id, artist_id, "att_id".to_string(), pool)
            .await?
            .ok_or(ConcaveError::MissingData)?,
    )
}

pub async fn add_api(pool: &Pool) -> Result<i32, ConcaveError> {
    Ok(db::apis::add_api("ticketmaster".to_string(), pool).await?)
}

pub async fn store_api_key(
    api_id: i32,
    api_key: String,
    pool: &Pool,
) -> Result<bool, ConcaveError> {
    Ok(db::apis::add_api_kv(api_id, "key".to_string(), api_key, pool).await?)
}

pub async fn get_api_key(api_id: i32, pool: &Pool) -> Result<Option<String>, ConcaveError> {
    Ok(db::apis::get_api_info(api_id, "key".to_string(), pool).await?)
}
