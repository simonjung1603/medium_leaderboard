use crate::models::Submission;
use anyhow::anyhow;
use axum::http::{HeaderMap, HeaderValue};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{associations::HasTable, r2d2, Insertable, QueryDsl, RunQueryDsl, SqliteConnection};
use dioxus::logger::tracing;
use rss::Channel;
use serde::Deserialize;
use std::time::Duration;

async fn update_rss(pool: &r2d2::Pool<ConnectionManager<SqliteConnection>>) -> anyhow::Result<()> {
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
                if let Ok(_) = submissions.find(&guid).first::<Submission>(&mut connection) {
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
    pool: &r2d2::Pool<ConnectionManager<SqliteConnection>>,
) -> anyhow::Result<()> {
    tracing::info!("Updating all story details.");

    Ok(())
}

async fn fetch_story_details(id: &str) -> anyhow::Result<Submission> {
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
        return Ok(Submission {
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
    pool: &r2d2::Pool<ConnectionManager<SqliteConnection>>,
) -> anyhow::Result<()> {
    tracing::info!("Updating all clap counts");

    Ok(())
}

pub fn setup_scheduled_tasks(pool: Pool<ConnectionManager<SqliteConnection>>) {
    let mut rss_timer = tokio::time::interval(Duration::from_hours(1));
    let mut details_timer = tokio::time::interval(Duration::from_days(1));
    let mut claps_timer = tokio::time::interval(Duration::from_mins(10));

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
