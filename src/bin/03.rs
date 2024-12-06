advent_of_code::solution!(3);

fn parse_and_multiply(input: &str) -> u32 {
    let parts: Vec<&str> = input.split(",").collect();

    if parts.len() != 2 {
        return 0;
    }

    let left = parts[0].trim().parse::<u32>();
    let right = parts[1].trim().parse::<u32>();

    if let (Ok(l), Ok(r)) = (left, right) {
        l * r
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut product = 0;
    for might_be_mul in input.split("mul(") {
        let first_closing_bracket = match might_be_mul.find(")") {
            Some(x) => x,
            None => continue,
        };

        product += parse_and_multiply(&might_be_mul[..first_closing_bracket])
    }

    Some(product)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
