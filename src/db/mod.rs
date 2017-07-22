use settings::Settings;

use diesel::pg::PgConnection;
use r2d2::{Config, InitializationError, Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use rocket::{Request, State, Outcome};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome as RequestOutcome};
use std::ops::Deref;

pub mod models;
pub mod schema;

type PoolConnectionResult = Result<Pool<ConnectionManager<PgConnection>>, InitializationError>;

pub fn establish_pool(settings: &Settings) -> PoolConnectionResult {
    let config = Config::builder()
        .pool_size(settings.database.database_pool_size)
        .build();

    let manager = ConnectionManager::<PgConnection>::new(settings.database.database_url.clone());
    Pool::new(config, manager)
}

pub struct DatabaseConnection(pub PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DatabaseConnection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> RequestOutcome<DatabaseConnection, ()> {
        let pool = request.guard::<State<Pool<_>>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DatabaseConnection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for DatabaseConnection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
