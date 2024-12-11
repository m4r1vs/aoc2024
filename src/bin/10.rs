use std::collections::HashSet;

advent_of_code::solution!(10);

struct Grid {
    width: usize,
    height: usize,
    content: Vec<Vec<u8>>,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Option<&u8> {
        self.content.get(y).and_then(|row| row.get(x))
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        Self {
            width: input.find("\n").unwrap(),
            height: input.lines().count(),
            content: input
                .lines()
                .map(|l| l.trim_end().bytes().map(|b| b - b'0').collect())
                .collect(),
        }
    }
}

fn get_score(
    start_x: usize,
    start_y: usize,
    current: u8,
    grid: &Grid,
    visited: &mut Option<&mut HashSet<(usize, usize)>>,
) -> usize {
    [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .iter()
        .map(|dir| {
            (
                (dir.0 + start_x as isize) as usize,
                (dir.1 + start_y as isize) as usize,
            )
        })
        .map(|(x, y)| match grid.get(x, y) {
            Some(spot) => {
                if current == 1 && *spot == 0 && visited.as_mut().map_or(true, |v| v.insert((x, y)))
                {
                    return 1;
                }
                if *spot == current - 1 {
                    return get_score(x, y, *spot, grid, visited);
                }
                0
            }
            None => 0,
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::from(input);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut sum = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            if *grid.get(x, y).unwrap() == 9 {
                sum += get_score(x, y, 9, &grid, &mut Some(&mut visited));
                visited.clear();
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::from(input);
    let mut sum = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            if *grid.get(x, y).unwrap() == 9 {
                sum += get_score(x, y, 9, &grid, &mut None);
            }
        }
    }

    Some(sum)
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
