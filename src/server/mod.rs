use std::ops::Sub;
use crate::models::{InsertClapHistory, InsertSubmission, Submission};
use anyhow::anyhow;
use axum::http::{HeaderMap, HeaderValue};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{associations::HasTable, r2d2, Insertable, QueryDsl, RunQueryDsl, pg::PgConnection, SelectableHelper, ExpressionMethods, QueryResult};
use dioxus::logger::tracing;
use rss::Channel;
use serde::Deserialize;
use std::time::Duration;
use chrono::TimeDelta;
use dioxus::prelude::ServerFnError;
use crate::{get_all_submissions, schema};
use crate::schema::clap_history::dsl::clap_history;
use crate::schema::submissions::clap_count_last_updated_at;
use crate::schema::submissions::dsl::{submissions as submissions_table};

async fn update_rss(pool: &r2d2::Pool<ConnectionManager<PgConnection>>) -> anyhow::Result<()> {
    tracing::info!("Fetching rss feed.");

    use crate::schema::submissions::dsl::submissions;

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
                if submissions.find(&guid).first::<Submission>(&mut connection).is_ok() {
                    tracing::info!("Submission for guid {} already present in db.", guid);
                    continue;
                }
                let new_submission = fetch_story_details(&guid).await?;

                let rows_affected = new_submission
                    .insert_into(submissions::table())
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
    _pool: &r2d2::Pool<ConnectionManager<PgConnection>>,
) -> anyhow::Result<()> {
    tracing::info!("Updating all story details.");

    Ok(())
}

#[allow(non_snake_case, unused)]
async fn fetch_story_details(id: &str) -> anyhow::Result<InsertSubmission> {
    tracing::info!("Fetching details for guid {}.", id);

    const POST_PAGE_QUERY: &str = r#"query PostPageQuery($postId: ID!) {postResult(id: $postId) {__typename\n ... on Post {id\n creator {id\n name\n username\n __typename}\n mediumUrl\n latestPublishedVersion\n latestPublishedAt\n clapCount\n title\n previewImage{id\n __typename}\n tags{\n id\n __typename}\n wordCount\n __typename}}}"#;

    let body = format!(
        r#"[{{"operationName": "PostPageQuery", "query": "{}", "variables": {{"postId": "{}"}}}}]"#,
        POST_PAGE_QUERY, id
    );

    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("application/json"));

    #[derive(Deserialize, Debug)]
    struct GraphQlResponse<T> {
        data: T,
    }
    #[derive(Deserialize, Debug)]
    struct PostPageQueryResponse {
        postResult: PostResponse,
    }
    #[derive(Deserialize, Debug)]
    struct PostResponse {
        id: String,
        creator: CreatorResponse,
        mediumUrl: String,
        latestPublishedVersion: String,
        latestPublishedAt: i64,
        clapCount: i32,
        title: String,
        previewImage: PreviewImageResponse,
        wordCount: i32,
    }
    #[derive(Deserialize, Debug)]
    struct CreatorResponse {
        id: String,
        name: String,
        username: String,
    }
    #[derive(Deserialize, Debug)]
    struct PreviewImageResponse {
        id: String,
    }

    let response = reqwest::Client::new()
        .post("https://medium.com/_/graphql")
        .body(body)
        .headers(headers)
        .send()
        .await?
        .json::<Vec<GraphQlResponse<PostPageQueryResponse>>>()
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
            .postResult;
        return Ok(InsertSubmission {
            guid: r.id,
            realname: r.creator.name,
            username: r.creator.username,
            latest_published_version: r.latestPublishedVersion,
            latest_published_at: r.latestPublishedAt,
            clap_count: r.clapCount,
            title: r.title,
            img_id: r.previewImage.id,
            word_count: r.wordCount,
        });
    }

    Err(anyhow!("Error fetching response: {:?}", response))
}

async fn update_claps(
    pool: &r2d2::Pool<ConnectionManager<PgConnection>>,
) -> anyhow::Result<()> {
    let mut connection = pool.get()?;

    match submissions_table.select(clap_count_last_updated_at).order_by(clap_count_last_updated_at.desc()).first::<chrono::DateTime<chrono::Local>>(&mut connection) {
        Ok(dateTime) => if chrono::Local::now().signed_duration_since(dateTime) < TimeDelta::minutes(15) {
            tracing::info!("Checked within last 15 minutes, not checking again!");
            return Ok(());
        }
        Err(err) => tracing::error!("Error fetching last update time: {}", err)
    }
    tracing::info!("Updating all clap counts");

    const CLAP_COUNT_QUERY: &str = r#"query ClapCountQuery($postId: ID!, $includeFirstBoostedAt: Boolean!) {\n  postResult(id: $postId) {\n    __typename\n    ... on Post {\n      id\n      clapCount\n      firstBoostedAt @include(if: $includeFirstBoostedAt)\n      __typename\n    }\n  }\n}\n"#;

    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("application/json"));

    #[derive(Deserialize, Debug)]
    struct GraphQlResponse<T> {
        data: T,
    }
    #[derive(Deserialize, Debug)]
    struct PostPageQueryResponse {
        postResult: PostResponse,
    }
    #[derive(Deserialize, Debug)]
    struct PostResponse {
        clapCount: i32,
    }

    let submissions = submissions_table
        .select(Submission::as_select())
        .load(&mut connection)
        .expect("Error loading submissions.");

    let mut client = reqwest::Client::new();

    for submission in submissions {
        let body = format!(
            r#"[{{"operationName":"ClapCountQuery","variables":{{"postId":"{}","includeFirstBoostedAt":false}},"query":"{}"}}]"#,
            submission.guid, CLAP_COUNT_QUERY);

        let clap_count = client
            .post("https://medium.com/_/graphql")
            .body(body)
            .headers(headers.clone())
            .send()
            .await?.json::<Vec<GraphQlResponse<PostPageQueryResponse>>>().await?.into_iter().next()
            .ok_or(anyhow!("Unexpected error reading clap_count graphql response"))?.data.postResult.clapCount;

        tracing::info!("{}: {}", submission.guid, clap_count);

        if clap_count != submission.clap_count {
            tracing::info!("{}: {} --> {}", submission.title, submission.clap_count, clap_count);

            let affected_rows = InsertClapHistory {
                guid: submission.guid.clone(),
                clap_count,
            }.insert_into(clap_history).execute(&mut connection);

            if let Ok(1) = affected_rows {
                tracing::info!("Inserted into history.");
            } else {
                tracing::warn!("Inserting clap_count into history failed.")
            }

            if let Ok(1) = diesel::update(&submission)
                .set(schema::submissions::clap_count.eq(clap_count))
                .execute(&mut connection) {
                tracing::info!("Updated entry in submissions.");
            } else {
                tracing::error!("Update in submissions failed or affected multiple rows!");
            }
        }
    }

    Ok(())
}

pub fn setup_scheduled_tasks(pool: Pool<ConnectionManager<PgConnection>>) {
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
