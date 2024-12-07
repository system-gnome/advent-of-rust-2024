mod days;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <day>. Example: cargo run 1-1");
        std::process::exit(1);
    }

    let day = &args[1];
    match day.as_str() {
        "1-1" => days::day_1_1::run(),
        "1-2" => days::day_1_2::run(),
        "2-1" => days::day_2_1::run(),
        "2-2" => days::day_2_2::run(),
        "3-1" => days::day_3_1::run(),
        "3-2" => days::day_3_2::run(),
        "4-1" => days::day_4_1::run(),
        "4-2" => days::day_4_2::run(),
        "5-1" => days::day_5_1::run(),
        "5-2" => days::day_5_2::run(),
        "6-1" => days::day_6_1::run(),
        "6-2" => days::day_6_2::run(),
        "7-1" => days::day_7_1::run(),
        "7-2" => days::day_7_2::run(),
        _ => eprintln!("Day {} not implemented!", day),
    }
}
