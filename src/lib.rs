pub mod app;
pub mod core;
pub mod db;
pub mod schema;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConcaveError {
    #[error("artist does not exist")]
    ArtistDoesNotExist,
    #[error("missing extra data for artist")]
    MissingData,
    #[error("Source does not exist")]
    SourceDoesNotExist,
    #[error("artist has no mbid attached")]
    NoMbidError,
    #[error("musicbrainz error")]
    MusicBrainzError(#[from] musicbrainz_rs::Error),
    #[error("SQL error")]
    InteractError(#[from] deadpool_diesel::InteractError),
    #[error("SQL error")]
    DieselError(#[from] diesel::result::Error),
    #[error("password parsing error")]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error("request error")]
    RequestError(#[from] reqwest::Error),
    #[error("json parse error")]
    JsonParseError(#[from] serde_json::Error),
    #[error("invalid source")]
    InvalidSource,
}
