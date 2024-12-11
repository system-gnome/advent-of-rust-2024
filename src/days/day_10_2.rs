use std::fs;

pub fn run() {
    let file_path = "src/inputs/day_10.txt";
    let input_content = fs::read_to_string(file_path).expect("Failed to read input file");
    let total_trailheads = calculate_trailheads(&input_content);
    println!(
        "[Day 10, Task 2] Result of trailheads: {}",
        total_trailheads
    );
}

pub fn calculate_trailheads(input: &str) -> i32 {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|character| character.to_digit(10).map(|digit| digit as u8))
                .collect()
        })
        .collect();

    let num_rows = grid.len();
    let num_cols = if num_rows > 0 { grid[0].len() } else { 0 };

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut trailhead_count = 0;

    let mut starting_positions = vec![];
    for row in 0..num_rows {
        for col in 0..num_cols {
            if grid[row][col] == 0 {
                starting_positions.push((row, col));
            }
        }
    }

    for start_position in starting_positions {
        let mut position_queue = std::collections::VecDeque::new();

        position_queue.push_back((start_position, 0));

        while let Some(((current_row, current_col), current_step)) = position_queue.pop_front() {
            if grid[current_row][current_col] != current_step {
                continue;
            }

            if current_step == 9 {
                trailhead_count += 1;
                continue;
            }

            for &(row_offset, col_offset) in &directions {
                let next_row = current_row as isize + row_offset;
                let next_col = current_col as isize + col_offset;

                if next_row >= 0
                    && next_row < num_rows as isize
                    && next_col >= 0
                    && next_col < num_cols as isize
                {
                    let next_row = next_row as usize;
                    let next_col = next_col as usize;

                    position_queue.push_back(((next_row, next_col), current_step + 1));
                }
            }
        }
    }

    trailhead_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_trailheads() {
        let input =
            "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";
        let total_trailheads = calculate_trailheads(&input);
        assert_eq!(total_trailheads, 81);
    }
}
