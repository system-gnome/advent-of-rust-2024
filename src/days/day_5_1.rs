use std::collections::HashMap;
use std::fs;

pub fn run() {
    let (rules, updates) = parse_input("src/inputs/day_5.txt");

    let result = sum_middle_pages(rules, updates);
    println!("[Day 5, Task 1] The result of middle pages sum: {}", result);
}

fn parse_input(file_path: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let content = fs::read_to_string(file_path).expect("Failed to read input file");
    let mut sections = content.split("\n\n");

    let rules_section = sections.next().expect("Missing rules section");
    let updates_section = sections.next().expect("Missing updates section");

    let rules = rules_section
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();

    let updates = updates_section
        .lines()
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn is_update_correct(order_rules: &[(usize, usize)], update: &[usize]) -> bool {
    let positions: HashMap<_, _> = update
        .iter()
        .enumerate()
        .map(|(i, &page)| (page, i))
        .collect();

    for &(x, y) in order_rules {
        if let (Some(&pos_x), Some(&pos_y)) = (positions.get(&x), positions.get(&y)) {
            if pos_x >= pos_y {
                return false;
            }
        }
    }
    true
}

fn middle_page(update: &[usize]) -> Option<usize> {
    if update.is_empty() {
        None
    } else {
        Some(update[update.len() / 2])
    }
}

fn sum_middle_pages(order_rules: Vec<(usize, usize)>, updates: Vec<Vec<usize>>) -> usize {
    updates
        .into_iter()
        .filter(|update| is_update_correct(&order_rules, update))
        .filter_map(|update| middle_page(&update))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_update_correct() {
        let rules = vec![(47, 53), (97, 13), (97, 61), (97, 47), (75, 29)];
        let update_correct = vec![75, 47, 61, 53, 29];
        let update_incorrect = vec![75, 97, 47, 61, 53];

        assert!(is_update_correct(&rules, &update_correct));
        assert!(!is_update_correct(&rules, &update_incorrect));
    }

    #[test]
    fn test_middle_page() {
        assert_eq!(middle_page(&[75, 47, 61, 53, 29]), Some(61));
        assert_eq!(middle_page(&[97, 61, 53, 29, 13]), Some(53));
        assert_eq!(middle_page(&[75, 29, 13]), Some(29));
        assert_eq!(middle_page(&[]), None);
    }

    #[test]
    fn test_sum_middle_pages() {
        let rules = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        let updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        let result = sum_middle_pages(rules, updates);
        assert_eq!(result, 143);
    }
}
