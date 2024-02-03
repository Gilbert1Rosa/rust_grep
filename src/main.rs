use std::env;
use std::process;
use rust_grep::main;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = main::Config::new(&args).unwrap_or_else(|err| {
        println!("A problem has ocurred: {err}");
        process::exit(1);
    });

    if let Err(e) = main::run(config) {
        println!("An error occurred: {e}");
        process::exit(1);
    }
}