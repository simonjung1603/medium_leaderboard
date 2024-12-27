use anyhow::anyhow;

use diesel::r2d2::*;
use diesel::{r2d2, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub type DbPool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

#[cfg(feature = "server")]
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[cfg(feature = "server")]
pub fn init_db_connection(
    connection_string: &str,
) -> anyhow::Result<r2d2::Pool<ConnectionManager<PgConnection>>> {
    let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(connection_string);
    let pool = r2d2::Pool::builder().build(manager)?;
    if let Err(err) = pool.get()?.run_pending_migrations(MIGRATIONS) {
        return Err(anyhow!("Error running migrations: {}", err.to_string()));
    }
    Ok(pool)
}
