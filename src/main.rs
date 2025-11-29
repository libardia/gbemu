mod gb;

use gb::GB;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::env;

fn main() {
    /* #region Set up logger */
    let args = env::args().collect::<Vec<_>>();
    if args.len() > 1 {
        env::set_var("RUST_LOG", &args[1]);
    }
    env::set_var("RUST_BACKTRACE", "1");

    SimpleLogger::new()
        .with_level(LevelFilter::Trace)
        .env()
        .init()
        .unwrap();
    /* #endregion */

    let mut gb = GB::new();
    gb.run();
}
