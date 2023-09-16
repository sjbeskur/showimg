use opencv as cv;
use cv::types::VectorOfu8;

use super::{Config, AppResult};

#[cfg(feature = "ocv")]
pub fn run_ocv(config: Config) -> AppResult<()>{        
    //dbg!(&config);
    match super::open(&config.file) {
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
