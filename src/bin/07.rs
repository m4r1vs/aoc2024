use std::collections::VecDeque;

advent_of_code::solution!(7);

#[derive(Clone, Debug)]
struct Operation {
    f: fn(u64, u64) -> u64,
    // priority: bool // For order of operation (NOT NEEDED)
}

fn add(first: u64, second: u64) -> u64 {
    first + second
}

fn multiply(first: u64, second: u64) -> u64 {
    first * second
}

fn concatinate(first: u64, second: u64) -> u64 {
    let mut multiplier = 1;
    let mut temp = second;

    while temp > 0 {
        multiplier *= 10;
        temp /= 10;
    }

    first * multiplier + second
}

struct CartesianProduct<'a, T: Clone> {
    set: &'a VecDeque<T>,      // The set to be permutated
    indices: Vec<usize>,       // Internal tracker for combinations
    total_combinations: usize, // Desired combinations
    done: bool,                // Internal flag to mark completion
}

impl<'a, T: Clone> CartesianProduct<'a, T> {
    fn new(set: &'a VecDeque<T>, n: usize) -> Self {
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
    type Item = VecDeque<T>;

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

fn true_of_desired_calculatable(
    desired: u64,
    nums: VecDeque<u64>,
    operations: &VecDeque<Operation>,
) -> bool {
    let combinations = CartesianProduct::new(&operations, nums.len() - 1);

    for combination in combinations {
        let mut tmp_result = nums.clone();

        let comb_iterator = combination.iter();

        for o in comb_iterator {
            let (x, y) = (
                tmp_result.pop_front().unwrap(),
                tmp_result.pop_front().unwrap(),
            );

            tmp_result.push_front((o.f)(x, y));
        }

        // Always calced from left to right, so this stuff is NOT needed lol:
        // while {
        //     match comb_iterator.next() {
        //         Some(x) => {
        //             o = x;
        //             true
        //         }
        //         None => false,
        //     }
        // } {
        //     let (x, y) = (
        //         tmp_result.pop_front().unwrap(),
        //         tmp_result.pop_front().unwrap(),
        //     );

        //     tmp_result.push_front((o.f)(x, y));

        //
        // let current_is_last = i + 1 == combination.len();
        // let next_has_priority = !current_is_last && combination[i + 1].priority;
        // println!(
        //     "{} {} {} = {}",
        //     x,
        //     if o.priority { "*" } else { "+" },
        //     y,
        //     (o.f)(x, y)
        // );

        // if o.priority || current_is_last || !next_has_priority {
        //     let (x, y) = (
        //         tmp_result.pop_front().unwrap(),
        //         tmp_result.pop_front().unwrap(),
        //     );

        //     print!(
        //         "{} {} {} = {}",
        //         x,
        //         if o.priority { "*" } else { "+" },
        //         y,
        //         (o.f)(x, y)
        //     );

        //     tmp_result.push_front((o.f)(x, y));
        // } else {
        //     let first = tmp_result.pop_front().unwrap();
        //     let (_, o2) = comb_iterator.next().unwrap();
        //     let (x, y) = (
        //         tmp_result.pop_front().unwrap(),
        //         tmp_result.pop_front().unwrap(),
        //     );

        //     print!(
        //         "{} {} ({} {} {}) = {}",
        //         first,
        //         if o.priority { "*" } else { "+" },
        //         x,
        //         if o2.priority { "*" } else { "+" },
        //         y,
        //         (o.f)(first, (o2.f)(x, y))
        //     );

        //     tmp_result.push_front((o.f)(first, (o2.f)(x, y)));
        // }

        // println!();
        // }

        // assert_eq!(tmp_result.len(), 1);

        if tmp_result.get(0).unwrap() == &desired {
            return true;
        };
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let number_map = parse_input(input);

    let operations = vec![Operation { f: add }, Operation { f: multiply }];

    for (desired, nums) in number_map {
        if true_of_desired_calculatable(desired, nums, &operations) {
            sum += desired;
        };
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let number_map = parse_input(input);

    let operations = vec![
        Operation { f: add },
        Operation { f: multiply },
        Operation { f: concatinate },
    ];

    for (desired, nums) in number_map {
        if true_of_desired_calculatable(desired, nums, &operations) {
            sum += desired;
        };
    }

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
