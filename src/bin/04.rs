use core::panic;

advent_of_code::solution!(4);

#[derive(Debug, PartialEq)]
enum Xmas {
    X,
    M,
    A,
    S,
}

struct Grid {
    letters: Vec<Vec<Xmas>>, // true if there is an obstacle
}

impl Grid {
    fn get(&self, x: isize, y: isize) -> Option<&Xmas> {
        self.letters
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let obstacles: Vec<Vec<Xmas>> = input
            .lines()
            .map(|row| {
                row.bytes()
                    .map(|b| match b {
                        b'X' => Xmas::X,
                        b'M' => Xmas::M,
                        b'A' => Xmas::A,
                        b'S' => Xmas::S,
                        other => panic!("Expected one of XMAS, found {}", other),
                    })
                    .collect()
            })
            .collect();

        Self { letters: obstacles }
    }
}

fn get_xmasses(x: isize, y: isize, grid: &Grid) -> usize {
    let pattern = [Xmas::M, Xmas::A, Xmas::S];

    [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ]
    .iter()
    .filter(|dir| {
        let mut newpos = (x + dir.0, y + dir.1);
        for expected in &pattern {
            let actual = grid.get(newpos.0, newpos.1).unwrap_or(&Xmas::X);
            if actual != expected {
                return false;
            }
            newpos = (newpos.0 + dir.0, newpos.1 + dir.1);
        }
        true
    })
    .count()
}

fn get_crossmasses(x: isize, y: isize, grid: &Grid) -> bool {
    for dir in [((-1, -1), (1, 1)), ((-1, 1), (1, -1))] {
        let chars = (
            grid.get(x + dir.0 .0, y + dir.0 .1).unwrap_or(&Xmas::X),
            grid.get(x + dir.1 .0, y + dir.1 .1).unwrap_or(&Xmas::X),
        );

        if chars != (&Xmas::M, &Xmas::S) && chars != (&Xmas::S, &Xmas::M) {
            return false;
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::from(input);

    Some(
        grid.letters
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, char)| {
                        if *char != Xmas::X {
                            return 0;
                        }

                        get_xmasses(x as isize, y as isize, &grid)
                    })
                    .sum::<usize>()
            })
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::from(input);

    Some(
        grid.letters
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(x, char)| {
                        if *char != &Xmas::A {
                            return false;
                        }

                        get_crossmasses(*x as isize, y as isize, &grid)
                    })
                    .count()
            })
            .sum::<usize>(),
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
