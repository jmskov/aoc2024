advent_of_code::solution!(5);
use std::collections::HashMap;

fn parse_input(input: &str) -> (String, String) {
    let idx = input.find("\n\n").unwrap();
    (input[0..idx].to_string(), input[idx+2..input.len()].to_string())
}

fn parse_rules(input: &str) -> HashMap<u32, Vec<u32>> {
    let mut vals: HashMap<u32, Vec<u32>> = HashMap::new();
    
    for rule in input.lines() {
        let r: Vec<u32> = rule.split("|").map(|x| x.parse::<u32>().unwrap()).collect();
        if vals.contains_key(&r[0]) {
            vals.get_mut(&r[0]).unwrap().push(r[1]);
        } else {
            vals.insert(r[0], vec![r[1]]);
        }
    }

    vals
}

fn fix_order(order: &str, rules: &HashMap<u32, Vec<u32>>) -> u32 {
    let mut v: Vec<u32> = order.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
    let mut mid = 0;
    let mut do_sort = true;
    let mut did_sort = false;
    while do_sort {
        do_sort = false;
        for i in (0..v.len()).rev() {
            if rules.contains_key(&v[i]) {
                // check remaining here
                for n in rules.get(&v[i]).unwrap() {
                    // now we can search for n!
                    for j in 0..i { 
                        if v[j] == *n {
                            do_sort = true;
                            // move v[j] to end of array
                            v[j..].rotate_left(1);
                            did_sort = true;
                        }
                    }
                }
            }
        }
    }

    if did_sort {
        mid = v[v.len()/2];
    }
    mid
}

fn check_order_rules(order: &str, rules: &HashMap<u32, Vec<u32>>) -> u32 {
    let v: Vec<u32> = order.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
    let mut mid = 0;
    let mut good = true;
    for i in (0..v.len()).rev() {
        if rules.contains_key(&v[i]) {
            // check remaining here
            for n in rules.get(&v[i]).unwrap() {
                // now we can search for n!
                for j in 0..i { 
                    if v[j] == *n {
                        good = false;
                    }
                }
                if !good {
                    break;
                }
            }
        }
        if !good {
            break;
        }
    }
    if good {
        mid = v[v.len()/2];
    }
    mid
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);
    let rule_map = parse_rules(&rules);
    let mut sum = 0;
    for update in updates.lines() {
        sum += check_order_rules(&update, &rule_map);
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);
    let rule_map = parse_rules(&rules);
    let mut sum = 0;
    for update in updates.lines() {
        sum += fix_order(&update, &rule_map);
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
