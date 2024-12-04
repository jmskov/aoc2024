advent_of_code::solution!(3);
use regex::Regex;

fn extract_mul(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for (_, [num1, num2]) in re.captures_iter(input).map(|c| c.extract()) {
        sum += num1.parse::<u32>().unwrap() * num2.parse::<u32>().unwrap();
    }
    sum
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(extract_mul(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    
    // Idea for this one:
    let mut sum = 0;
    let mut start_idx = Some(0);

    let slen = input.len() as usize;

    while start_idx != None {
        let sidx = start_idx.unwrap() as usize;
        // 1. Find the next don't() ...
        let stop_idx = input[sidx..slen].find("don't()").map(|idx| idx + sidx);
        // Process substring up until then
        if let Some(eidx) = stop_idx {
            sum += extract_mul(&input[sidx..eidx]);
            // Find the next do()...
            start_idx = input[eidx..slen].find("do()").map(|idx| idx + eidx);
        } else {
            // Otherwise, no more don't!
            sum += extract_mul(&input[sidx..slen]);
            start_idx = None;
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = part_two(&input);
        assert_eq!(result, Some(48));
    }
}
