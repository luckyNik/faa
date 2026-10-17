use std::io;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    if let Err(e) = faa::cli::run(stdin.lock(), stdout.lock()) {
        eprintln!("I/O error: {e}");
        std::process::exit(1);
    }
}
