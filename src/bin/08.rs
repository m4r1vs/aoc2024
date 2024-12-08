use std::collections::{HashMap, HashSet};
use std::ops::Deref;

advent_of_code::solution!(8);

struct BoundingBox {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
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

    fn insert(&mut self, value: (usize, usize)) -> bool {
        let (x, y) = value;
        if self.is_within_bounds(x, y) {
            self.set.insert(value)
        } else {
            false
        }
    }

    fn is_within_bounds(&self, x: usize, y: usize) -> bool {
        x >= self.bounding_box.min_x
            && x <= self.bounding_box.max_x
            && y >= self.bounding_box.min_y
            && y <= self.bounding_box.max_y
    }
}

impl Deref for BoundedHashSet {
    type Target = HashSet<(usize, usize)>;

    fn deref(&self) -> &Self::Target {
        &self.set
    }
}

struct UniqueCombinations<'a, T: Clone> {
    set: &'a Vec<T>,     // The set to generate combinations from
    indices: Vec<usize>, // Internal tracker for combinations
    comb_length: usize,  // Length of each combination
    done: bool,          // Internal flag to mark completion
}

impl<'a, T: Clone> UniqueCombinations<'a, T> {
    fn new(set: &'a Vec<T>, comb_length: usize) -> Self {
        if comb_length == 0 || set.is_empty() || comb_length > set.len() {
            return Self {
                set,
                indices: Vec::new(),
                comb_length,
                done: true,
            };
        }

        Self {
            set,
            indices: (0..comb_length).collect(),
            comb_length,
            done: false,
        }
    }
}

impl<'a, T: Clone> Iterator for UniqueCombinations<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let current = self.indices.iter().map(|&i| self.set[i].clone()).collect();

        let mut i = self.comb_length;
        while i > 0 {
            i -= 1;

            if self.indices[i] < self.set.len() - (self.comb_length - i) {
                self.indices[i] += 1;
                for j in i + 1..self.comb_length {
                    self.indices[j] = self.indices[j - 1] + 1;
                }
                return Some(current);
            }
        }

        self.done = true;
        Some(current)
    }
}

fn get_antinodes(coords: Vec<(usize, usize)>) -> ((usize, usize), (usize, usize)) {
    let (x1, y1) = coords.first().unwrap();
    let (x2, y2) = coords.get(1).unwrap();

    ((2 * x2 - x1, 2 * y2 - y1), (2 * x1 - x2, 2 * y1 - y2))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (antennas, bounds) = parse_input(input);
    let mut antinodes = BoundedHashSet::new(bounds);

    for coords in antennas.values() {
        let combinations = UniqueCombinations::new(coords, 2);
        for combination in combinations {
            let (a1, a2) = get_antinodes(combination);
            antinodes.insert(a1);
            antinodes.insert(a2);
        }
    }

    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
            min_x: 0,
            min_y: 0,
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
