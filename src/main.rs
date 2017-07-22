#![feature(plugin)]
#![plugin(rocket_codegen)]


#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate slog;

extern crate config;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate regex;
extern crate serde;
extern crate slog_async;
extern crate slog_term;

use slog::{Drain, Logger};
use slog_async::Async;
use slog_term::{FullFormat, TermDecorator};

mod db;
mod routes;
mod settings;

fn main() {
    let decorator = TermDecorator::new().build();
    let drain = Async::new(FullFormat::new(decorator).build().fuse())
        .build()
        .fuse();
    let logger = &Logger::root(drain, o!());

    let settings = &settings::load(logger);
    let connection_pool = db::establish_pool(settings).unwrap();

    rocket::ignite()
        .manage(connection_pool)
        .mount("/api", routes::api::get())
        .launch();
}
