use std::collections::HashMap;

advent_of_code::solution!(5);

fn is_ordering_correct(page_ordering: &[usize], rules: &HashMap<usize, Vec<usize>>) -> bool {
    for (i, page) in page_ordering.iter().enumerate() {
        for next in page_ordering.iter().skip(i + 1) {
            if let Some(rule) = rules.get(page) {
                if rule.contains(next) {
                    return false;
                }
            }
        }
    }

    true
}

// TODO: Very brute-forced method. Do better. https://en.wikipedia.org/wiki/Adjacency_list ?
fn fix_ordering(page_ordering: &[usize], rules: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut fixed_ordering: Vec<usize> = page_ordering.to_owned();

    while !is_ordering_correct(&fixed_ordering, rules) {
        for (i, page) in fixed_ordering.clone().iter().enumerate() {
            for j in (i + 1)..(fixed_ordering.len()) {
                if let Some(rule) = rules.get(page) {
                    if rule.contains(&fixed_ordering[j]) {
                        fixed_ordering.swap(i, j);
                    }
                }
            }
        }
    }

    fixed_ordering
}

pub fn part_one(input: &str) -> Option<usize> {
    let (rules, page_orderings) = parse_input(input);

    Some(
        page_orderings
            .iter()
            .filter(|ordering| is_ordering_correct(ordering, &rules))
            .map(|ordering| ordering[ordering.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (rules, page_orderings) = parse_input(input);

    Some(
        page_orderings
            .iter()
            .filter(|ordering| !is_ordering_correct(ordering, &rules))
            .map(|ordering| fix_ordering(ordering, &rules))
            .map(|orderring| orderring[orderring.len() / 2])
            .sum(),
    )
}

fn parse_input(input: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut rules = HashMap::<usize, Vec<usize>>::new();

    let (rules_str, page_orderings_str) = input.split_once("\n\n").unwrap();

    rules_str.lines().for_each(|line| {
        let (left, right) = line.split_once("|").unwrap();
        let right_parsed = &right.parse::<usize>().unwrap();
        match rules.get_mut(right_parsed) {
            Some(v) => v.push(left.parse::<usize>().unwrap()),
            None => {
                rules.insert(*right_parsed, vec![left.parse::<usize>().unwrap()]);
            }
        };
    });

    let page_orderings = page_orderings_str
        .lines()
        .map(|line| {
            line.split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

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
