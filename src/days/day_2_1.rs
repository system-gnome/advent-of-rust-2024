use std::fs::File;
use std::io::{self, BufRead};

pub fn run() {
    let file_path = "src/inputs/day_2.txt";

    match count_valid_lists(file_path) {
        Ok(valid_count) => println!(
            "[Day 2, Task 1] - The sum of valid lists is: {}",
            valid_count
        ),
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}

fn count_valid_lists(file_path: &str) -> io::Result<i32> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut valid_count = 0;

    for line in reader.lines() {
        let line = line?;
        if is_list_valid(&line) {
            valid_count += 1;
        }
    }

    Ok(valid_count)
}

fn is_list_valid(line: &str) -> bool {
    let mut numbers = line
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok());

    let mut previous = match numbers.next() {
        Some(num) => num,
        None => return false,
    };

    for number in numbers {
        let difference = number - previous;
        if difference.abs() < 1 || difference.abs() > 3 {
            return false;
        }
        previous = number;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_list_valid() {
        let sequence = "1 2 3 4 5";
        assert_eq!(is_list_valid(&sequence), true);

        let sequence = "9 7 5 4 2";
        assert_eq!(is_list_valid(&sequence), true);

        let sequence = "1 7 3 4 5";
        assert_eq!(is_list_valid(&sequence), false);
    }
}
