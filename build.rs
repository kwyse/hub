extern crate diesel;

use std::env;
use diesel::{migrations, Connection};
use diesel::pg::PgConnection;

fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url)
        .expect("Failed to establish connection to database");

    migrations::run_pending_migrations(&conn).ok();
}
