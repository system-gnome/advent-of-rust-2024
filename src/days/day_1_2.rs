use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn run() {
    let file_path = "src/inputs/day_1.txt";

    let (list1, list2) = read_input(file_path).expect("Failed to read input file");

    let result: i32 = similarity_score(&list1, &list2);

    println!("[Day 1, Task 2] - The similarity score is: {}", result);
}

fn similarity_score(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let counts = count_elements(list2);

    return list1
        .iter()
        .filter_map(|&key| counts.get(&key).map(|&count| key * count))
        .sum();
}

fn count_elements(vec: &Vec<i32>) -> HashMap<i32, i32> {
    let mut counts = HashMap::new();

    for &value in vec {
        *counts.entry(value).or_insert(0) += 1;
    }

    counts
}

fn read_input(file_path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().expect("Invalid number in input file"))
            .collect();

        if nums.len() == 2 {
            list1.push(nums[0]);
            list2.push(nums[1]);
        } else {
            panic!("Invalid line format: {}", line);
        }
    }

    Ok((list1, list2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similarity_score() {
        let list1 = vec![3, 4, 2, 1, 3, 3];
        let list2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(similarity_score(&list1, &list2), 31);
    }
}
