use std::collections::HashMap;

pub fn run() {
    let file_path = "src/inputs/day_11.txt";
    let input_content = std::fs::read_to_string(file_path).expect("Failed to read input file");

    let stone_count = get_stone_count(25, &input_content);
    println!(
        "[Day 11, Task 1] Stone count after 25 blinks: {}",
        stone_count
    );

    let stone_count = get_stone_count(75, &input_content);
    println!(
        "[Day 11, Task 2] Stone count after 75 blinks: {}",
        stone_count
    );
}

fn get_stone_count(blink_count: usize, input: &str) -> u64 {
    let mut stones_map: HashMap<String, u64> = HashMap::new();
    for stone in input.split_whitespace() {
        *stones_map.entry(stone.to_string()).or_insert(0) += 1;
    }

    for _ in 0..blink_count {
        stones_map = blink_all(&stones_map);
    }

    stones_map.values().sum()
}

fn blink_all(stones: &HashMap<String, u64>) -> HashMap<String, u64> {
    let mut new_stones = HashMap::with_capacity(stones.len() * 2);
    for (stone, &count) in stones {
        let transformed_stones = blink_stone(stone);
        for transformed in transformed_stones {
            *new_stones.entry(transformed).or_insert(0) += count;
        }
    }
    new_stones
}

fn blink_stone(stone: &str) -> Vec<String> {
    if stone == "0" {
        return vec!["1".to_string()];
    }

    let length = stone.len();
    if length % 2 == 0 {
        let half = length / 2;
        let left_half = remove_leading_zeros(&stone[..half]);
        let right_half = remove_leading_zeros(&stone[half..]);
        vec![left_half, right_half]
    } else {
        vec![multiply_by_2024(stone)]
    }
}

fn remove_leading_zeros(s: &str) -> String {
    let trimmed = s.trim_start_matches('0');
    if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

fn multiply_by_2024(num_str: &str) -> String {
    let digits: Vec<u8> = num_str.bytes().rev().map(|b| b - b'0').collect();

    let mul_single = |n: &Vec<u8>, factor: u8| -> Vec<u8> {
        let mut result = Vec::with_capacity(n.len() + 1);
        let mut carry = 0;
        for &digit in n {
            let prod = digit as u16 * factor as u16 + carry;
            result.push((prod % 10) as u8);
            carry = prod / 10;
        }
        if carry > 0 {
            result.push(carry as u8);
        }
        result
    };

    let add_vectors = |a: &Vec<u8>, b: &Vec<u8>| -> Vec<u8> {
        let len = a.len().max(b.len());
        let mut result = Vec::with_capacity(len + 1);
        let mut carry = 0;
        for i in 0..len {
            let av = if i < a.len() { a[i] } else { 0 };
            let bv = if i < b.len() { b[i] } else { 0 };
            let sum = av as u16 + bv as u16 + carry;
            result.push((sum % 10) as u8);
            carry = sum / 10;
        }
        if carry > 0 {
            result.push(carry as u8);
        }
        result
    };

    let part2 = mul_single(&digits, 2);

    let part4 = mul_single(&digits, 4);

    let mut part20 = vec![0];
    part20.extend(&part2);

    let mut part2000 = vec![0, 0, 0];
    part2000.extend(&part2);

    let sum_1 = add_vectors(&part2000, &part20);

    let sum_final = add_vectors(&sum_1, &part4);

    let mut result_str: String = sum_final
        .into_iter()
        .rev()
        .map(|d| (d + b'0') as char)
        .collect();

    while result_str.len() > 1 && result_str.starts_with('0') {
        result_str.remove(0);
    }

    result_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "125 17";

        let stone_count = get_stone_count(6, input);
        assert_eq!(stone_count, 22);

        let stone_count = get_stone_count(25, input);
        assert_eq!(stone_count, 55312);
    }
}
