
use clap::{Arg, Command};
use super::Config;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn get_args() -> super::AppResult<Config> {
    let matches = Command::new("showimg")
        .version(VERSION)
        .author("Sam Beskur <sam.beskur@gmail.com>")
        .about("A simple image viewer program.")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input file(s) to read")
                .num_args(1)
                .default_value("-"),
        ).get_matches();

    Ok(Config {
        file: matches
            .get_one::<String>("file").unwrap().to_owned()
    })
}
