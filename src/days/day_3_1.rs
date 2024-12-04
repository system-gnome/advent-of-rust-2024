use std::fs;
use std::io;

pub fn run() {
    let file_path = "src/inputs/day_3.txt";
    let parsed_memory = read_memory_and_parse(file_path).unwrap();
    let result = validate_memory_from_file(&parsed_memory);
    println!(
        "[Day 3, Task 1] - The result of valid memory calculations: {}",
        result
    )
}

fn read_memory_and_parse(file_path: &str) -> io::Result<String> {
    let content = fs::read_to_string(file_path)?;
    Ok(content.replace('\n', ""))
}

fn validate_memory_from_file(memory: &str) -> i32 {
    let mut total_sum = 0;
    let bytes = memory.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index..].starts_with(b"mul(") {
            index += 4; // Move past "mul("

            let start_num1 = index;
            let mut digit_count = 0;
            while index < bytes.len() && bytes[index].is_ascii_digit() && digit_count < 3 {
                index += 1;
                digit_count += 1;
            }
            let num1_str = &memory[start_num1..index];

            if num1_str.is_empty() {
                continue;
            }

            if index < bytes.len() && bytes[index] == b',' {
                index += 1; // Move past ','

                let start_num2 = index;
                digit_count = 0;
                while index < bytes.len() && bytes[index].is_ascii_digit() && digit_count < 3 {
                    index += 1;
                    digit_count += 1;
                }
                let num2_str = &memory[start_num2..index];

                if num2_str.is_empty() {
                    continue;
                }

                if index < bytes.len() && bytes[index] == b')' {
                    index += 1; // Move past ')'

                    if let (Ok(num1), Ok(num2)) = (num1_str.parse::<i32>(), num2_str.parse::<i32>())
                    {
                        total_sum += num1 * num2;
                    }
                    continue;
                }
            }
        }
        index += 1;
    }
    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_memory() {
        let memory_string =
            String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(validate_memory_from_file(&memory_string), 161);
    }
}
