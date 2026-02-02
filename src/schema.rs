// @generated automatically by Diesel CLI.

diesel::table! {
    apiinfo (id) {
        id -> Integer,
        api_id -> Integer,
        key -> Text,
        value -> Text,
    }
}

diesel::table! {
    apis (id) {
        id -> Integer,
        sitename -> Text,
    }
}

diesel::table! {
    artistdata (id) {
        id -> Integer,
        artist_id -> Integer,
        key -> Text,
        value -> Text,
        api_id -> Integer,
    }
}

diesel::table! {
    artists (id) {
        id -> Integer,
        name -> Text,
        mbid -> Nullable<Text>,
    }
}

diesel::table! {
    logins (id) {
        id -> Integer,
        user -> Text,
        pass -> Text,
    }
}

diesel::joinable!(apiinfo -> apis (api_id));
diesel::joinable!(artistdata -> artists (artist_id));

diesel::allow_tables_to_appear_in_same_query!(
    apiinfo,
    apis,
    artistdata,
    artists,
    logins,
);
