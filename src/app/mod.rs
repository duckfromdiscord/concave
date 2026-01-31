use std::collections::HashMap;

use crate::ConcaveError;
use crate::core::artist::get_artist_image;
use crate::core::event::Location;
use crate::core::sources::SourceInfo;
use crate::db::AppState;
use actix_web::http::StatusCode;
use actix_web::web::Redirect;
use actix_web::{Either as ActixEither, Responder, get, post};
use actix_web::{HttpResponse, web};
use deadpool_diesel::sqlite::Pool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct TryImage {
    pub mbid: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SearchArtist {
    //pub name: Option<String>,
    //pub mbid: Option<String>,
    pub query: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SearchArtistResponseArtist {
    pub name: String,
    pub mbid: String,
    pub disambig: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct TryImageResponse {
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SearchArtistResponse {
    pub error: Option<String>,
    pub results: Vec<SearchArtistResponseArtist>,
}

#[get("/api/try_image")]
pub async fn try_image(_data: web::Data<AppState>, info: web::Query<TryImage>) -> impl Responder {
    if let Some(id) = &info.mbid {
        match get_artist_image(id).await {
            Ok(bytes) => match bytes {
                Some(bytes) => ActixEither::Left(HttpResponse::build(StatusCode::OK).body(bytes)),
                None => ActixEither::Right(Redirect::to("/empty.png").temporary()),
            },
            Err(err) => ActixEither::Left(
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(TryImageResponse {
                    error: Some(err.to_string()),
                }),
            ),
        }
    } else {
        ActixEither::Left(HttpResponse::build(StatusCode::OK).json(TryImageResponse {
            error: Some("No artist supplied".to_string()),
        }))
    }
}

#[get("/api/search_artist")]
pub async fn search_artist(
    _data: web::Data<AppState>,
    info: web::Query<SearchArtist>,
) -> HttpResponse {
    match &info.query {
        Some(query) => {
            let mut results = vec![];

            // if name resolves as a UUID, search it as a mbid first.
            let uuid = uuid::Uuid::parse_str(query);
            match uuid {
                Ok(uuid) => {
                    let maybe_artist = crate::core::artist::get_by_mbid(uuid.to_string()).await;
                    match maybe_artist {
                        Ok(artist) => results.push(SearchArtistResponseArtist {
                            name: artist.name,
                            mbid: artist.mbid.unwrap(),
                            disambig: artist.disambig,
                        }),
                        Err(_err) => (),
                    };
                }
                Err(_) => (),
            }

            // if for some crazy reason we're working with an artist that has a UUID as a name let's also search the uuid itself.
            // i've seen worse.

            results.append(
                &mut crate::core::artist::search_artist(query.to_string())
                    .await
                    .unwrap()
                    .iter()
                    .map(|a| SearchArtistResponseArtist {
                        name: a.name.clone(),
                        mbid: a.mbid.clone().unwrap(),
                        disambig: a.disambig.clone(),
                    })
                    .collect::<Vec<_>>(),
            );

            HttpResponse::build(StatusCode::OK).json(SearchArtistResponse {
                results: results,
                error: None,
            })
        }
        None => HttpResponse::build(StatusCode::OK).json(SearchArtistResponse {
            results: vec![],
            error: Some("No artist supplied".to_string()),
        }),
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AddArtist {
    pub mbid: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct AddArtistResponse {
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct AddApiResponse {
    pub id: Option<i32>,
    pub error: Option<String>,
}

#[get("/api/add_artist")]
pub async fn add_artist(data: web::Data<AppState>, info: web::Query<AddArtist>) -> HttpResponse {
    let maybe_artist = crate::core::artist::get_by_mbid(info.mbid.to_string()).await;
    match maybe_artist {
        Ok(artist) => {
            crate::db::artists::add_artist(artist.name, artist.mbid, &data.db)
                .await
                .unwrap();
            HttpResponse::build(StatusCode::OK).json(AddArtistResponse { error: None })
        }
        Err(err) => HttpResponse::build(StatusCode::OK).json(AddArtistResponse {
            error: Some(err.to_string()),
        }),
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeleteArtist {
    pub id: Option<i32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeleteApi {
    pub id: Option<i32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AddApi {
    pub name: String,
    pub params: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteArtistResponse {
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteApiResponse {
    pub error: Option<String>,
}

#[get("/api/delete_artist")]
pub async fn delete_artist(
    data: web::Data<AppState>,
    info: web::Query<DeleteArtist>,
) -> HttpResponse {
    match info.id {
        Some(id) => HttpResponse::build(StatusCode::OK).json(DeleteArtistResponse {
            error: crate::db::artists::remove_artist(id, &data.db)
                .await
                .err()
                .map(|_| "Database error".to_string()),
        }),
        None => HttpResponse::build(StatusCode::OK).json(DeleteArtistResponse {
            error: Some("No artist supplied".to_string()),
        }),
    }
}

#[get("/api/delete_api")]
pub async fn delete_api(data: web::Data<AppState>, info: web::Query<DeleteApi>) -> HttpResponse {
    match info.id {
        Some(id) => HttpResponse::build(StatusCode::OK).json(DeleteApiResponse {
            error: crate::db::apis::remove_api(id, &data.db)
                .await
                .err()
                .map(|_| "Database error".to_string()),
        }),
        None => HttpResponse::build(StatusCode::OK).json(DeleteApiResponse {
            error: Some("No artist supplied".to_string()),
        }),
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ListArtistItem {
    pub id: i32,
    pub name: String,
    pub mbid: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ListApiItem {
    pub id: i32,
    pub name: String,
    pub readable_name: String,
    pub params: HashMap<String, String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ListArtistResponse {
    pub artists: Vec<ListArtistItem>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ListApiResponse {
    pub apis: Option<Vec<ListApiItem>>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ListSourceResponse {
    pub sources: Vec<SourceInfo>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Event {
    pub date: String,
    pub name: String,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct LookupResponse {
    pub events: Option<Vec<Event>>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LookupAt {
    pub api_id: i32,
    pub artist_id: i32,
}

#[get("/api/list_artists")]
pub async fn list_artists(data: web::Data<AppState>) -> HttpResponse {
    let artists = crate::db::artists::list_artists(&data.db)
        .await
        .unwrap()
        .iter()
        .map(|a| ListArtistItem {
            id: a.id.clone(),
            name: a.name.clone(),
            mbid: a.mbid.clone(),
        })
        .collect::<Vec<_>>();
    HttpResponse::build(StatusCode::OK).json(ListArtistResponse {
        artists,
        error: None,
    })
}

#[post("/api/add_api")]
pub async fn add_api(data: web::Data<AppState>, info: web::Json<AddApi>) -> HttpResponse {
    let maybe_new_source = crate::core::sources::Source::from_query(info.into_inner());
    match maybe_new_source {
        Ok(source) => {
            let id = match crate::db::apis::add_api(source.get_name(), &data.db).await {
                Ok(id) => id,
                Err(err) => {
                    return HttpResponse::build(StatusCode::OK).json(AddApiResponse {
                        id: None,
                        error: Some(err.to_string()),
                    });
                }
            };
            for param in source.get_params() {
                match crate::db::apis::add_api_kv(id, param.0, param.1, &data.db).await {
                    Ok(_) => (),
                    Err(err) => {
                        return HttpResponse::build(StatusCode::OK).json(AddApiResponse {
                            id: None,
                            error: Some(err.to_string()),
                        });
                    }
                }
            }
            HttpResponse::build(StatusCode::OK).json(AddApiResponse {
                id: Some(id),
                error: None,
            })
        }
        Err(err) => HttpResponse::build(StatusCode::OK).json(AddApiResponse {
            id: None,
            error: Some(err.to_string()),
        }),
    }
}

async fn get_apis(pool: &Pool) -> Result<Vec<ListApiItem>, String> {
    let sources = crate::core::sources::Source::get_sources_map();

    let mut apis = match crate::db::apis::list_apis(pool).await {
        Ok(apis) => apis
            .iter()
            .map(|a| ListApiItem {
                id: a.id.clone(),
                name: a.sitename.clone(),
                readable_name: sources.get(&a.sitename).unwrap().readable.clone(),
                params: HashMap::new(),
            })
            .collect::<Vec<_>>(),
        Err(err) => return Err(err.to_string()),
    };

    for api in apis.iter_mut() {
        let this_source = sources.get(&api.name).unwrap();
        let params = crate::db::apis::get_api_infos(api.id, this_source.clone().params, pool).await;
        match params {
            Ok(params) => {
                // if the DB returned nothing there's no params!
                for param in params.unwrap_or(vec![]) {
                    api.params.insert(param.0, param.1);
                }
            }
            Err(err) => return Err(err.to_string()),
        }
    }
    Ok(apis)
}

#[get("/api/list_apis")]
pub async fn list_apis(data: web::Data<AppState>) -> HttpResponse {
    let apis = get_apis(&data.db).await;
    HttpResponse::build(StatusCode::OK).json(match apis {
        Ok(apis) => ListApiResponse {
            apis: Some(apis),
            error: None,
        },
        Err(err) => ListApiResponse {
            apis: None,
            error: Some(err),
        },
    })
}

// List available sources that can be added
#[get("/api/get_sources")]
pub async fn get_sources(_data: web::Data<AppState>) -> HttpResponse {
    let sources = crate::core::sources::Source::get_sources();
    HttpResponse::build(StatusCode::OK).json(ListSourceResponse {
        sources,
        error: None,
    })
}

fn option_loc(loc: &Option<Location>) -> Option<String> {
    if let Some(loc) = loc {
        if let Some(venue) = &loc.venue {
            let mut ret = loc
                .coords
                .as_ref()
                .map(|coords| " ".to_owned() + &coords)
                .unwrap_or("".to_string());
            if let Some(venue_loc) = &loc.loc {
                ret = venue.to_owned() + ", " + &venue_loc + &ret;
            } else {
                ret = venue.to_owned() + " " + &ret;
            }
            Some(ret)
        } else {
            None
        }
    } else {
        None
    }
}

#[get("/api/lookup_at")]
pub async fn lookup_at(data: web::Data<AppState>, info: web::Query<LookupAt>) -> HttpResponse {
    let apis = get_apis(&data.db).await;
    let api = match crate::db::apis::get_api(info.api_id, &data.db).await {
        Ok(any_api) => match any_api {
            Some(api) => api,
            None => {
                return HttpResponse::build(StatusCode::OK).json(LookupResponse {
                    events: None,
                    error: Some(ConcaveError::SourceDoesNotExist.to_string()),
                });
            }
        },
        Err(err) => {
            return HttpResponse::build(StatusCode::OK).json(LookupResponse {
                events: None,
                error: Some(err.to_string()),
            });
        }
    };
    let params = match crate::db::apis::get_all_api_infos(info.api_id, &data.db).await {
        Ok(any_infos) => {
            let mut map = HashMap::new();
            for param in any_infos.unwrap_or(vec![]) {
                map.insert(param.0, param.1);
            }
            map
        }
        Err(err) => {
            return HttpResponse::build(StatusCode::OK).json(LookupResponse {
                events: None,
                error: Some(err.to_string()),
            });
        }
    };
    match apis {
        Ok(apis) => {
            let source =
                match crate::core::sources::Source::from_name_and_params(api.sitename, params) {
                    Ok(source) => source,
                    Err(err) => {
                        return HttpResponse::build(StatusCode::OK).json(LookupResponse {
                            events: None,
                            error: Some(err.to_string()),
                        });
                    }
                };
            match source
                .lookup(info.artist_id, info.api_id, &data.db, data.client.clone())
                .await
            {
                Ok(result) => {
                    return HttpResponse::build(StatusCode::OK).json(LookupResponse {
                        events: Some(
                            result
                                .iter()
                                .map(|e| Event {
                                    name: e.name.clone(),
                                    date: e.date.map_either(|f| f.to_rfc3339(), |g| g).to_string(),
                                    location: option_loc(&e.location),
                                })
                                .collect::<Vec<_>>(),
                        ),
                        error: None,
                    });
                }
                Err(err) => {
                    return HttpResponse::build(StatusCode::OK).json(LookupResponse {
                        events: None,
                        error: Some(err.to_string()),
                    });
                }
            }
        }
        Err(err) => {
            return HttpResponse::build(StatusCode::OK).json(LookupResponse {
                events: None,
                error: Some(err),
            });
        }
    }
}
