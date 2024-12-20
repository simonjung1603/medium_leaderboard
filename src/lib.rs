use dioxus::document;
use dioxus::prelude::fc_to_builder;
use dioxus::prelude::IntoDynNode;
use dioxus::prelude::Readable;
use dioxus::prelude::GlobalSignal;
use dioxus::prelude::use_resource;
use dioxus::prelude::Element;
use dioxus::prelude::{ServerFnError};
use dioxus::prelude::manganis;
use dioxus::prelude::Asset;
use dioxus::prelude::server;
use dioxus::prelude::server_fn;
use dioxus::prelude::{asset, rsx};
use dioxus::prelude::component;
use self::models::*;
use anyhow::anyhow;
use dioxus::dioxus_core::SpawnIfAsync;
use dioxus::logger::tracing;
use dioxus::prelude::server_fn::serde::Deserialize;
use dioxus::prelude::dioxus_core;
use dioxus::prelude::dioxus_elements;
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
    diesel::{prelude, Connection, QueryDsl, RunQueryDsl, SelectableHelper, pg::PgConnection, ExpressionMethods},
    dioxus::prelude::{
        extract,
        FromContext,
        DioxusRouterExt,
        ServeConfigBuilder
    },
    diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness},
    diesel::{Insertable, NotFound},
    dotenvy::dotenv,
};

pub type ContextProviders = Arc<Vec<Box<(dyn Fn() -> Box<(dyn std::any::Any)> + Send + Sync)>>>;

mod components;
mod models;
#[cfg(feature = "server")]
mod schema;
#[cfg(feature = "server")]
pub mod server;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const BORDER: Asset = asset!("/assets/border.png");

#[cfg(feature = "server")]
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
#[cfg(feature = "server")]
pub fn init_db_connection(connection_string: &str) -> anyhow::Result<r2d2::Pool<ConnectionManager<PgConnection>>>{
    let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(connection_string);
    let mut pool = r2d2::Pool::builder().build(manager)?;
    if let Err(err) = pool.get()?.run_pending_migrations(MIGRATIONS){
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

    dbg!(&all_submissions);

    Ok(all_submissions)
}

#[component]
pub fn App() -> Element {
    let submission_elements = use_resource(get_all_submissions);
    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: "https://cdnjs.cloudflare.com/ajax/libs/bulma/1.0.2/css/bulma.min.css" }
        script{src: "https://kit.fontawesome.com/98b204fec6.js", crossorigin:"anonymous"}

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
