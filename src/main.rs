
#[show_image::main]
fn main() {
    if let Err(e) = showimg::get_args().and_then(showimg::run){
        eprintln!("{}",e);
        std::process::exit(1);
    }
}
