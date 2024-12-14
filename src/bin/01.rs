use std::collections::HashMap;

advent_of_code::solution!(1);

/// The Chief Historian is missing! We found two lists of location IDs.
/// We need to find the difference between them by sorting both lists.
pub fn part_one(input: &str) -> Option<u32> {
    let line_count = input.len() / 14;

    let mut left_list: Vec<u32> = Vec::with_capacity(line_count);
    let mut right_list: Vec<u32> = Vec::with_capacity(line_count);

    input.lines().map(parse_input).for_each(|(left, right)| {
        left_list.push(left);
        right_list.push(right);
    });

    left_list.sort_unstable();
    right_list.sort_unstable();

    Some(
        left_list
            .iter()
            .zip(right_list)
            .map(|(left, right)| left.abs_diff(right))
            .sum(),
    )
}

/// The difference is HUGEE!!! Or is it??
/// Maybe our methods were wrong. Find out how often a number on the left
/// appears on the right!
pub fn part_two(input: &str) -> Option<u32> {
    let line_count = input.len() / 14;
    let mut left_list: Vec<u32> = Vec::with_capacity(line_count);
    let mut right_list: HashMap<u32, u32> = HashMap::with_capacity(line_count);

    input.lines().map(parse_input).for_each(|(left, right)| {
        left_list.push(left);
        right_list.entry(right).and_modify(|v| *v += 1).or_insert(1);
    });

    Some(
        left_list
            .iter()
            .map(|left| {
                if let Some(right) = right_list.get(left) {
                    return left * right;
                }
                0
            })
            .sum(),
    )
}

fn parse_input(input: &str) -> (u32, u32) {
    (
        input.get(0..5).map(str::parse::<u32>).unwrap().unwrap(),
        input.get(8..13).map(str::parse::<u32>).unwrap().unwrap(),
    )
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
