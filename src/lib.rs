use std::error::Error;
use std::io::{self, BufReader, BufRead};
use std::fs::File;
use clap::{Arg, Command};

use opencv as cv;
use cv::types::VectorOfu8;

#[derive(Debug)]
pub struct Config{
    file: String,
    // todo: add more options (color resize?)
}

type AppResult<T> = Result<T, Box<dyn Error>>;


pub fn run(config: Config) -> AppResult<()>{
        
    //dbg!(&config);
    match open(&config.file) {
        Err(err) => eprintln!("{}", err),

        Ok(mut file) =>{
            let mut buffer : Vec<u8> = Vec::new();
            let _read_count = file.read_to_end(&mut buffer)?;
            let result = cv::imgcodecs::imdecode(&VectorOfu8::from_iter(buffer), cv::imgcodecs::IMREAD_COLOR); // IMREAD_GRAYSCALE);
            
            cv::highgui::named_window(&config.file, cv::highgui::WINDOW_KEEPRATIO)?;
            cv::highgui::imshow(&config.file, &result?)?;
            let _key = cv::highgui::wait_key(0)?;
            cv::highgui::destroy_all_windows()?;
        },
    }

    Ok(())
}


pub fn get_args() -> AppResult<Config> {
    let matches = Command::new("showimg")
        .version("0.1.2")
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