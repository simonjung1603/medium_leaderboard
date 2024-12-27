mod graphql;

use std::time::Duration;
use anyhow::anyhow;
use chrono::TimeDelta;
use diesel::SelectableHelper;
use dioxus::logger::tracing;
use rss::Channel;
use crate::db::DbPool;
use crate::models::{InsertClapHistory, InsertSubmission, Submission};
use crate::server::graphql::{GRAPHQL_ENDPOINT, GraphQlRequest};
use crate::server::graphql::clap_count_query::{ClapCountQuery, ClapCountResult};
use crate::server::graphql::story_details_query::{PostPageQuery, PostPageResult};
use diesel::{QueryDsl, RunQueryDsl, Insertable, associations::HasTable, ExpressionMethods};
use reqwest::{Method, Request};

async fn update_rss(pool: &DbPool) -> anyhow::Result<()> {
    use crate::schema::submissions::dsl;
    tracing::info!("Fetching rss feed.");

    let response = reqwest::get("https://medium.com/feed/my-fair-lighthouse/tagged/mfl-contest")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&response[..])?;
    let mut connection = pool.get()?;

    for item in channel.items {
        let guid = item
            .guid
            .as_ref()
            .and_then(|uri| uri.value.split('/').last().map(|guid| guid.to_owned()));

        match guid {
            None => {
                tracing::warn!("Could not extract guid from rss item: {:?}", item);
                continue;
            }
            Some(guid) => {
                tracing::info!("Got guid: {}", guid);
                if dsl::submissions
                    .find(&guid)
                    .first::<Submission>(&mut connection)
                    .is_ok()
                {
                    tracing::info!("Submission for guid {} already present in db.", guid);
                    continue;
                }
                let new_submission = fetch_story_details(&guid).await?;

                let rows_affected = new_submission
                    .insert_into(dsl::submissions::table())
                    .execute(&mut connection)?;

                if rows_affected != 1 {
                    tracing::warn!("Insertion of submission failed: {}", guid);
                }
            }
        };
    }

    Ok(())
}

async fn update_story_details(
    _pool: &DbPool,
) -> anyhow::Result<()> {
    tracing::info!("Updating all story details.");

    Ok(())
}

#[allow(non_snake_case, unused)]
async fn fetch_story_details(postId: &str) -> anyhow::Result<InsertSubmission> {
    tracing::info!("Fetching details for guid {}.", postId);

    let response = reqwest::Client::new()
        .post(GRAPHQL_ENDPOINT)
        .json(&vec![GraphQlRequest::from(PostPageQuery { post_id: postId })])
        .send()
        .await?
        .json::<PostPageResult>()
        .await;

    if let Ok(response) = response {
        if response.len() != 1 {
            return Err(anyhow!(
                "Graphql response does not contain a single object."
            ));
        }
        let r = response
            .into_iter()
            .next()
            .ok_or(anyhow!("Unexpected error reading graphql response"))?
            .data
            .post_result;
        return Ok(InsertSubmission {
            guid: r.id,
            realname: r.creator.name,
            username: r.creator.username,
            latest_published_version: r.latest_published_version,
            latest_published_at: r.latest_published_at,
            clap_count: r.clap_count,
            title: r.title,
            img_id: r.preview_image.id,
            word_count: r.word_count,
        });
    }

    Err(anyhow!("Error fetching response: {:?}", response))
}

async fn update_claps(pool: &DbPool) -> anyhow::Result<()> {
    use crate::db::submissions::dsl as dsls;
    use crate::db::clap_history::dsl as dsl;
    let mut connection = pool.get()?;

    match dsls::submissions
        .select(dsls::clap_count_last_updated_at)
        .order_by(dsls::clap_count_last_updated_at.desc())
        .first::<chrono::DateTime<chrono::Local>>(&mut connection)
    {
        Ok(date_time) => {
            if chrono::Local::now().signed_duration_since(date_time) < TimeDelta::minutes(14) {
                tracing::info!("Checked within last 15 minutes, not checking again!");
                return Ok(());
            }
        }
        Err(err) => tracing::error!("Error fetching last update time: {}", err),
    }
    tracing::info!("Updating all clap counts");


    let submissions = dsls::submissions
        .select(Submission::as_select())
        .load(&mut connection)
        .expect("Error loading submissions.");

    let client = reqwest::Client::new();

    for submission in submissions {
        let clap_count = client
            .post(GRAPHQL_ENDPOINT)
            .json(&vec![GraphQlRequest::from(ClapCountQuery { post_id: &submission.guid, include_first_boosted_at: false })])
            .send()
            .await?
            .json::<ClapCountResult>()
            .await?
            .into_iter()
            .next()
            .ok_or(anyhow!(
                "Unexpected error reading clap_count graphql response"
            ))?
            .data
            .post_result
            .clap_count;

        tracing::info!("{}: {}", submission.guid, clap_count);

        if clap_count != submission.clap_count {
            tracing::info!(
                "{}: {} --> {}",
                submission.title,
                submission.clap_count,
                clap_count
            );

            let affected_rows = InsertClapHistory {
                guid: submission.guid.clone(),
                clap_count,
            }
                .insert_into(dsl::clap_history)
                .execute(&mut connection);

            if let Ok(1) = affected_rows {
                tracing::info!("Inserted into history.");
            } else {
                tracing::warn!("Inserting clap_count into history failed.")
            }

            if let Ok(1) = diesel::update(&submission)
                .set(dsls::clap_count.eq(clap_count))
                .execute(&mut connection)
            {
                tracing::info!("Updated entry in submissions.");
            } else {
                tracing::error!("Update in submissions failed or affected multiple rows!");
            }
        }

        if let Ok(1) = diesel::update(&submission)
            .set(dsls::clap_count_last_updated_at.eq(chrono::Local::now()))
            .execute(&mut connection)
        {
            tracing::info!("Updated clap_count_last_updated_at.");
        } else {
            tracing::error!("Updating clap_count_last_updated_at failed.");
        }
    }

    Ok(())
}

pub fn setup_scheduled_tasks(pool: DbPool) {
    let mut rss_timer = tokio::time::interval(Duration::from_secs(60 * 60));
    let mut details_timer = tokio::time::interval(Duration::from_secs(60 * 60 * 24));
    let mut claps_timer = tokio::time::interval(Duration::from_secs(60 * 15));

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = rss_timer.tick() => {
                    if let Err(e) = update_rss(&pool).await{
                        tracing::error!("Error fetching rss feed:\n{}" ,e.to_string());
                    }}
                _ = details_timer.tick() => {if let Err(e) = update_story_details(&pool).await{
                        tracing::error!("Error fetching submission details:\n{}" ,e.to_string());
                    }}
                _ = claps_timer.tick() => {if let Err(e) = update_claps(&pool).await{
                        tracing::error!("Error fetching number of claps:\n{}" ,e.to_string());
                    }}
            }
        }
    });
}
