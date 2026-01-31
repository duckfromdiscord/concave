pub mod musicbrainz;
pub mod ticketmaster;

use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};
use deadpool_diesel::sqlite::Pool;
use either::Either;
use reqwest::Client;
use serde::Serialize;

use crate::{ConcaveError, app::AddApi, core::artist::get_mbartist_by_mbid};

use super::event::{Event, Location};

#[derive(Clone, Debug)]
pub enum Source {
    MbzEvents(),
    // API key
    Ticketmaster(String),
}

#[derive(Debug, Serialize, Clone)]
pub struct SourceInfo {
    pub name: String,
    pub readable: String,
    pub params: Vec<String>,
}

impl Source {
    pub fn get_sources() -> Vec<SourceInfo> {
        vec![
            SourceInfo {
                name: "mbz".to_string(),
                readable: "Musicbrainz".to_string(),
                params: vec![],
            },
            SourceInfo {
                name: "ticketmaster".to_string(),
                readable: "Ticketmaster".to_string(),
                params: vec!["key".to_string()],
            },
        ]
    }
    pub fn get_sources_map() -> HashMap<String, SourceInfo> {
        let mut map = HashMap::new();
        for source in Self::get_sources() {
            map.insert(source.clone().name, source);
        }
        map
    }
    pub fn from_name_and_params(
        name: String,
        params: HashMap<String, String>,
    ) -> Result<Self, ConcaveError> {
        match name.as_str() {
            "mbz" => Ok(Self::MbzEvents()),
            "ticketmaster" => Ok(Self::Ticketmaster(
                params
                    .get("key")
                    .ok_or(ConcaveError::InvalidSource)?
                    .to_string(),
            )),
            _ => Err(ConcaveError::InvalidSource),
        }
    }
    pub fn from_query(query: AddApi) -> Result<Self, ConcaveError> {
        Self::from_name_and_params(query.name, query.params)
    }
    pub fn get_params(&self) -> HashMap<String, String> {
        match self {
            Self::MbzEvents() => HashMap::new(),
            Self::Ticketmaster(key) => {
                let mut map = HashMap::new();
                map.insert("key".to_string(), key.to_string());
                map
            }
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Self::MbzEvents() => "mbz".to_string(),
            Self::Ticketmaster(_) => "ticketmaster".to_string(),
        }
    }
    //TODO: decouple?
    pub async fn get_data_for(
        self,
        artist_id: i32,
        api_id: i32,
        pool: &Pool,
    ) -> Result<String, ConcaveError> {
        match self {
            Source::MbzEvents() => musicbrainz::get_id(artist_id, pool).await,
            Source::Ticketmaster(_) => ticketmaster::get_id(artist_id, api_id, pool).await,
        }
    }
    pub async fn lookup(
        self,
        artist_id: i32,
        api_id: i32,
        pool: &Pool,
        client: Client,
    ) -> Result<Vec<Event>, ConcaveError> {
        let data = self.clone().get_data_for(artist_id, api_id, pool).await?;
        match self {
            Source::MbzEvents() => musicbrainz::lookup(data).await,
            Source::Ticketmaster(api_key) => {
                let res = ticketmaster::search_events(
                    None,
                    None,
                    Some(data.to_string()),
                    api_key.to_string(),
                    client,
                )
                .await?;
                if let Some(results) = res.embedded {
                    let events = results.events.unwrap();
                    let mut return_events = vec![];
                    for event in events {
                        let dates = event.dates.unwrap();
                        if let Some(date) = dates.clone().start.or(dates.end) {
                            let either = if let Some(date_time) = date.date_time {
                                Some(Either::Left(
                                    DateTime::parse_from_rfc3339(&date_time).unwrap().to_utc(),
                                ))
                            } else {
                                date.local_date.map(|local_date| {
                                    Either::Right(
                                        NaiveDate::parse_from_str(&local_date, "%Y-%m-%d").unwrap(), // a timezone won't help if we only have a date
                                    )
                                })
                            };
                            if let Some(date) = either {
                                let event_embedded = event.embedded.unwrap();
                                let venues = event_embedded.venues.unwrap();
                                let venue = venues.first().unwrap();
                                let loc = Location {
                                    coords: venue.clone().location.map(|l| {
                                        l.latitude.unwrap() + ", " + l.longitude.unwrap().as_str()
                                    }),
                                    venue: venue.clone().name,
                                    loc: venue.clone().city.map(|c| c.name.unwrap()),
                                };
                                return_events.push(Event {
                                    date,
                                    name: event.name.unwrap(),
                                    location: Some(loc),
                                })
                            }
                        }
                    }
                    return Ok(return_events);
                }
                Err(ConcaveError::ArtistDoesNotExist)
            } //_ => todo!(),
        }
    }
}

fn coincides(a: Either<DateTime<Utc>, NaiveDate>, b: Either<DateTime<Utc>, NaiveDate>) -> bool {
    let a_nd = match a {
        Either::Left(dt) => dt.naive_local().date(),
        Either::Right(nd) => nd,
    };
    let b_nd = match b {
        Either::Left(dt) => dt.naive_local().date(),
        Either::Right(nd) => nd,
    };
    (b_nd - a_nd).num_days().abs() < 2
}

/*pub async fn research_artist(
    artist_id: i32,
    sources: Vec<Source>,
    remove_duplicate_dates: bool,
    client: Client,
) -> Result<Vec<Event>, ConcaveError> {
    let mut events: Vec<Event> = vec![];
    for s in sources {
        match s.lookup(s.data_for(artist_id), client.clone()).await {
            Ok(mut found_events) => match remove_duplicate_dates {
                true => events.append(
                    &mut found_events
                        .iter()
                        .filter(|new_event| {
                            events
                                .iter()
                                .any(|old_event| coincides(old_event.date, new_event.date))
                        })
                        .cloned()
                        .collect::<Vec<_>>(),
                ),
                false => events.append(&mut found_events),
            },
            Err(err) => match err {
                ConcaveError::NoMbidError => (),
                _ => {
                    return Err(err);
                }
            },
        }
    }
    Ok(events)
}*/
