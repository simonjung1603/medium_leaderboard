#![feature(duration_constructors)]

use self::models::*;
use anyhow::anyhow;
use dioxus::dioxus_core::SpawnIfAsync;
use dioxus::logger::tracing;
use dioxus::prelude::server_fn::serde::Deserialize;
use dioxus::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Method;
use rss::Channel;
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use std::{env, fs};

#[cfg(feature = "server")]
use {
    diesel::associations::HasTable,
    diesel::r2d2,
    diesel::r2d2::{ConnectionManager, Pool},
    diesel::{prelude, Connection, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection, ExpressionMethods},
    diesel::{Insertable, NotFound},
    dotenvy::dotenv,
};

use components::{Card, Echo, Hero};

mod components;
mod models;
#[cfg(feature = "server")]
mod schema;
#[cfg(feature = "server")]
mod server;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const BORDER: Asset = asset!("/assets/border.png");

#[cfg(feature = "server")]
#[shuttle_runtime::main]
async fn main() {
    dioxus::logger::initialize_default();
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let manager = diesel::r2d2::ConnectionManager::<SqliteConnection>::new(db_url);
    let mut pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Error connecting to db.");

    server::setup_scheduled_tasks(pool.clone());

    let context_providers: Arc<Vec<Box<(dyn Fn() -> Box<(dyn std::any::Any)> + Send + Sync)>>> =
        Arc::new(vec![Box::new(move || Box::new(pool.clone()))]);

    let address = dioxus_cli_config::fullstack_address_or_localhost();
    let router = axum::Router::new()
        .serve_dioxus_application(
            ServeConfigBuilder::default().context_providers(context_providers),
            App,
        )
        .into_make_service();
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}

#[server(GetAllSubmissions)]
async fn get_all_submissions() -> Result<Vec<Submission>, ServerFnError> {
    use crate::schema::submissions::dsl::*;
    let FromContext::<Pool<ConnectionManager<SqliteConnection>>>(pool) = extract().await?;
    let mut connection = pool.get()?;
    let all_submissions = submissions
        .select(Submission::as_select()).order_by(clap_count.desc())
        .load(&mut connection)
        .expect("Error loading submissions.");

    dbg!(&all_submissions);

    Ok(all_submissions)
}

#[component]
fn App() -> Element {
    let submission_elements = use_resource(get_all_submissions);
    // Build cool things ✌️
    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: "https://cdnjs.cloudflare.com/ajax/libs/bulma/1.0.2/css/bulma.min.css" }
        script{src: "https://kit.fontawesome.com/98b204fec6.js", crossorigin:"anonymous"}

        /*
        <section class="hero is-primary">
  <div class="hero-body">
    <p class="title">Primary hero</p>
    <p class="subtitle">Primary subtitle</p>
  </div>
</section>
         */
        section{class:"hero has-background-primary-dark",
            div{class:"hero-body",
                p{class:"title", "Transformation"}
                p{class:"subtitle",
                    p{"A " em{"My Fair Lighthouse"} " writing contest"}
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
                            for (i, submission) in submission_elements.into_iter().enumerate(){
                                tr{
                                    th{
                                        {format!("{}.", i+1)}
                                    }
                                    td{
                                        {submission.title.clone()}
                                    }
                                    td{
                                        {format!("{} (@{})", submission.realname.clone(), submission.username.clone())}
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
    }
}

/*
div{class: "columns is-centered",
                div{class: "column is-half",
                    Card{}
                }
            }
            br{}
            br{}
            div{class: "columns is-centered is-8",
                div{class: "column is-one-third",
                    Card{}
                }
                div{class: "column is-one-third",
                    Card{}
                }
            }
            br{}
            br{}
            div{class: "grid is-col-min-12 is-gap-6",
                Card{}
                Card{}
                Card{}
                Card{}
                Card{}
                Card{}
                Card{}
            }
 */
