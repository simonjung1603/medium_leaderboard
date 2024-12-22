#[cfg(feature = "server")]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Insertable))]
#[cfg_attr(feature = "server", diesel(table_name = crate::schema::clap_history))]
#[cfg_attr(feature = "server", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct InsertClapHistory {
    pub guid: String,
    pub clap_count: i32,
}
