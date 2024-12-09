use std::collections::{HashMap, HashSet};
use std::ops::Deref;

advent_of_code::solution!(8);

struct BoundingBox {
    max_x: usize,
    max_y: usize,
}

impl BoundingBox {
    fn is_within_bounds(&self, coords: (usize, usize)) -> bool {
        coords.0 <= self.max_x && coords.1 <= self.max_y
    }
}

// HashSet but insertions outside the bounding box are discarded
struct BoundedHashSet {
    set: HashSet<(usize, usize)>,
    bounding_box: BoundingBox,
}

impl BoundedHashSet {
    fn new(bounding_box: BoundingBox) -> Self {
        Self {
            set: HashSet::new(),
            bounding_box,
        }
    }

    // Return false only if out of bounds
    fn insert(&mut self, value: (usize, usize)) -> bool {
        let (x, y) = value;
        if self.bounding_box.is_within_bounds((x, y)) {
            self.set.insert(value);
            true
        } else {
            false
        }
    }
}

// Use HashSet implementations for all other functions
impl Deref for BoundedHashSet {
    type Target = HashSet<(usize, usize)>;

    fn deref(&self) -> &Self::Target {
        &self.set
    }
}

struct UniqueCombinations<'a, T: Clone> {
    set: &'a Vec<T>,     // The set to generate combinations from
    indices: Vec<usize>, // Internal tracker for combinations
    done: bool,          // Internal flag to mark completion
}

impl<'a, T: Clone> UniqueCombinations<'a, T> {
    fn new(set: &'a Vec<T>) -> Self {
        if set.is_empty() || 2 > set.len() {
            return Self {
                set,
                indices: Vec::new(),
                done: true,
            };
        }

        Self {
            set,
            indices: (0..2).collect(),
            done: false,
        }
    }
}

impl<T: Clone> Iterator for UniqueCombinations<'_, T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let current = (
            self.set[self.indices[0]].clone(),
            self.set[self.indices[1]].clone(),
        );

        let mut i = 2;
        while i > 0 {
            i -= 1;

            if self.indices[i] < self.set.len() - (2 - i) {
                self.indices[i] += 1;
                for j in i + 1..2 {
                    self.indices[j] = self.indices[j - 1] + 1;
                }
                return Some(current);
            }
        }

        self.done = true;
        Some(current)
    }
}

fn get_antinode(coords: ((usize, usize), (usize, usize))) -> (usize, usize) {
    let (x1, y1) = coords.0;
    let (x2, y2) = coords.1;

    (2 * x2 - x1, 2 * y2 - y1)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (antennas, bounds) = parse_input(input);
    let mut antinodes = BoundedHashSet::new(bounds);

    for coords in antennas.values() {
        let combinations = UniqueCombinations::new(coords);
        for combination in combinations {
            antinodes.insert(get_antinode((combination.0, combination.1)));
            antinodes.insert(get_antinode((combination.1, combination.0)));
        }
    }

    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (antennas, bounds) = parse_input(input);
    let mut antinodes = BoundedHashSet::new(bounds);

    for coords in antennas.values() {
        let combinations = UniqueCombinations::new(coords);
        for combination in combinations {
            // Every antenna with a match is an antinode now
            antinodes.insert(combination.0);
            antinodes.insert(combination.1);

            let mut a0 = get_antinode((combination.0, combination.1));
            let mut prev = combination.1;
            while antinodes.insert(a0) {
                (prev, a0) = (a0, get_antinode((prev, a0)));
            }

            let mut a1 = get_antinode((combination.1, combination.0));
            let mut prev = combination.0;
            while antinodes.insert(a1) {
                (prev, a1) = (a1, get_antinode((prev, a1)));
            }
        }
    }

    Some(antinodes.len())
}

fn parse_input(input: &str) -> (HashMap<char, Vec<(usize, usize)>>, BoundingBox) {
    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    let mut y = 0;

    input.lines().for_each(|line| {
        line.char_indices()
            .filter(|(_, c)| *c != '.')
            .for_each(|(x, char)| match map.get_mut(&char) {
                Some(vec) => vec.push((x, y)),
                None => {
                    map.insert(char, vec![(x, y)]);
                }
            });
        y += 1;
    });

    (
        map,
        BoundingBox {
            max_x: input.find("\n").unwrap() - 1,
            max_y: y - 1,
        },
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
