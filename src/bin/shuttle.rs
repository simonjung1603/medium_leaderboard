#[cfg(feature = "server")]
use {
    std::{
        path::Path,
        sync::Arc,
    },
    dioxus::{
        logger::tracing,
        prelude::{DioxusRouterExt, ServeConfigBuilder},
    },
    medium_leaderboard::{ContextProviders, init_db_connection, server},
    shuttle_runtime::Error,
};

use medium_leaderboard::App;

#[cfg(feature = "server")]
#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] connection_string: String) -> shuttle_axum::ShuttleAxum {
    dioxus::logger::initialize_default();

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
                    Ok(false) => tracing::info!("{:?} does not exists. Trying to create.", target_public_dir),
                    Err(err) => tracing::error!("Error checking if {:?} exists: {}", target_public_dir, err),
                    _ => tracing::error!("Unexpected error!")
                }

                /*
                if let Err(err) = fs::create_dir_all(&target_public_dir) {
                    tracing::error!("Error creating runtime/public dir: {}", err);
                } else {
                    tracing::info!("Created target public dir.")
                }
                 */

                match copy_dir::copy_dir("/app/public", target_public_dir) {
                    Ok(ve) => {
                        tracing::info!("Copied public dir to runtime.");
                        if ve.is_empty() == false {
                            tracing::error!("Errors occurred during copy operation:");
                            ve.into_iter().for_each(|e| tracing::error!("Error copying: {}", e))
                        }
                    }
                    Err(ve) => tracing::error!("Error copying: {}", ve)
                }
            }
        }
    }

    let pool = match init_db_connection(&connection_string) {
        Ok(pool) => pool,
        Err(err) => return Err(Error::Database(err.to_string()))
    };

    server::setup_scheduled_tasks(pool.clone());

    let context_providers: ContextProviders =
        Arc::new(vec![Box::new(move || Box::new(pool.clone()))]);

    let router = axum::Router::new()
        .serve_dioxus_application(
            ServeConfigBuilder::default().context_providers(context_providers),
            App,
        );

    Ok(router.into())
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App)
}