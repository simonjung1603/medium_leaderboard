use std::sync::Arc;
use dioxus::prelude::{DioxusRouterExt, ServeConfigBuilder};
use medium_leaderboard::{App, init_db_connection, server};
use shuttle_runtime::Error;


#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] connection_string: String) -> shuttle_axum::ShuttleAxum {
    dioxus::logger::initialize_default();

    let pool = match init_db_connection(&connection_string){
        Ok(pool) => pool,
        Err(err) => return Err(Error::Database(err.to_string()))
    };

    server::setup_scheduled_tasks(pool.clone());

    let context_providers: Arc<Vec<Box<(dyn Fn() -> Box<(dyn std::any::Any)> + Send + Sync)>>> =
        Arc::new(vec![Box::new(move || Box::new(pool.clone()))]);

    let router = axum::Router::new()
        .serve_dioxus_application(
            ServeConfigBuilder::default().context_providers(context_providers),
            App,
        );

    Ok(router.into())
}