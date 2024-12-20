use dioxus::logger::tracing;
#[cfg(feature = "server")]
use {
    std::env,
 std::sync::Arc,
 dioxus::fullstack::ServeConfigBuilder,
 dioxus::prelude::DioxusRouterExt,
 dioxus_cli_config::fullstack_address_or_localhost,
 dotenvy::dotenv,
 medium_leaderboard::{init_db_connection, server, ContextProviders},
};

use medium_leaderboard::{App};

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    dioxus::logger::initialize_default();
    dotenv().ok();

    let connection_string = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match init_db_connection(&connection_string){
        Ok(pool) => pool,
        Err(err) => {tracing::error!("{}", err.to_string()); return}
    };

    server::setup_scheduled_tasks(pool.clone());

    let context_providers: ContextProviders =
        Arc::new(vec![Box::new(move || Box::new(pool.clone()))]);

    let router = axum::Router::new()
        .serve_dioxus_application(
            ServeConfigBuilder::default().context_providers(context_providers),
            App,
        );

    let addr = fullstack_address_or_localhost();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router.into_make_service()).await.unwrap();
}
#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}