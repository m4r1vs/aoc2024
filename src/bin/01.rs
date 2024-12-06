use std::collections::HashMap;

use rbtree::RBTree;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut tree1: RBTree<u32, bool> = RBTree::new();
    let mut tree2: RBTree<u32, bool> = RBTree::new();

    for (i, number) in input
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .enumerate()
    {
        if i % 2 == 0 {
            tree1.insert(number, false);
        } else {
            tree2.insert(number, false);
        }
    }

    assert_eq!(tree1.len(), tree2.len());

    let mut total_distance: u32 = 0;

    let mut tree1_iter = tree1.iter();
    let mut tree2_iter = tree2.iter();
    let mut smallest_unhandled_node_t1 = tree1_iter.next();

    while smallest_unhandled_node_t1.is_some() {
        let (value_t1, _) = smallest_unhandled_node_t1.unwrap(); // guaranteed by while
        let (value_t2, _) = tree2_iter.next().unwrap(); // guaranteed by equal length

        if value_t1 > value_t2 {
            total_distance += value_t1 - value_t2;
        } else {
            total_distance += value_t2 - value_t1;
        }

        smallest_unhandled_node_t1 = tree1_iter.next();
    }

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: HashMap<u32, u32> = HashMap::new();
    let mut array: Vec<u32> = Vec::new();

    for (i, number) in input
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .enumerate()
    {
        if i % 2 == 0 {
            array.push(number);
        } else {
            map.entry(number).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    let mut score = 0;
    for number in array.iter() {
        let occurances = map.get(&number);
        if occurances.is_some() {
            score += number * occurances.unwrap();
        }
    }

    Some(score)
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
