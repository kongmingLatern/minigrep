use minigrep::Config;
use std::env;
use std::process;
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application e: {}", e);
        process::exit(1);
    }
}