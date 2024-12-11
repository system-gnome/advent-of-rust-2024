use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn run() {
    let file_name = "src/inputs/day_8.txt";
    let input = fs::read_to_string(file_name).unwrap();

    let antennas = parse_input(&input);
    let (width, height) = get_dimensions(&input);

    let antinode_count = place_antinodes_and_return_count(&antennas, width, height);
    println!(
        "[Day 8, Task 1] The count of antinode locations: {}",
        antinode_count
    );
}

fn place_antinodes_and_return_count(
    antennas: &HashMap<char, Vec<Point>>,
    width: i32,
    height: i32,
) -> usize {
    antennas
        .iter()
        .fold(
            HashSet::new(),
            |mut antinode_locations: HashSet<Point>, (_, locations)| {
                for i in 0..locations.len() {
                    for j in i + 1..locations.len() {
                        let a = locations[i];
                        let b = locations[j];
                        let dist_vec = subtract_points(a, b);

                        let mut na = add_points(a, dist_vec);
                        while check_point_inbounds(na, width, height) {
                            antinode_locations.insert(na);
                            na = add_points(na, dist_vec);
                        }

                        let mut nb = subtract_points(b, dist_vec);
                        while check_point_inbounds(nb, width, height) {
                            antinode_locations.insert(nb);
                            nb = subtract_points(nb, dist_vec);
                        }
                    }
                }
                if locations.len() > 1 {
                    antinode_locations.extend(locations.iter());
                }

                antinode_locations
            },
        )
        .len()
}

fn parse_input(input: &str) -> HashMap<char, Vec<Point>> {
    input
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut map, (i, line)| {
            line.char_indices()
                .filter(|&(_, c)| c != '.')
                .for_each(|(j, c)| {
                    map.entry(c).or_insert_with(Vec::new).push(Point {
                        x: j as i32,
                        y: i as i32,
                    })
                });
            map
        })
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn get_dimensions(input: &str) -> (i32, i32) {
    let width = input.split_once('\n').unwrap().0.len();
    let height = input.lines().count();
    (width as i32, height as i32)
}

fn check_point_inbounds(p: Point, width: i32, height: i32) -> bool {
    0 <= p.x && p.x < width && 0 <= p.y && p.y < height
}

fn add_points(a: Point, b: Point) -> Point {
    Point {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}

fn subtract_points(a: Point, b: Point) -> Point {
    Point {
        x: a.x - b.x,
        y: a.y - b.y,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_antinodes_and_return_count() {
        let input = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let antennas = parse_input(input);
        let (width, height) = get_dimensions(input);
        let antinode_count = place_antinodes_and_return_count(&antennas, width, height);
        assert_eq!(antinode_count, 34);
    }
}
