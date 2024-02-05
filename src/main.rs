use std::process;

fn main() {
    if let Err(e) = find_duplicate_files::run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}