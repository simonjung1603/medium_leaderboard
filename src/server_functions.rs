use crate::models::*;
use chrono::{DateTime, Local, TimeDelta};
use dioxus::prelude::*;
use std::ops::Add;
use anyhow::anyhow;
use dioxus::logger::tracing;

#[cfg(feature = "server")]
use crate::db::*;

#[server(GetAllSubmissions)]
pub async fn get_all_submissions() -> Result<Vec<Submission>, ServerFnError> {
    use crate::schema::submissions::dsl::*;
    let FromContext::<DbPool>(pool) = extract().await?;
    let mut connection = pool.get()?;
    let all_submissions = submissions.filter(username.ne_all(vec!["vilovshka", "flawrite"]))
        .select(Submission::as_select())
        .order_by(clap_count.desc())
        .load(&mut connection)
        .expect("Error loading submissions.");

    Ok(all_submissions)
}

#[server(UpdateCategory)]
pub async fn update_category(update_guid: String, update_category: Category) -> Result<(), ServerFnError> {
    use crate::schema::submissions::dsl::*;
    let FromContext::<DbPool>(pool) = extract().await?;
    let mut connection = pool.get()?;

    if Ok(1) == diesel::update(submissions).filter(guid.eq(update_guid.clone())).set(category.eq(update_category)).execute(&mut connection) {
        tracing::info!("Updated category for {} to {:?}.", update_guid, update_category);
        return Ok(());
    }

    tracing::error!("Error updating category");
    Err(ServerFnError::new("Error updating category"))
}

#[server(GetLatestUpdateTime)]
pub async fn get_latest_and_next_update_time() -> Result<(DateTime<Local>, DateTime<Local>), ServerFnError> {
    use crate::schema::submissions::dsl::*;
    let FromContext::<DbPool>(pool) = extract().await?;
    let mut connection = pool.get()?;

    let latest_update_time = match submissions
        .select(clap_count_last_updated_at)
        .order_by(clap_count_last_updated_at.desc())
        .first::<chrono::DateTime<chrono::Local>>(&mut connection)
    {
        Ok(db_time) => chrono::DateTime::from(db_time),
        Err(_) => Local::now(),
    };

    Ok((
        latest_update_time,
        latest_update_time.add(TimeDelta::minutes(15)),
    ))
}
