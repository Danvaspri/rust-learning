use rust_learning::Config;
use std::{env, process};
fn main() {
  



    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In filename {}", config.filename);

    if let Err(e) = rust_learning::run(config) {
        eprint!("Application error: {}", e);
        process::exit(1);
    }
}
