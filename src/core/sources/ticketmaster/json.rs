use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct TmDateData {
    #[serde(rename = "localDate")]
    pub local_date: Option<String>,
    #[serde(rename = "localTime")]
    pub local_time: Option<String>,
    #[serde(rename = "dateTime")]
    pub date_time: Option<String>,
    #[serde(rename = "dateTBD")]
    pub date_tbd: Option<bool>,
    #[serde(rename = "dateTBA")]
    pub date_tba: Option<bool>,
    #[serde(rename = "timeTBA")]
    pub time_tba: Option<bool>,
    #[serde(rename = "noSpecificTime")]
    pub no_specific_time: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TmDateStatus {
    pub code: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TmDates {
    pub start: Option<TmDateData>,
    pub end: Option<TmDateData>,
    pub status: Option<TmDateStatus>,
    #[serde(rename = "spanMultipleDays")]
    pub span_multiple_days: Option<bool>,
    pub timezone: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TmIdPair {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TmClassification {
    pub primary: Option<bool>,
    pub segment: Option<TmIdPair>,
    pub genre: Option<TmIdPair>,
    #[serde(rename = "subGenre")]
    pub subgenre: Option<TmIdPair>,
    #[serde(rename = "type")]
    pub c_type: Option<TmIdPair>,
    #[serde(rename = "subType")]
    pub sub_type: Option<TmIdPair>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TmAgeRestrictions {
    #[serde(rename = "legalAgeEnforced")]
    pub legal_age_enforced: Option<bool>,
    pub id: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TmCity {
    pub name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TmState {
    pub name: Option<String>,
    #[serde(rename = "stateCode")]
    pub state_code: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TmCountry {
    pub name: Option<String>,
    #[serde(rename = "countryCode")]
    pub country_code: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TmAddress {
    pub line1: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TmLatLong {
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TmVenue {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub venue_type: Option<String>,
    pub id: Option<String>,
    pub test: Option<bool>,
    pub url: Option<String>,
    pub timezone: Option<String>,
    pub city: Option<TmCity>,
    pub state: Option<TmState>,
    pub country: Option<TmCountry>,
    pub address: Option<TmAddress>,
    pub location: Option<TmLatLong>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TmMusicbrainz {
    pub id: Option<String>,
    pub url: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TmUrl {
    pub url: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TmExternalLinks {
    pub musicbrainz: Option<Vec<TmMusicbrainz>>, // was so happy to see this
    pub spotify: Option<Vec<TmUrl>>,
    pub wiki: Option<Vec<TmUrl>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TmAttraction {
    pub name: Option<String>,
    #[serde(rename = "legalAgeEnforced")]
    pub attraction_type: Option<String>,
    pub id: Option<String>,
    pub test: Option<bool>,
    #[serde(rename = "externalLinks")]
    pub external_links: Option<TmExternalLinks>,
    pub classifications: Option<Vec<TmClassification>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TmEventEmbedded {
    pub venues: Option<Vec<TmVenue>>,
    pub attractions: Option<Vec<TmAttraction>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TmEvent {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    pub id: Option<String>,
    pub dates: Option<TmDates>,
    pub classifications: Option<Vec<TmClassification>>,
    #[serde(rename = "ageRestrictions")]
    pub age_restrictions: Option<TmAgeRestrictions>,
    #[serde(rename = "_embedded")]
    pub embedded: Option<TmEventEmbedded>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EventSearchResponseEmbedded {
    pub events: Option<Vec<TmEvent>>,
}

/*#[derive(Deserialize, Debug, Clone)]
pub struct SearchResponseLinks {

}*/

#[derive(Deserialize, Debug, Clone)]
pub struct SearchResponsePage {
    pub size: Option<i32>,
    #[serde(rename = "totalElements")]
    pub total_elements: Option<i32>,
    #[serde(rename = "totalPages")]
    pub total_pages: Option<i32>,
    pub number: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EventSearchResponse {
    #[serde(rename = "_embedded")]
    pub embedded: Option<EventSearchResponseEmbedded>,
    //pub _links: SearchResponseLinks,
    pub page: Option<SearchResponsePage>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AttractionSearchResponseEmbedded {
    pub attractions: Vec<TmAttraction>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AttractionSearchResponse {
    #[serde(rename = "_embedded")]
    pub embedded: Option<AttractionSearchResponseEmbedded>,
    //pub _links: SearchResponseLinks,
    pub page: Option<SearchResponsePage>,
}
