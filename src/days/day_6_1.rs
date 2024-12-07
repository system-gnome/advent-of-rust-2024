use std::collections::HashSet;

pub fn run() {
    let file_path = "src/inputs/day_6.txt";
    let (map, start_position) = parse_input(file_path);
    let result = calculate_unique_fields(map, start_position);

    println!(
        "[Day 6, Task 1] The Guard traversed {} unique fields.",
        result
    );
}

fn parse_input(file_path: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let content = std::fs::read_to_string(file_path).expect("Failed to read input file");

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut guard_position: (usize, usize) = (0, 0);

    for (y, line) in content.lines().enumerate() {
        let row: Vec<char> = line.chars().collect();
        if let Some(x) = row.iter().position(|&c| c == '^') {
            guard_position = (y, x);
        }
        map.push(row);
    }

    (map, guard_position)
}

fn calculate_unique_fields(map: Vec<Vec<char>>, start_position: (usize, usize)) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut current_position = start_position;
    let mut direction = (-1, 0);

    let height = map.len();
    let width = map[0].len();

    loop {
        visited.insert(current_position);

        let next_position = (
            (current_position.0 as isize + direction.0) as usize,
            (current_position.1 as isize + direction.1) as usize,
        );

        if next_position.0 >= height || next_position.1 >= width {
            break;
        }

        let next_char = map[next_position.0][next_position.1];

        match next_char {
            '.' => {
                current_position = next_position;
            }
            '#' => {
                direction = match direction {
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    _ => direction,
                };
            }
            _ => {
                break;
            }
        }

        if visited.contains(&current_position) {
            continue;
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_unique_fields() {
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
        let result = calculate_unique_fields(map, start_position);
        assert_eq!(result, 41);
    }
}
