use minigrep::Config;
use owo_colors::colors;
use owo_colors::colors::*;
use owo_colors::OwoColorize;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem Parsing the arguemt: {}", err);
        process::exit(1);
    });

    println!(
        "Searching for {}",
        config.query.fg::<Green>().bg::<colors::Default>()
    );
    println!(
        "In File {}",
        config.filename.fg::<Green>().bg::<colors::Default>()
    );

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
