use std::collections::HashMap;

advent_of_code::solution!(11);

fn log10(mut number: usize) -> usize {
    let mut count = 0;
    while number >= 10 {
        number /= 10;
        count += 1;
    }
    count
}

fn split_number_if_even(number: usize) -> Option<(usize, usize)> {
    let digits = log10(number) + 1;

    if digits % 2 == 0 {
        let divisor = 10usize.pow(digits as u32 / 2);
        return Some((number / divisor, number % divisor));
    }

    None
}

fn blink_on_stone(
    stone: usize,
    mut blink_count: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if blink_count == 0 {
        return 1;
    }

    blink_count -= 1;

    if let Some(result) = cache.get(&(stone, blink_count)) {
        return *result;
    }

    let result;

    // rule 1: 0 becomes 1
    if stone == 0 {
        result = blink_on_stone(1, blink_count, cache);
    }
    // rule 2: even number => split into 2
    else if let Some((left, right)) = split_number_if_even(stone) {
        result =
            blink_on_stone(left, blink_count, cache) + blink_on_stone(right, blink_count, cache);
    }
    // rule 3: odd number => multiply by 2024
    else {
        result = blink_on_stone(stone * 2024, blink_count, cache);
    }

    cache.insert((stone, blink_count), result);

    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut cache: HashMap<(usize, usize), usize> = HashMap::with_capacity(5000);

    Some(
        parse_input(input)
            .iter()
            .map(|&stone| blink_on_stone(stone, 25, &mut cache))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut cache: HashMap<(usize, usize), usize> = HashMap::with_capacity(150000);

    Some(
        parse_input(input)
            .iter()
            .map(|&stone| blink_on_stone(stone, 75, &mut cache))
            .sum(),
    )
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
        .collect()
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
