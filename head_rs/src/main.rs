fn main() {
    // get args and run the run function
    if let Err(e) = head_rs::get_args().and_then(head_rs::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
