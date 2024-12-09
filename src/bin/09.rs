advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let mut defragged = parse_input(input);
    let mut last_find_index = 0;
    while let Some(id) = defragged.pop() {
        if id == usize::MAX {
            continue;
        }
        match defragged[last_find_index..]
            .iter_mut()
            .enumerate()
            .find(|(_, s)| *s == &usize::MAX)
        {
            Some((i, free)) => {
                *free = id;
                last_find_index += i + 1;
            }
            None => {
                defragged.push(id);
                break;
            }
        };
    }
    Some(defragged.iter().enumerate().map(|(i, id)| i * id).sum())
}

fn parse_input(input: &str) -> Vec<usize> {
    let mut defragged: Vec<usize> = Vec::new();

    let mut is_file = true;
    let mut id = 0;
    for byte in input.trim_end().bytes() {
        if is_file {
            for _ in 0..(byte - 48) {
                defragged.push(id);
            }
            id += 1;
        } else {
            for _ in 0..(byte - 48) {
                defragged.push(usize::MAX)
            }
        }
        is_file = !is_file;
    }

    defragged
}

pub fn part_two(input: &str) -> Option<usize> {
    println!("{}", input);
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
