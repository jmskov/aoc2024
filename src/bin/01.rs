advent_of_code::solution!(1);
use std::collections::HashMap;

fn parse_vecs(input: &str) -> (Vec<u32>, Vec<u32>) {
    // 1. Split the input string into two arrays
    let vec_all: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    // get the separate arrays, prob inefficient
    let vec1: Vec<u32> = vec_all.iter().step_by(2).copied().collect();

    let vec2: Vec<u32> = vec_all.iter().skip(1).step_by(2).copied().collect();

    (vec1, vec2)
}

pub fn part_one(input: &str) -> Option<u32> {
    // 1. Parse the input
    let (mut vec1, mut vec2) = parse_vecs(input);
    // 2. Sort each array in ascending order
    vec1.sort();
    vec2.sort();

    // 3. Take the difference of the array and sum
    let mut res = 0;
    for i in 0..vec1.len() {
        res += vec1[i].abs_diff(vec2[i]);
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    // 1. Parse the input
    let (vec1, vec2) = parse_vecs(input);
    // 2. Hash the right list
    let mut vals = HashMap::new();

    for i in 0..vec2.len() {
        vals.entry(vec2[i]).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut res = 0;
    for i in 0..vec1.len() {
        // try to get the entry
        if let Some(x) = vals.get(&vec1[i]) {
            res += vec1[i] * x;
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
