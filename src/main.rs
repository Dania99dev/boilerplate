use boilerplate::{initialize, CommandType, Config};
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    match config.command {
        CommandType::New => initialize(config),
    };
}
