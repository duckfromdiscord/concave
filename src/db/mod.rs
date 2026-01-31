use deadpool_diesel::sqlite::Pool;
use reqwest::Client;

pub mod accounts;
pub mod apis;
pub mod artists;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool,
    pub client: Client,
}

impl AppState {}
