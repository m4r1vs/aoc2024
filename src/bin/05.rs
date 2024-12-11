use std::cmp::Ordering::*;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    let (rules, page_orderings) = parse_input(input);

    Some(
        page_orderings
            .iter()
            .filter(|ordering| ordering.is_sorted_by(|&left, &right| rules[left][right] == Less))
            .map(|ordering| ordering[ordering.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (rules, mut page_orderings) = parse_input(input);

    Some(
        page_orderings
            .iter_mut()
            .filter(|ordering| !ordering.is_sorted_by(|&left, &right| rules[left][right] == Less))
            .map(|new_ordering| {
                let middle = new_ordering.len() / 2;

                // this method orders until the middle and returns (left, middle, right)
                *new_ordering
                    .select_nth_unstable_by(middle, |&left, &right| rules[left][right])
                    .1
            })
            .sum(),
    )
}

fn parse_input(input: &str) -> ([[std::cmp::Ordering; 100]; 100], Vec<Vec<usize>>) {
    let mut rules = [[Greater; 100]; 100];

    let (rules_str, page_orderings_str) = input.split_once("\n\n").unwrap();

    for line in rules_str.lines() {
        let (left, right): (usize, usize) = line
            .split_once("|")
            .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
            .unwrap();

        rules[left][right] = Less;
    }

    let page_orderings: Vec<Vec<usize>> = page_orderings_str
        .lines()
        .map(|line| line.split(",").filter_map(|s| s.parse().ok()).collect())
        .collect();

    (rules, page_orderings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
