use std::fs;

pub fn run() {
    let filename = "src/inputs/day_9.txt";
    let input = fs::read_to_string(filename).expect("Failed to read input file");
    let result = calculate_checksum_compacted(input.trim());
    println!(
        "[Day 9, Task 2] The resulting compacted checksum: {}",
        result
    );
}

fn calculate_checksum_compacted(disk_map: &str) -> u64 {
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

    let max_file_id = if file_id == 0 { 0 } else { file_id - 1 };

    for fid in (0..=max_file_id).rev() {
        let positions: Vec<usize> = disk
            .iter()
            .enumerate()
            .filter_map(|(idx, &b)| if b == Some(fid) { Some(idx) } else { None })
            .collect();

        if positions.is_empty() {
            continue;
        }

        let file_size = positions.len();

        let file_leftmost = *positions.first().unwrap();

        let free_spans = find_free_spans(&disk, file_leftmost);

        if let Some((start, _end)) = free_spans
            .into_iter()
            .find(|&(s, e)| (e - s + 1) >= file_size)
        {
            for &pos in &positions {
                disk[pos] = None;
            }

            for offset in 0..file_size {
                disk[start + offset] = Some(fid);
            }
        }
    }

    let mut checksum: u64 = 0;
    for (i, block) in disk.iter().enumerate() {
        if let Some(fid) = block {
            checksum += (i as u64) * (*fid as u64);
        }
    }

    checksum
}

fn find_free_spans(disk: &[Option<u32>], limit: usize) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut in_span = false;
    let mut span_start = 0;

    for i in 0..limit {
        if disk[i].is_none() && !in_span {
            in_span = true;
            span_start = i;
        } else if disk[i].is_some() && in_span {
            spans.push((span_start, i - 1));
            in_span = false;
        }
    }

    if in_span {
        spans.push((span_start, limit - 1));
    }

    spans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_checksum_compacted() {
        let input = "2333133121414131402";
        let chksum = calculate_checksum_compacted(input);
        assert_eq!(chksum, 2858);
    }
}
