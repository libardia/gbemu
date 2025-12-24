#[allow(dead_code)]
mod gb;

use gb::GB;
use log::LevelFilter;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .env()
        .init()
        .unwrap();

    let mut gb = GB::new();
    gb.run();
}
