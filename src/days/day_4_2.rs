use std::fs;
use std::io;

pub fn run() {
    let file_path = "src/inputs/day_4.txt";
    let word_grid = read_file_to_grid(file_path).expect("Failed to read input file");
    let result = count_mas_x_shapes(&word_grid);

    println!(
        "[Day 4, Task 2] The result count of X-MAS matches: {}",
        result
    );
}

fn count_mas_x_shapes(word_grid: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    let rows = word_grid.len();
    let cols = if rows > 0 { word_grid[0].len() } else { 0 };

    for y in 0..rows - 2 {
        for x in 0..cols - 2 {
            if word_grid[y][x] == 'M'
                && word_grid[y + 1][x + 1] == 'A'
                && word_grid[y + 2][x + 2] == 'S'
                && word_grid[y + 2][x] == 'M'
                && word_grid[y][x + 2] == 'S'
            {
                result += 1;
                println!("Found valid 'MAS' X at position ({}, {})", y + 1, x + 1);
            }
            if word_grid[y][x] == 'M'
                && word_grid[y + 1][x + 1] == 'A'
                && word_grid[y + 2][x + 2] == 'S'
                && word_grid[y + 2][x] == 'S'
                && word_grid[y][x + 2] == 'M'
            {
                result += 1;
                println!("Found valid 'MAS' X at position ({}, {})", y + 1, x + 1);
            }
            if word_grid[y][x] == 'S'
                && word_grid[y + 1][x + 1] == 'A'
                && word_grid[y + 2][x + 2] == 'M'
                && word_grid[y + 2][x] == 'M'
                && word_grid[y][x + 2] == 'S'
            {
                result += 1;
                println!("Found valid 'MAS' X at position ({}, {})", y + 1, x + 1);
            }
            if word_grid[y][x] == 'S'
                && word_grid[y + 1][x + 1] == 'A'
                && word_grid[y + 2][x + 2] == 'M'
                && word_grid[y + 2][x] == 'S'
                && word_grid[y][x + 2] == 'M'
            {
                result += 1;
                println!("Found valid 'MAS' X at position ({}, {})", y + 1, x + 1);
            }
        }
    }

    result
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
    fn test_mas_x_shape_valid() {
        let grid = vec![
            vec!['.', 'M', '.', 'S', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', 'A', '.', '.', 'M', 'S', 'M', 'S', '.'],
            vec!['.', 'M', '.', 'S', '.', 'M', 'A', 'A', '.', '.'],
            vec!['.', '.', 'A', '.', 'A', 'S', 'M', 'S', 'M', '.'],
            vec!['.', 'M', '.', 'S', '.', 'M', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['S', '.', 'S', '.', 'S', '.', 'S', '.', 'S', '.'],
            vec!['.', 'A', '.', 'A', '.', 'A', '.', 'A', '.', '.'],
            vec!['M', '.', 'M', '.', 'M', '.', 'M', '.', 'M', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(count_mas_x_shapes(&grid), 9);

        let grid = vec![
            vec!['M', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'S'],
        ];
        assert_eq!(count_mas_x_shapes(&grid), 1);

        let grid = vec![
            vec!['M', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'S'],
        ];
        assert_eq!(count_mas_x_shapes(&grid), 1);

        let grid = vec![
            vec!['S', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'M'],
        ];
        assert_eq!(count_mas_x_shapes(&grid), 1);

        let grid = vec![
            vec!['S', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'M'],
        ];
        assert_eq!(count_mas_x_shapes(&grid), 1);

        let grid = vec![
            vec!['S', '.', 'S', '.', 'S'],
            vec!['.', 'A', '.', 'A', '.'],
            vec!['M', '.', 'M', '.', 'M'],
        ];
        assert_eq!(count_mas_x_shapes(&grid), 2);
    }

    #[test]
    fn test_mas_x_shape_invalid() {
        let grid = vec![
            vec!['S', 'X', 'X'],
            vec!['X', 'A', 'X'],
            vec!['M', 'X', 'M'],
        ];

        assert_eq!(count_mas_x_shapes(&grid), 0);

        let grid = vec![
            vec!['X', 'M', 'X'],
            vec!['M', 'A', 'S'],
            vec!['X', 'S', 'X'],
        ];

        assert_eq!(count_mas_x_shapes(&grid), 0);

        let grid = vec![
            vec!['M', 'M', 'S'],
            vec!['M', 'A', 'S'],
            vec!['S', 'S', 'M'],
        ];

        assert_eq!(count_mas_x_shapes(&grid), 0);
    }
}
