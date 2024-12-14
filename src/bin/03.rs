advent_of_code::solution!(3);

/// Copied from https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2024/day03.rs
///
/// My first solution using split() is about 20x slower than this custom parser. I wonder if it
/// could be made even faster..
fn parse_and_multiply(input: &str, enable_dos_and_donts: bool) -> u32 {
    let mem = input.as_bytes();
    let mut i = 0;
    let mut sum = 0;
    let mut enabled = true;

    while i < mem.len() {
        if mem[i] != b'm' && mem[i] != b'd' {
            i += 1;
            continue;
        }

        if mem[i..].starts_with(b"mul(") {
            i += 4;
        } else if mem[i..].starts_with(b"do()") {
            i += 4;
            enabled = true;
        } else if mem[i..].starts_with(b"don't()") {
            i += 7;
            enabled = false;
        } else {
            i += 1;
            continue;
        }

        let mut first = 0;

        while mem[i].is_ascii_digit() {
            first = 10 * first + (mem[i] - b'0') as u32;
            i += 1;
        }

        if mem[i] != b',' {
            continue;
        }

        i += 1;

        let mut second = 0;

        while mem[i].is_ascii_digit() {
            second = 10 * second + (mem[i] - b'0') as u32;
            i += 1;
        }

        if mem[i] != b')' {
            continue;
        }

        i += 1;

        if !enable_dos_and_donts || enabled {
            sum += first * second;
        }
    }

    sum
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse_and_multiply(input, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse_and_multiply(input, true))
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
