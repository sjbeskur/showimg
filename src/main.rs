fn main() {
    if let Err(e) = imshow::get_args().and_then(imshow::run){
        eprintln!("{}",e);
        std::process::exit(1);
    }
}
