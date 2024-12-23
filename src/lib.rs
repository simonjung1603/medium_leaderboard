use chrono::Local;
use chrono::DateTime;
use dioxus::{
    prelude::*,
    document,
};
use self::models::*;
use anyhow::anyhow;
use std::{
    fmt::Debug,
    sync::Arc,
    env,
};
use web_sys::js_sys;

#[cfg(feature = "server")]
use {
    std::ops::Add,
    chrono::TimeDelta,
    diesel::data_types::PgTimestamp,
    diesel::QueryResult,
    diesel::sql_types::Timestamptz,
    diesel::{
        r2d2,
        r2d2::{ConnectionManager, Pool},
        QueryDsl,
        RunQueryDsl,
        SelectableHelper,
        pg::PgConnection,
        ExpressionMethods,
    },
    dioxus::prelude::{
        extract,
        FromContext,
    },
    diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness},
};

pub type ContextProviders = Arc<Vec<Box<(dyn Fn() -> Box<(dyn std::any::Any)> + Send + Sync)>>>;

mod components;
mod models;
#[cfg(feature = "server")]
mod schema;
#[cfg(feature = "server")]
pub mod server;

const FAVICON: Asset = asset!("/assets/favicon.ico");

#[cfg(feature = "server")]
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[cfg(feature = "server")]
pub fn init_db_connection(connection_string: &str) -> anyhow::Result<r2d2::Pool<ConnectionManager<PgConnection>>> {
    let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(connection_string);
    let pool = r2d2::Pool::builder().build(manager)?;
    if let Err(err) = pool.get()?.run_pending_migrations(MIGRATIONS) {
        return Err(anyhow!("Error running migrations: {}", err.to_string()));
    }
    Ok(pool)
}

#[server(GetAllSubmissions)]
async fn get_all_submissions() -> Result<Vec<Submission>, ServerFnError> {
    use crate::schema::submissions::dsl::*;
    let FromContext::<Pool<ConnectionManager<PgConnection>>>(pool) = extract().await?;
    let mut connection = pool.get()?;
    let all_submissions = submissions
        .select(Submission::as_select()).order_by(clap_count.desc())
        .load(&mut connection)
        .expect("Error loading submissions.");

    Ok(all_submissions)
}

#[server(GetLatestUpdateTime)]
async fn get_latest_and_next_update_time() -> Result<(DateTime<Local>, DateTime<Local>), ServerFnError> {
    use crate::schema::submissions::dsl::*;
    let FromContext::<Pool<ConnectionManager<PgConnection>>>(pool) = extract().await?;
    let mut connection = pool.get()?;

    let latest_update_time = match submissions.select(clap_count_last_updated_at).order_by(clap_count_last_updated_at.desc()).first::<chrono::DateTime<chrono::Local>>(&mut connection) {
        Ok(db_time) => chrono::DateTime::from(db_time),
        Err(_) => Local::now()
    };

    Ok((latest_update_time, latest_update_time.add(TimeDelta::minutes(15))))
}

