mod days;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: advent_of_code <day>");
        std::process::exit(1);
    }

    let day = &args[1];
    match day.as_str() {
        "1-1" => days::day_1_1::run(),
        "1-2" => days::day_1_2::run(),
        "2-1" => days::day_2_1::run(),
        "2-2" => days::day_2_2::run(),
        _ => eprintln!("Day {} not implemented!", day),
    }
}
