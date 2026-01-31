use bytes::Bytes;
use musicbrainz_rs::entity::artist::{Artist as MbArtist, ArtistSearchQuery};
use musicbrainz_rs::prelude::*;

use musicbrainz_rs::entity::relations::RelationContent::Url;

use crate::ConcaveError;

#[derive(Debug, Clone)]
pub struct ArtistResult {
    pub name: String,
    pub mbid: Option<String>,
    pub disambig: Option<String>,
}

fn find_deezer(haystack: &str) -> Option<String> {
    use {once_cell::sync::Lazy, regex::Regex};

    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(?m)https://www\.deezer\.com/artist/(?<id>.*)").unwrap());
    let Some(caps) = RE.captures(haystack) else {
        return None;
    };
    Some(caps["id"].to_string())
}

pub async fn find_artist_images(artist_mbid: &String) -> Result<Vec<String>, ConcaveError> {
    let artist = MbArtist::fetch()
        .id(&artist_mbid)
        .with_url_relations()
        .execute()
        .await
        .map_err(|_| ConcaveError::ArtistDoesNotExist)?;

    let mut results = vec![];

    match artist.relations {
        Some(relations) => {
            results.append(
                &mut relations
                    .iter()
                    // Streaming relationships
                    .filter(|r| {
                        r.type_id == "63cc5d1f-f096-4c94-a43f-ecb32ea94161"
                            || r.type_id == "769085a1-c2f7-4c24-a532-2375a77693bd"
                    })
                    .filter_map(|r| match &r.content {
                        Url(url) => find_deezer(&url.resource),
                        _ => None,
                    })
                    .map(|id| "https://api.deezer.com/artist/".to_owned() + &id + "/image?size=big")
                    .collect::<Vec<_>>(),
            );
        }
        None => (),
    };

    Ok(results)
}

pub async fn get_artist_image(artist_mbid: &String) -> Result<Option<Bytes>, ConcaveError> {
    let images = find_artist_images(artist_mbid).await?;
    match images.first() {
        Some(url) => Ok(Some(reqwest::get(url).await?.bytes().await.unwrap())),
        None => Ok(None),
    }
}

pub async fn get_mbartist_by_mbid(artist_mbid: &String) -> Result<MbArtist, ConcaveError> {
    Ok(MbArtist::fetch()
        .id(&artist_mbid)
        .execute()
        .await
        .map_err(|_| ConcaveError::ArtistDoesNotExist)?)
}

pub async fn get_by_mbid(artist_mbid: String) -> Result<ArtistResult, ConcaveError> {
    Ok(ArtistResult {
        name: get_mbartist_by_mbid(&artist_mbid).await?.name,
        mbid: Some(artist_mbid),
        disambig: None,
    })
}

pub async fn search_artist(artist_name: String) -> Result<Vec<ArtistResult>, ConcaveError> {
    let query = ArtistSearchQuery::query_builder()
        .artist(&artist_name)
        .build();
    let query_result = MbArtist::search(query).execute().await?;
    Ok(query_result
        .entities
        .into_iter()
        .map(|e| ArtistResult {
            name: e.name,
            mbid: Some(e.id),
            disambig: match e.disambiguation.as_str() {
                "" => None,
                _ => Some(e.disambiguation),
            },
        })
        .collect::<Vec<_>>())
}
