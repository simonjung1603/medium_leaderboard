#[cfg(feature = "server")]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Queryable, Selectable, Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema::submissions))]
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
}
