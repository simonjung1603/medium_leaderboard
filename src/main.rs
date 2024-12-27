use dioxus::logger::tracing;
use medium_leaderboard::components::app::App;
use std::env;

#[cfg(feature = "server")]
use {
    axum::Router,
    dioxus::prelude::{DioxusRouterExt, ServeConfigBuilder},
    dioxus_cli_config::fullstack_address_or_localhost,
    dotenvy::dotenv,
    medium_leaderboard::{db::*, server, ContextProviders},
};

#[cfg(feature = "server")]
fn standalone_setup(connection_string: &str) -> anyhow::Result<Router> {
    let pool = match init_db_connection(&connection_string) {
        Ok(pool) => pool,
        Err(err) => {
            tracing::error!("{}", err.to_string());
            return Err(err);
        }
    };

    server::setup_scheduled_tasks(pool.clone());

    let context_providers: ContextProviders =
        ContextProviders::new(vec![Box::new(move || Box::new(pool.clone()))]);

    Ok(Router::new().serve_dioxus_application(
        ServeConfigBuilder::default().context_providers(context_providers),
        App,
    ))
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::logger::initialize_default();
    tracing::info!("INIT WEB");
    dioxus::launch(App);
}

#[cfg(all(feature = "server", not(feature = "shuttle")))]
#[tokio::main]
async fn main() {
    dioxus::logger::initialize_default();

    tracing::info!("INIT STANDALONE");

    dotenv().ok();
    let connection_string = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let addr = fullstack_address_or_localhost();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        standalone_setup(&connection_string)
            .expect("Failed to create dioxus router")
            .into_make_service(),
    )
    .await
    .unwrap();
}

#[cfg(feature = "shuttle")]
#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] connection_string: String,
) -> shuttle_axum::ShuttleAxum {
    dioxus::logger::initialize_default();

    tracing::info!("INIT SHUTTLE");

    if let Ok(true) = Path::new("/app/public").try_exists() {
        tracing::info!("/app/public exists.");

        let target_public_dir = std::env::current_exe()
            .expect("Failed to get current executable path")
            .parent()
            .expect("Failed to get executable parent path")
            .join("public");

        match target_public_dir.try_exists() {
            Ok(true) => tracing::info!("{:?} already exists.", target_public_dir),
            other => {
                match other {
                    Ok(false) => {
                        tracing::info!("{:?} does not exists. Trying to create.", target_public_dir)
                    }
                    Err(err) => {
                        tracing::error!("Error checking if {:?} exists: {}", target_public_dir, err)
                    }
                    _ => tracing::error!("Unexpected error!"),
                }

                match copy_dir::copy_dir("/app/public", target_public_dir) {
                    Ok(ve) => {
                        tracing::info!("Copied public dir to runtime.");
                        if ve.is_empty() == false {
                            tracing::error!("Errors occurred during copy operation:");
                            ve.into_iter()
                                .for_each(|e| tracing::error!("Error copying: {}", e))
                        }
                    }
                    Err(ve) => tracing::error!("Error copying: {}", ve),
                }
            }
        }
    }

    Ok(standalone_setup(&connection_string)?.into())
}
