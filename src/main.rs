#![feature(plugin)]
#![plugin(rocket_codegen)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate slog;

extern crate config;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate regex;
extern crate serde;
extern crate slog_async;
extern crate slog_term;

use slog::{Drain, Logger};
use slog_async::Async;
use slog_term::{FullFormat, TermDecorator};

use routes::api;

mod db;
mod routes;
mod settings;

fn main() {
    let decorator = TermDecorator::new().build();
    let drain = Async::new(FullFormat::new(decorator).build().fuse())
        .build()
        .fuse();
    let logger = &Logger::root(drain, o!());

    let settings = settings::load(logger);
    let connection_pool = db::establish_pool(&settings)
        .expect("Failed to initialize database connection pool");

    rocket::ignite()
        .manage(connection_pool)
        .manage(settings)
        .mount("/api", routes![
            api::show
        ])
        .launch();
}
