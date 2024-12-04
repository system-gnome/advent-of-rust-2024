use std::fs;
use std::io;

pub fn run() {
    let file_path = "src/inputs/day_4.txt";
    let word_grid = read_file_to_grid(file_path).expect("Failed to read input file");
    let target: Vec<char> = vec!['X', 'M', 'A', 'S'];
    let mut result = 0;

    let directions = [
        (-1, 0),  // North
        (-1, 1),  // Northeast
        (0, 1),   // East
        (1, 1),   // Southeast
        (1, 0),   // South
        (1, -1),  // Southwest
        (0, -1),  // West
        (-1, -1), // Northwest
    ];

    for y in 0..word_grid.len() {
        for x in 0..word_grid[y].len() {
            for &(dy, dx) in &directions {
                if matches_sequence(&word_grid, y as isize, x as isize, dy, dx, &target) {
                    println!(
                        "Found sequence {:?} starting at ({}, {}) in direction ({}, {})",
                        target, y, x, dy, dx
                    );
                    result += 1;
                }
            }
        }
    }

    println!(
        "[Day 4, Task 1] The result count of XMAS matches: {}",
        result
    )
}

fn matches_sequence(
    grid: &Vec<Vec<char>>,
    start_y: isize,
    start_x: isize,
    dy: isize,
    dx: isize,
    target: &[char],
) -> bool {
    let mut y = start_y;
    let mut x = start_x;

    for &char in target {
        if y < 0 || y >= grid.len() as isize || x < 0 || x >= grid[0].len() as isize {
            return false;
        }
        if grid[y as usize][x as usize] != char {
            return false;
        }
        y += dy;
        x += dx;
    }

    true
}

fn read_file_to_grid(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    let content = fs::read_to_string(file_path)?;
    let grid = content.lines().map(|line| line.chars().collect()).collect();
    Ok(grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_sequence_horizontal() {
        let grid = vec![
            vec!['X', 'M', 'A', 'S', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.'],
        ];
        let target: Vec<char> = vec!['X', 'M', 'A', 'S'];

        assert!(matches_sequence(&grid, 0, 0, 0, 1, &target));
        assert!(!matches_sequence(&grid, 0, 1, 0, 1, &target));
    }

    #[test]
    fn test_matches_sequence_vertical() {
        let grid = vec![
            vec!['X', '.', '.', '.'],
            vec!['M', '.', '.', '.'],
            vec!['A', '.', '.', '.'],
            vec!['S', '.', '.', '.'],
        ];
        let target: Vec<char> = vec!['X', 'M', 'A', 'S'];

        assert!(matches_sequence(&grid, 0, 0, 1, 0, &target));
        assert!(!matches_sequence(&grid, 1, 0, 1, 0, &target));
    }

    #[test]
    fn test_matches_sequence_diagonal() {
        let grid = vec![
            vec!['X', '.', '.', '.'],
            vec!['.', 'M', '.', '.'],
            vec!['.', '.', 'A', '.'],
            vec!['.', '.', '.', 'S'],
        ];
        let target: Vec<char> = vec!['X', 'M', 'A', 'S'];

        assert!(matches_sequence(&grid, 0, 0, 1, 1, &target));
        assert!(!matches_sequence(&grid, 0, 1, 1, 1, &target));
    }
}
