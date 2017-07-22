use settings::Settings;

use diesel::pg::PgConnection;
use r2d2::{Config, InitializationError, Pool};
use r2d2_diesel::ConnectionManager;

pub mod models;
pub mod schema;

type PoolConnectionResult = Result<Pool<ConnectionManager<PgConnection>>, InitializationError>;

pub fn establish_pool(settings: &Settings) -> PoolConnectionResult {
    let config = Config::builder()
        .pool_size(settings.database.pool_size)
        .build();

    let manager = ConnectionManager::<PgConnection>::new(settings.database.url.clone());
    Pool::new(config, manager)
}
