use std::fs::File;
use std::io::{self, BufRead};

pub fn run() {
    let file_path = "src/inputs/day_2.txt";

    match calculate_safe_count(file_path) {
        Ok(safe_count) => println!(
            "[Day 2, Task 2] - The sum of safe lists with issue dampening is: {}",
            safe_count
        ),
        Err(error) => eprintln!("Error reading file: {}", error),
    }
}

fn calculate_safe_count(file_path: &str) -> io::Result<i32> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut safe_count = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        if is_valid_with_removal(&numbers) {
            safe_count += 1;
        }
    }

    Ok(safe_count)
}

fn is_valid_with_removal(numbers: &[i32]) -> bool {
    if is_valid(numbers) {
        return true;
    }

    for index in 0..numbers.len() {
        let mut modified_numbers = numbers.to_vec();
        modified_numbers.remove(index);
        if is_valid(&modified_numbers) {
            return true;
        }
    }

    false
}

fn is_valid(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return false;
    }

    let is_increasing = numbers[1] - numbers[0] > 0;
    let mut previous = numbers[0];

    for &current in &numbers[1..] {
        let difference = current - previous;

        if difference.abs() < 1 || difference.abs() > 3 {
            return false;
        }

        if is_increasing && difference < 0 || !is_increasing && difference > 0 {
            return false;
        }

        previous = current;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid(&[1, 2, 3, 4, 5]), true);
        assert_eq!(is_valid(&[9, 7, 5, 4, 2]), true);
        assert_eq!(is_valid(&[1, 7, 3, 4, 5]), false);
    }

    #[test]
    fn test_is_valid_with_removal() {
        assert_eq!(is_valid_with_removal(&[7, 6, 4, 2, 1]), true);
        assert_eq!(is_valid_with_removal(&[1, 2, 7, 8, 9]), false);
        assert_eq!(is_valid_with_removal(&[9, 7, 6, 2, 1]), false);
        assert_eq!(is_valid_with_removal(&[1, 3, 2, 4, 5]), true);
        assert_eq!(is_valid_with_removal(&[8, 6, 4, 4, 1]), true);
        assert_eq!(is_valid_with_removal(&[1, 3, 6, 7, 9]), true);
    }
}
