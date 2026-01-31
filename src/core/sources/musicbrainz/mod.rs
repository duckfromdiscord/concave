use crate::{
    ConcaveError,
    core::{
        event::{Event, Location},
        sources::get_mbartist_by_mbid,
    },
};

use deadpool_diesel::sqlite::Pool;
use either::Either;
use musicbrainz_rs::{
    Browse,
    entity::{event::Event as MbEvent, relations::RelationContent},
};

pub fn get_required_data() -> Vec<(String, String)> {
    // "mbid" is in artists table but this is only for artistinfo
    // TODO: move mbid to artistinfo
    vec![]
}

pub async fn get_id(artist_id: i32, pool: &Pool) -> Result<String, ConcaveError> {
    Ok(crate::db::artists::get_artist_by_id(artist_id, pool)
        .await?
        .ok_or(ConcaveError::ArtistDoesNotExist)?
        .mbid
        .ok_or(ConcaveError::NoMbidError)?)
}

pub async fn lookup(mbid: String) -> Result<Vec<Event>, ConcaveError> {
    // verify the mbid exists
    let mbartist = get_mbartist_by_mbid(&mbid).await?;
    let events = MbEvent::browse()
        .by_artist(&mbartist.id)
        .with_area_relations()
        .with_place_relations()
        .execute()
        .await?;
    Ok(events.entities.iter()
                            .filter(|e| !e.cancelled.unwrap_or(false))
                            .filter(|e| e.life_span.as_ref().is_some())
                            .filter(|e| e.life_span.as_ref().unwrap().begin.is_some())
                            .map(|e| Event {
                                name: e.name.clone(),
                                date: Either::Right(e.life_span.as_ref().unwrap().begin.unwrap()),
                                location: e.relations.as_ref().unwrap_or(&vec![]).iter()
                                    // is main or guest performer
                                    .filter(|r|
                                        // these are two different things in MBZ (for place vs area to event relationships)
                                        r.relation_type == "held at" || r.relation_type == "held in")
                                    .filter_map(|r| match &r.content {
                                        RelationContent::Area(area) => Some(Location {
                                            coords: None,
                                            venue: None,
                                            loc: Some(area.name.clone())
                                        }),
                                        RelationContent::Place(place) => Some(match &place.area {
                                            Some(area) => Location {
                                            coords: place.coordinates.clone().map(|c| c.latitude.to_string() + ", " + c.longitude.to_string().as_str()),
                                            venue: Some(place.name.clone()),
                                            loc: Some(area.name.clone()),
                                        },
                                            None => Location {
                                            coords: place.coordinates.clone().map(|c| c.latitude.to_string() + ", " + c.longitude.to_string().as_str()),
                                            venue: Some(place.name.clone()),
                                            loc: None,
                                        }
                                        }),
                                        _ => None,
                                    })
                                    .collect::<Vec<_>>()
                                    .first().map(|p| Some(p)).unwrap_or(None).cloned()
                            })
                            .collect::<Vec<_>>())
}
