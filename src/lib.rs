mod cli;
pub use cli::*;

#[cfg(feature = "ocv")]
pub mod ocv;

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use image::io::Reader as ImageReader;
use image::*;

use show_image::{create_window, event, AsImageView, ImageInfo, ImageView};
use std::io::Cursor;

#[derive(Debug)]
pub struct Config {
    file: String,
    // todo: add more options (color resize?)
}

type AppResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> AppResult<()> {
    #[cfg(feature = "ocv")]
    {
        return ocv::run_ocv(config);
    }
    #[cfg(feature = "default")]
    {
        return run_default(config);
    }
}

pub fn run_default(config: Config) -> AppResult<()> {
    match open(&config.file) {
        Err(err) => eprintln!("{}", err),

        Ok(mut file) => {
            let mut buffer: Vec<u8> = Vec::new();
            let _read_count = file.read_to_end(&mut buffer)?;

            let image = ImageReader::new(Cursor::new(buffer)).with_guessed_format()?.decode()?;
            println!("made it here");
            //let result = cv::imgcodecs::imdecode(&VectorOfu8::from_iter(buffer), cv::imgcodecs::IMREAD_COLOR); // IMREAD_GRAYSCALE);
            //cv::highgui::named_window(&config.file, cv::highgui::WINDOW_KEEPRATIO)?;
            //cv::highgui::imshow(&config.file, &result?)?;
            //let _key = cv::highgui::wait_key(0)?;
            //cv::highgui::destroy_all_windows()?;


            //            let image = ImageView::new(info, &img2);

            // Create a window with default options and display the image.
            let window = create_window("image", Default::default())?;
            window.set_image("image-001", image)?;

            // Print keyboard events until Escape is pressed, then exit.
            // If the user closes the window, the channel is closed and the loop also exits.
            for event in window.event_channel()? {
                if let event::WindowEvent::KeyboardInput(event) = event {
                    println!("{:#?}", event);
                    if event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                        break;
                    }
                }
            }            

        }
    }

    Ok(())
}

pub fn open(filename: &str) -> AppResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
