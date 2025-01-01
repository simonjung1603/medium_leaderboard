#[cfg(feature = "server")]
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    deserialize,
    backend::Backend,
    prelude::*,
    sql_types::SmallInt,
    AsExpression,
    serialize::{Output, ToSql},
};
use serde::{Deserialize, Serialize};

#[repr(i16)]
#[derive(Debug, Default, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(FromSqlRow, AsExpression))]
#[cfg_attr(feature = "server", diesel(sql_type = SmallInt))]
pub enum Category {
    #[default]
    None = 0,
    Poetry = 1,
    Fiction = 2,
    PersonalEssay = 3,
}

#[cfg(feature = "server")]
impl<DB> FromSql<SmallInt, DB> for Category
    where
        DB: Backend,
        i16: FromSql<SmallInt, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        match i16::from_sql(bytes)? {
            0 => Ok(Category::None),
            1 => Ok(Category::Poetry),
            2 => Ok(Category::Fiction),
            3 => Ok(Category::PersonalEssay),
            x => Err(format!("Unrecognized variant {}", x).into()),
        }
    }
}

#[cfg(feature = "server")]
impl<DB> ToSql<SmallInt, DB> for Category
    where DB: Backend,
          i16: ToSql<SmallInt, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> diesel::serialize::Result {
        match self {
            Category::None => 0.to_sql(out),
            Category::Poetry => 1.to_sql(out),
            Category::Fiction => 2.to_sql(out),
            Category::PersonalEssay => 3.to_sql(out),
        }
    }
}


#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
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
    pub category: Category,
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
