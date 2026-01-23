
fn main() {
    if let Err(e) = csv_reader::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
