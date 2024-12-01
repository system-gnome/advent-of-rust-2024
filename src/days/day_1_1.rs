use std::fs::File;
use std::io::{self, BufRead};

pub fn run() {
    let file_path = "src/inputs/day_1.txt";

    let (list1, list2) = read_input(file_path).expect("Failed to read input file");

    let result = sum_of_differences(&list1, &list2);

    println!("[Day 1, Task 1] - The sum of differences is: {}", result);
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

fn sum_of_differences(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    if list1.len() != list2.len() {
        panic!("Lists must have the same length");
    }

    let mut sorted_list1 = list1.clone();
    let mut sorted_list2 = list2.clone();

    sorted_list1.sort();
    sorted_list2.sort();

    sorted_list1
        .iter()
        .zip(sorted_list2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_differences() {
        let list1 = vec![3, 4, 2, 1, 3, 3];
        let list2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(sum_of_differences(&list1, &list2), 11);

        let list1 = vec![1, 2, 3];
        let list2 = vec![4, 5, 6];
        assert_eq!(sum_of_differences(&list1, &list2), 9);

        let list1 = vec![10, 20, 30];
        let list2 = vec![15, 25, 35];
        assert_eq!(sum_of_differences(&list1, &list2), 15);
    }
}
