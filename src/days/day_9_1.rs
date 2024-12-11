use std::fs;

pub fn run() {
    let filename = "src/inputs/day_9.txt";
    let input = fs::read_to_string(filename).expect("Failed to read input file");
    let result = calculate_checksum(input.trim());
    println!("[Day 9, Task 1] The resulting checksum: {}", result);
}

fn calculate_checksum(disk_map: &str) -> u64 {
    let chars: Vec<char> = disk_map.chars().collect();
    let mut file_lengths = Vec::new();
    let mut free_lengths = Vec::new();

    let mut i = 0;
    while i < chars.len() {
        let file_len = chars[i].to_digit(10).expect("Invalid file length") as usize;
        file_lengths.push(file_len);
        i += 1;
        if i < chars.len() {
            let free_len = chars[i].to_digit(10).expect("Invalid free length") as usize;
            free_lengths.push(free_len);
            i += 1;
        } else {
            free_lengths.push(0);
        }
    }

    if free_lengths.len() < file_lengths.len() {
        free_lengths.push(0);
    }

    let mut disk: Vec<Option<u32>> = Vec::new();
    let mut file_id = 0_u32;
    for (&f_len, &fr_len) in file_lengths.iter().zip(free_lengths.iter()) {
        if f_len > 0 {
            for _ in 0..f_len {
                disk.push(Some(file_id));
            }
            file_id += 1;
        }
        for _ in 0..fr_len {
            disk.push(None);
        }
    }

    loop {
        let left_dot = disk.iter().position(|&b| b.is_none());
        if left_dot.is_none() {
            break;
        }
        let left_dot = left_dot.unwrap();

        let right_file = disk.iter().rposition(|&b| b.is_some());
        if right_file.is_none() {
            break;
        }
        let right_file = right_file.unwrap();

        if right_file <= left_dot {
            break;
        }

        let file_block = disk[right_file];
        disk[left_dot] = file_block;
        disk[right_file] = None;
    }

    let mut checksum: u64 = 0;
    for (i, &block) in disk.iter().enumerate() {
        if let Some(fid) = block {
            checksum += (i as u64) * (fid as u64);
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_checksum() {
        let input = "2333133121414131402";
        let chksum = calculate_checksum(input);
        assert_eq!(chksum, 1928);
    }
}
