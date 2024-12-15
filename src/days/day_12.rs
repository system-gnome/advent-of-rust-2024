use std::collections::{HashSet, VecDeque};

pub fn run() {
    let file_path = "src/inputs/day_12.txt";
    let input_content = std::fs::read_to_string(file_path).expect("Failed to read input file");

    let fencing_costs = get_fencing_costs(&input_content);
    println!(
        "[Day 12, Task 1] The resulting fencing costs are: {}",
        fencing_costs
    );

    let fencing_costs_discounted = get_fencing_costs_discounted(&input_content);
    println!(
        "[Day 12, Task 2] The resulting fencing costs with discount are: {}",
        fencing_costs_discounted
    );
}

fn get_fencing_costs(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }

    let rows = lines.len();
    let cols = lines[0].len();
    let grid: Vec<Vec<char>> = lines.iter().map(|&line| line.chars().collect()).collect();

    let mut visited = vec![vec![false; cols]; rows];
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut total_cost = 0;

    for r in 0..rows {
        for c in 0..cols {
            if visited[r][c] {
                continue;
            }

            let plant_type = grid[r][c];
            let region_cells =
                flood_fill_region((r, c), &grid, &mut visited, plant_type, &directions);

            let area = region_cells.len() as i32;

            let perimeter = calculate_perimeter(&region_cells, &grid, &directions);

            total_cost += area * perimeter;
        }
    }

    total_cost
}

fn flood_fill_region(
    start: (usize, usize),
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    plant_type: char,
    directions: &[(i32, i32)],
) -> Vec<(usize, usize)> {
    let mut stack = vec![start];
    let mut region_cells = Vec::new();

    while let Some((cr, cc)) = stack.pop() {
        if visited[cr][cc] {
            continue;
        }
        visited[cr][cc] = true;
        region_cells.push((cr, cc));

        for &(dr, dc) in directions {
            let nr = cr as i32 + dr;
            let nc = cc as i32 + dc;
            if nr >= 0 && nr < grid.len() as i32 && nc >= 0 && nc < grid[0].len() as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                if !visited[nr][nc] && grid[nr][nc] == plant_type {
                    stack.push((nr, nc));
                }
            }
        }
    }

    region_cells
}

fn calculate_perimeter(
    region_cells: &[(usize, usize)],
    grid: &Vec<Vec<char>>,
    directions: &[(i32, i32)],
) -> i32 {
    let cell_set: HashSet<(usize, usize)> = region_cells.iter().cloned().collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut perimeter = 0;

    for &(cr, cc) in region_cells {
        for &(dr, dc) in directions {
            let nr = cr as i32 + dr;
            let nc = cc as i32 + dc;
            if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                perimeter += 1;
            } else if !cell_set.contains(&(nr as usize, nc as usize)) {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn get_fencing_costs_discounted(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }

    let original_cols = lines[0].len();
    let border_row = "*".repeat(original_cols + 2);

    let mut map = Vec::new();
    map.push(border_row.clone());

    for line in lines {
        let mut row_str = String::from("*");
        row_str.push_str(line);
        row_str.push('*');
        map.push(row_str);
    }

    map.push(border_row);

    let rows = map.len();
    let cols = map[0].len();
    let mut visited = vec![vec![false; cols]; rows];

    let mut total_cost = 0;

    for r in 0..rows {
        for c in 0..cols {
            if visited[r][c] {
                continue;
            }

            let cell_char = map[r].chars().nth(c).unwrap();
            if cell_char == '*' {
                continue;
            }

            total_cost += calculate_discounted_cost((r, c), &map, &mut visited);
        }
    }

    total_cost
}

fn calculate_discounted_cost(
    start: (usize, usize),
    map: &Vec<String>,
    visited: &mut Vec<Vec<bool>>,
) -> i64 {
    let cell_char = map[start.0].chars().nth(start.1).unwrap();
    let mut area = 0i64;
    let mut corner_sum = 0i64;

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some((rr, cc)) = queue.pop_front() {
        if visited[rr][cc] {
            continue;
        }

        visited[rr][cc] = true;
        area += 1;
        corner_sum += count_corners((rr, cc), map);

        for (dr, dc) in &[(0, -1), (-1, 0), (0, 1), (1, 0)] {
            let nr = rr as isize + dr;
            let nc = cc as isize + dc;
            if nr < 0 || nc < 0 || nr >= map.len() as isize || nc >= map[0].len() as isize {
                continue;
            }

            let (nr, nc) = (nr as usize, nc as usize);
            if map[nr].chars().nth(nc).unwrap() == cell_char && !visited[nr][nc] {
                queue.push_back((nr, nc));
            }
        }
    }

    area * corner_sum
}

fn count_corners(pos: (usize, usize), map: &Vec<String>) -> i64 {
    let (r, c) = pos;
    let cell_char = map[r].chars().nth(c).unwrap();

    let get = |rr: isize, cc: isize| map[rr as usize].chars().nth(cc as usize).unwrap();

    let r = r as isize;
    let c = c as isize;
    let mut corner_count = 0;

    let neighbors = [
        ((1, 0), (0, 1)),
        ((1, 0), (0, -1)),
        ((-1, 0), (0, 1)),
        ((-1, 0), (0, -1)),
    ];

    for &((dr1, dc1), (dr2, dc2)) in &neighbors {
        let nr1 = r + dr1;
        let nc1 = c + dc1;
        let nr2 = r + dr2;
        let nc2 = c + dc2;

        let within_bounds =
            |rr, cc| rr >= 0 && cc >= 0 && rr < map.len() as isize && cc < map[0].len() as isize;

        if within_bounds(nr1, nc1) && within_bounds(nr2, nc2) {
            if get(nr1, nc1) != cell_char && get(nr2, nc2) != cell_char {
                corner_count += 1;
            }
        }
    }

    let squares = [
        ((1, 0), (0, 1), (1, 1)),
        ((1, 0), (0, -1), (1, -1)),
        ((-1, 0), (0, 1), (-1, 1)),
        ((-1, 0), (0, -1), (-1, -1)),
    ];

    for &((dr1, dc1), (dr2, dc2), (dr3, dc3)) in &squares {
        let nr1 = r + dr1;
        let nc1 = c + dc1;
        let nr2 = r + dr2;
        let nc2 = c + dc2;
        let nr3 = r + dr3;
        let nc3 = c + dc3;

        let within_bounds =
            |rr, cc| rr >= 0 && cc >= 0 && rr < map.len() as isize && cc < map[0].len() as isize;
        if within_bounds(nr1, nc1) && within_bounds(nr2, nc2) && within_bounds(nr3, nc3) {
            if get(nr1, nc1) == cell_char
                && get(nr2, nc2) == cell_char
                && get(nr3, nc3) != cell_char
            {
                corner_count += 1;
            }
        }
    }

    corner_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fencing_costs() {
        let input = "RRRRIICCFF\nRRRRIICCCF\nVVRRRCCFFF\nVVRCCCJFFF\nVVVVCJJCFE\nVVIVCCJJEE\nVVIIICJJEE\nMIIIIIJJEE\nMIIISIJEEE\nMMMISSJEEE";
        assert_eq!(get_fencing_costs(input), 1930);
    }

    #[test]
    fn test_get_fencing_costs_discounted() {
        let input = "RRRRIICCFF\nRRRRIICCCF\nVVRRRCCFFF\nVVRCCCJFFF\nVVVVCJJCFE\nVVIVCCJJEE\nVVIIICJJEE\nMIIIIIJJEE\nMIIISIJEEE\nMMMISSJEEE";
        assert_eq!(get_fencing_costs_discounted(input), 1206);
    }
}
