use std::collections::{HashMap, HashSet};
use std::fs;

pub fn run() {
    let file_path = "src/inputs/day_6.txt";
    let (map, start_position) = parse_input(file_path);
    let result = get_loop_scenario_count(map, start_position);

    println!(
        "[Day 6, Task 2] Number of box placements causing a loop: {}",
        result
    );
}

fn parse_input(file_path: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let content = fs::read_to_string(file_path).expect("Failed to read input file");
    let map: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    let start_position = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == '^').map(|x| (y, x)))
        .expect("Guard position not found");

    (map, start_position)
}

fn check_path(
    map: &[Vec<char>],
    start_pos: Option<(usize, usize)>,
    start_dir_idx: Option<usize>,
) -> (
    bool,
    HashSet<(usize, usize)>,
    HashMap<(usize, usize), ((usize, usize), usize)>,
) {
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let rows = map.len();
    let cols = map[0].len();

    let initial_pos = start_pos.unwrap_or_else(|| {
        map.iter()
            .enumerate()
            .find_map(|(y, row)| row.iter().position(|&c| c == '^').map(|x| (y, x)))
            .expect("Guard position not found")
    });

    let mut direction_idx = start_dir_idx.unwrap_or(0);
    let mut visited_positions = HashSet::new();
    let mut visited_paths = HashMap::new();
    let mut current_pos = initial_pos;

    visited_positions.insert(current_pos);

    loop {
        let (dy, dx) = directions[direction_idx];
        let next_pos = (
            (current_pos.0 as isize + dy) as usize,
            (current_pos.1 as isize + dx) as usize,
        );

        if next_pos.0 >= rows || next_pos.1 >= cols {
            return (true, visited_positions, visited_paths);
        }

        if map[next_pos.0][next_pos.1] == '#' {
            direction_idx = (direction_idx + 1) % 4;
            continue;
        }

        visited_positions.insert(next_pos);

        if let Some(&previous_entry) = visited_paths.get(&next_pos) {
            if previous_entry == (current_pos, direction_idx) {
                return (false, HashSet::new(), HashMap::new());
            }
        } else {
            visited_paths.insert(next_pos, (current_pos, direction_idx));
        }

        current_pos = next_pos;
    }
}

fn get_loop_scenario_count(map: Vec<Vec<char>>, start_position: (usize, usize)) -> usize {
    let (has_exited, visited_positions, visited_paths) =
        check_path(&map, Some(start_position), None);

    if !has_exited {
        return 0;
    }

    visited_positions
        .iter()
        .filter(|&&(y, x)| (y, x) != start_position)
        .filter(|&&(y, x)| {
            let mut modified_map = map.clone();
            modified_map[y][x] = '#';

            if let Some(&(previous_pos, direction_idx)) = visited_paths.get(&(y, x)) {
                let (exited_after_modification, _, _) =
                    check_path(&modified_map, Some(previous_pos), Some(direction_idx));
                !exited_after_modification
            } else {
                false
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loop_scenario_count() {
        let map = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
        ];
        let start_position = (6, 4);
        let result = get_loop_scenario_count(map, start_position);
        assert_eq!(result, 6);
    }
}
