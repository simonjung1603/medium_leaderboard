use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use diesel::prelude::*;

#[derive(Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Queryable, Selectable, Identifiable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema::submissions))]
#[cfg_attr(feature = "server", diesel(primary_key(guid)))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct Submission {
    pub guid: String,
    pub realname: String,
    pub username: String,
    pub latest_published_version: String,
    pub latest_published_at: i64,
    pub clap_count: i32,
    pub title: String,
    pub img_id: String,
    pub word_count: i32,
    pub clap_count_last_updated_at: chrono::DateTime<chrono::Local>,
    pub details_last_updated_at: chrono::DateTime<chrono::Local>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema::submissions))]
#[cfg_attr(feature = "server", diesel(primary_key(guid)))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct InsertSubmission {
    pub guid: String,
    pub realname: String,
    pub username: String,
    pub latest_published_version: String,
    pub latest_published_at: i64,
    pub clap_count: i32,
    pub title: String,
    pub img_id: String,
    pub word_count: i32,
}