#[component]
pub fn App() -> Element {
    let submission_elements = use_resource(get_all_submissions);
    let latest_update_time = use_resource(get_latest_and_next_update_time);
    let time_fmt = "%H:%M";
    let (latest, next) = match &*latest_update_time.read_unchecked() {
        None => ("...".to_string(), "...".to_string()),
        Some(Ok((latest, next))) => (latest.format(time_fmt).to_string(), next.format(time_fmt).to_string()),
        Some(Err(err)) => ("---".to_string(), "---".to_string()),
    };

    use_memo(move || {
        if let Some(Ok(subs)) = &*submission_elements.read_unchecked() {
            let titles = subs.iter().map(|sub| if sub.title.chars().count() > 15 { format!("'{:.12}...'", sub.title) } else { format!("'{}'", sub.title) }).collect::<Vec<_>>().join(", ");
            let counts = subs.iter().map(|sub| sub.clap_count.to_string()).collect::<Vec<_>>().join(", ");
            js_sys::eval(&format!(r#"
                const ctx = document.getElementById('my-chart').getContext('2d');
                new Chart(ctx, {{
                    type: 'bar',
                    data: {{
                        labels: [{}],
                        datasets: [{{
                            label: 'Amount of claps',
                            data: [{}],
                            backgroundColor: [
                              'rgba(255, 99, 132, 0.2)',
                              'rgba(255, 159, 64, 0.2)',
                              'rgba(255, 205, 86, 0.2)',
                              'rgba(75, 192, 192, 0.2)',
                              'rgba(54, 162, 235, 0.2)',
                              'rgba(153, 102, 255, 0.2)',
                              'rgba(201, 203, 207, 0.2)'
                            ],
                            borderColor: [
                              'rgb(255, 99, 132)',
                              'rgb(255, 159, 64)',
                              'rgb(255, 205, 86)',
                              'rgb(75, 192, 192)',
                              'rgb(54, 162, 235)',
                              'rgb(153, 102, 255)',
                              'rgb(201, 203, 207)'
                            ],
                        }}]
                    }},
                    options: {{
                        plugins: {{
                            legend: {{
                                labels: {{
                                    boxWidth: 0
                                }}
                            }}
                        }},
                        responsive: true,
                        scales: {{
                            y: {{ beginAtZero: true }}
                        }},
                    }}
                }});
            "#, titles, counts))
                .expect("Failed to execute JavaScript");
        }
    });

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: "https://cdnjs.cloudflare.com/ajax/libs/bulma/1.0.2/css/bulma.min.css" }
        script{src: "https://kit.fontawesome.com/98b204fec6.js", crossorigin:"anonymous"}
        script{src: "https://cdnjs.cloudflare.com/ajax/libs/Chart.js/4.4.1/chart.umd.js"}

        section{class:"hero has-background-secondary",
            div{class:"hero-body",
                div{class: "columns is-vcentered",
                    div{class: "column",
                        p{class:"title", "Transformation"}
                        p{class:"subtitle",
                            p{"A " em{"My Fair Lighthouse"} " writing contest"}
                        }
                    }
                    div{class: "column is-two-fifth is-pulled-right is-flex is-justify-content-end",
                        table{class: "table has-text-weight-light has-background-secondary is-size-7 is-bordered is-narrow",
                            tr{
                                td{"Version"}
                                td{"0.0.1"}
                            }
                            tr{
                                td{"Claps last updated"}
                                td{{latest}}
                            }
                            tr{
                                td{"Next scheduled update"}
                                td{{next}}
                            }
                        }
                    }
                }
            }
        }

        div{class: "container",
            div{class: "columns is-centered",
                div{class: "column is-two-thirds",
                div{class: "title mt-6", "Community vote live standings"}
                if let Some(Ok(submission_elements)) = &*submission_elements.read_unchecked(){
                    table{class: "table mt-6 is-bordered is-striped is-hoverable is-fullwidth",
                        thead{
                                tr{
                                    th{"Rank"}
                                    th{"Title"}
                                    th{"Author"}
                                    th{"Claps " i{class: "fa-solid fa-arrow-down"}}
                                    th{"Word count"}
                                }
                            }
                        tbody{
                            for (i, submission) in submission_elements.iter().enumerate(){
                                tr{
                                    th{
                                        {format!("{}.", i+1)}
                                    }
                                    td{
                                        {submission.title.clone()}
                                    }
                                    td{
                                        a{
                                            href: {format!("https://medium.com/@{}", submission.username.clone())},
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            {format!("@{}", submission.username.clone())}
                                        }
                                    }
                                    td{
                                        {submission.clap_count.to_string()}
                                    }
                                    td{
                                        {submission.word_count.to_string()}
                                    }
                                }
                            }
                        }
                    }
                }
                }
            }
        }

        div{class: "container is-max-tablet box mt-6",
            div {
                id: "chart-container",
                canvas {
                    id: "my-chart",
                }
            }
        }
    }
}
