// @generated automatically by Diesel CLI.

diesel::table! {
    submissions (guid) {
        guid -> Text,
        realname -> Text,
        username -> Text,
        latest_published_version -> Text,
        latest_published_at -> BigInt,
        clap_count -> Integer,
        title -> Text,
        img_id -> Text,
        word_count -> Integer,
    }
}
