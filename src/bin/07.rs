use std::{collections::VecDeque, thread};

advent_of_code::solution!(7);

fn add(first: u64, second: u64) -> u64 {
    first + second
}

fn multiply(first: u64, second: u64) -> u64 {
    first * second
}

fn concatinate(first: u64, second: u64) -> u64 {
    let num_digits = second.ilog10() + 1;
    first * 10u64.pow(num_digits) + second
}

struct CartesianProduct<'a, T: Clone> {
    set: &'a Vec<T>,           // The set to be permutated
    indices: Vec<usize>,       // Internal tracker for combinations
    total_combinations: usize, // Desired combinations
    done: bool,                // Internal flag to mark completion
}

impl<'a, T: Clone> CartesianProduct<'a, T> {
    fn new(set: &'a Vec<T>, n: usize) -> Self {
        if n == 0 || set.is_empty() {
            return Self {
                set,
                indices: Vec::new(),
                total_combinations: n,
                done: true,
            };
        }

        Self {
            set,
            indices: vec![0; n],
            total_combinations: n,
            done: false,
        }
    }
}

impl<'a, T: Clone> Iterator for CartesianProduct<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let current = self.indices.iter().map(|&i| self.set[i].clone()).collect();

        for i in (0..self.total_combinations).rev() {
            if self.indices[i] + 1 < self.set.len() {
                self.indices[i] += 1;
                for j in (i + 1)..self.total_combinations {
                    self.indices[j] = 0;
                }
                return Some(current);
            }
        }

        self.done = true;
        Some(current)
    }
}

fn is_solvable(desired: u64, nums: &VecDeque<u64>, operations: &Vec<fn(u64, u64) -> u64>) -> bool {
    let combinations = CartesianProduct::new(operations, nums.len() - 1);

    for combination in combinations {
        let mut tmp_result = nums.clone();

        for op in combination.iter() {
            let (x, y) = (
                tmp_result.pop_front().unwrap(),
                tmp_result.pop_front().unwrap(),
            );

            tmp_result.push_front((*op)(x, y));
        }

        if tmp_result.front().unwrap() == &desired {
            return true;
        };
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let number_map = parse_input(input);

    let operations = vec![add, multiply];

    let thread_count = 12;
    let chunk_size = match number_map.len() % thread_count {
        0 => number_map.len() / thread_count,
        left_over => (number_map.len() + thread_count - left_over) / thread_count,
    };

    let handles: Vec<_> = number_map
        .chunks(chunk_size)
        .map(|chunk| {
            let chunk = chunk.to_vec();

            thread::spawn({
                let ops = operations.clone();
                move || {
                    chunk
                        .iter()
                        .filter(|(desired, nums)| is_solvable(*desired, nums, &ops))
                        .map(|(desired, _)| desired)
                        .sum::<u64>()
                }
            })
        })
        .collect();

    let sum: u64 = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let number_map = parse_input(input);

    let operations = vec![add, multiply, concatinate];

    let thread_count = 12;
    let chunk_size = match number_map.len() % thread_count {
        0 => number_map.len() / thread_count,
        left_over => (number_map.len() + thread_count - left_over) / thread_count,
    };

    let handles: Vec<_> = number_map
        .chunks(chunk_size)
        .map(|chunk| {
            let chunk = chunk.to_vec();

            thread::spawn({
                let ops = operations.clone();
                move || {
                    chunk
                        .iter()
                        .filter(|(desired, nums)| is_solvable(*desired, nums, &ops))
                        .map(|(desired, _)| desired)
                        .sum::<u64>()
                }
            })
        })
        .collect();

    let sum: u64 = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum();

    Some(sum)
}

fn parse_input(input: &str) -> Vec<(u64, VecDeque<u64>)> {
    let mut number_map = Vec::new();

    for line in input.lines() {
        if let Some((p1, p2)) = line.split_once(":") {
            if let Ok(pn1) = p1.parse::<u64>() {
                let number_list = p2
                    .split_whitespace()
                    .filter_map(|s| s.parse::<u64>().ok())
                    .collect::<VecDeque<u64>>();
                number_map.push((pn1, number_list));
            }
        }
    }

    number_map
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
