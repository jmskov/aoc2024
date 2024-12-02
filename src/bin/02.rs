advent_of_code::solution!(2);

use advent_of_code::*;

fn is_level_safe(level: &Vec<u32>) -> bool {
    level.windows(2).all(|x| x[0] < x[1] && x[0].abs_diff(x[1]) <= 3) ||
    level.windows(2).all(|x| x[0] > x[1] && x[0].abs_diff(x[1]) <= 3)
}

pub fn part_one(input: &str) -> Option<u32> {
    // get line
    let mut safe_levels = 0;
    for line in input.lines() {
        let levels = split_to_uvec(line); 
        if is_level_safe(&levels) {
            safe_levels += 1;
        }
    }
    Some(safe_levels)
}

fn is_level_safe_skip_i(level: &Vec<u32>, idx: usize) -> bool {
    let mut vec_c = level.clone();
    vec_c.remove(idx);
    is_level_safe(&vec_c)
}

pub fn part_two(input: &str) -> Option<u32> {
    // get line
    let mut safe_levels = 0;
    for line in input.lines() {
        let levels = split_to_uvec(line);
        
        if !is_level_safe(&levels) {
            for i in 0..levels.len() {
                if is_level_safe_skip_i(&levels, i) {
                    safe_levels += 1;
                    break;
                }
            }
        } else {
            safe_levels += 1;
        }
    }
    Some(safe_levels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
