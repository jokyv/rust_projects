use std::process::exit;

fn main() {
    if let Err(e) = cat_rs::get_args().and_then(cat_rs::run) {
        eprintln!("{}", e);
        exit(1);
    }
}
