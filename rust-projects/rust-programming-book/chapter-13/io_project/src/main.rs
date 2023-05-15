use std::env;
use std::process;

use io_project::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --snip--

    if let Err(e) = io_project::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}