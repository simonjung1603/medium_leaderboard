// @generated automatically by Diesel CLI.

diesel::table! {
    clap_history (id) {
        id -> Int4,
        guid -> Text,
        clap_count -> Int4,
        timestamp -> Timestamptz,
    }
}

diesel::table! {
    submissions (guid) {
        guid -> Text,
        realname -> Text,
        username -> Text,
        latest_published_version -> Text,
        latest_published_at -> Int8,
        clap_count -> Int4,
        title -> Text,
        img_id -> Text,
        word_count -> Int4,
        clap_count_last_updated_at -> Timestamptz,
        details_last_updated_at -> Timestamptz,
    }
}

diesel::joinable!(clap_history -> submissions (guid));

diesel::allow_tables_to_appear_in_same_query!(clap_history, submissions);
