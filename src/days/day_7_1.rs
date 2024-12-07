use std::fs;

pub fn run() {
    let file_path = "src/inputs/day_7.txt";
    let equations = parse_input(file_path);
    let result = get_valid_equation_result(&equations);

    println!("[Day 7, Task 1] Sum of valid calculations: {}", result);
}

fn parse_input(file_path: &str) -> Vec<(i64, Vec<i64>)> {
    let content = fs::read_to_string(file_path).expect("Failed to read input file");

    let parsed = content
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            let key = parts.next().unwrap().trim().parse::<i64>().unwrap();
            let values = parts
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            (key, values)
        })
        .collect::<Vec<(i64, Vec<i64>)>>();

    parsed
}

fn get_valid_equation_result(equations: &Vec<(i64, Vec<i64>)>) -> i64 {
    let mut result = 0;
    for equation in equations {
        if check_if_equation_valid(equation) {
            result += equation.0;
        }
    }
    return result;
}

fn check_if_equation_valid(equation: &(i64, Vec<i64>)) -> bool {
    let n = equation.1.len();
    if n < 2 {
        return equation.1.get(0) == Some(&equation.0);
    }

    (0..1 << (n - 1)).any(|combination| {
        equation
            .1
            .iter()
            .skip(1)
            .enumerate()
            .fold(equation.1[0], |result, (i, &num)| {
                if (combination >> i) & 1 == 1 {
                    result * num
                } else {
                    result + num
                }
            })
            == equation.0
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_valid_equation_result() {
        let equations = vec![
            (190, vec![10, 19]),
            (3267, vec![81, 40, 27]),
            (83, vec![17, 5]),
            (156, vec![15, 6]),
            (7290, vec![6, 8, 6, 15]),
            (161011, vec![16, 10, 13]),
            (192, vec![17, 8, 14]),
            (21037, vec![9, 7, 18, 13]),
            (292, vec![11, 6, 16, 20]),
        ];

        assert_eq!(get_valid_equation_result(&equations), 3749);
    }
}
