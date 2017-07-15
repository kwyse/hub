#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate slog;

extern crate config;
extern crate regex;
extern crate serde;
extern crate slog_async;
extern crate slog_term;

use slog::{Drain, Logger};
use slog_async::Async;
use slog_term::{FullFormat, TermDecorator};

mod settings;

fn main() {
    let decorator = TermDecorator::new().build();
    let drain = Async::new(FullFormat::new(decorator).build().fuse())
        .build()
        .fuse();
    let logger = &Logger::root(drain, o!());

    let _ = settings::load(logger);
}
