use clap::{Arg, Command};
use std::error::Error;
use std::io::{self, BufReader, BufRead};
use std::fs::File;

//use opencv::highgui::*;
//use opencv::prelude::*;
//use opencv::imgcodecs::*;

#[derive(Debug)]

pub struct Config{
    file: String,
}

type AppResult<T> = Result<T, Box<dyn Error>>;


pub fn run(config: Config) -> AppResult<()>{
        
    dbg!(&config);

    // match open(&config.file) {
    //     Err(err) => eprintln!("{}", err),
    //     Ok(file) =>{
    //         const BUF_LEN : usize = 4096;
    //         let mut buffer: [u8;BUF_LEN] = [0; BUF_LEN];
    //         let mut read_count = file.read(&mut buffer)?;

    //         opencv::imgcodecs::imdecode(&VectorOfuchar::from_iter(buffer), opencv::imgcodecs::IMREAD_GRAYSCALE);

    //     },
    // }

    let result = opencv::imgcodecs::imread(&config.file, opencv::imgcodecs::IMREAD_COLOR);// IMREAD_GRAYSCALE);
    opencv::highgui::imshow(&config.file, &result?)?;
    let _key = opencv::highgui::wait_key(0)?;

    Ok(())
}


pub fn get_args() -> AppResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
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


pub fn open(filename: &str) -> AppResult<Box<dyn BufRead>> {
    match filename{
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}