use std::env;
use std::process;
use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let config = match Config::parse(&args) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Usage: minigrep <QUERY_STRING> <FILEPATH>");
            process::exit(1);
        }
    };

    if let Err(e) = run(&config) {
        eprintln!("{e}");
        process::exit(2);
    };
}
