advent_of_code::solution!(4);
use regex::Regex;

fn grid_step(idx: i32, ew: i32, ns: i32, nrows: i32, ncols: i32) -> Option<u32> {
    let mut result = None;
    
    // get row and col from idx
    // TODO: THIS LINE IS BAD
    let nrow = idx / ncols + ns;
    let ncol = idx % nrows + ew;
    
    if (nrow < 0 || nrow - 1 > nrows || ncol < 0 || ncol > ncols-1) {
        return result
    }

    let new_idx = idx + ns*ncols + ew;   // plus or minus
    let sz = nrows*ncols;
    if new_idx >= 0 && new_idx < sz {
        result = Some(new_idx as u32);
    }
    result
}

fn xmas_search(input: &str, cidx: i32, cchar: char, ew: i32, ns: i32, nrows: i32) -> bool {
    let mut result = false;
    // let nrows = input.lines().count() as i32;
    let ncols = (input.len() as i32) / nrows;
    if let Some(next_idx) = grid_step(cidx, ew, ns, nrows, ncols) {
        let mut nchar = 'M';
        if cchar == 'M' {
            nchar = 'A';
        } else if cchar == 'A' {
            nchar = 'S';
        }
        if input.chars().nth(next_idx as usize).unwrap() == nchar {
            if nchar == 'S' {
                result = true;
            } else {
                result = xmas_search(input, next_idx as i32, nchar, ew, ns, nrows);
            }
        }
    }
    result
}

fn is_c(input: &str, idx: u32, c: char) -> u32 {
    let result = input.chars().nth(idx as usize).unwrap() == c;
    result as u32
}

fn xxmas_search(input: &str, cidx: i32, nrows: i32) -> bool {
    let mut result = false;
    let ncols = (input.len() as i32) / nrows;
    let mut cs: Vec<char> = vec![];

    if let Some(next_idx) = grid_step(cidx, 1, 1, nrows, ncols) {
        cs.push(input.chars().nth(next_idx as usize).unwrap());
        
    } else {
        return result;
    }
    if let Some(next_idx) = grid_step(cidx, -1, 1, nrows, ncols) {
        cs.push(input.chars().nth(next_idx as usize).unwrap());
    } else {
        return result;
    }
    
    if let Some(next_idx) = grid_step(cidx, -1, -1, nrows, ncols) {
        cs.push(input.chars().nth(next_idx as usize).unwrap());
    } else {
        return result;
    }

    if let Some(next_idx) = grid_step(cidx, 1, -1, nrows, ncols) {
        cs.push(input.chars().nth(next_idx as usize).unwrap());
    } else {
        return result;
    }    

    let ss: String = cs.into_iter().collect();
    
    if ss == "MMSS" || ss == "MSSM" || ss == "SSMM" || ss == "SMMS" {
        result = true;
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {

    // Create 2D Array
    // For Each Potential Start, Find # of Valid XMAS
    // Regex on each slice?
    
    let pinput= input.chars().filter(|c| *c != '\n').collect::<String>();
    let nrows = (input.lines().count()) as i32;

    let mut count = 0;
    
    for i in 0..pinput.len() {
        let c = pinput.chars().nth(i).unwrap();
        if c == 'X' {
            // 8 directions
            count += xmas_search(&pinput, i as i32, c, 1, 0, nrows) as u32;
            count += xmas_search(&pinput, i as i32, c, -1, 0, nrows) as u32;
            count += xmas_search(&pinput, i as i32, c, 0, 1, nrows) as u32;
            count += xmas_search(&pinput, i as i32, c, 0, -1, nrows) as u32;
            count += xmas_search(&pinput, i as i32, c, 1, 1, nrows) as u32;
            count += xmas_search(&pinput, i as i32, c, 1, -1, nrows) as u32;
            count += xmas_search(&pinput, i as i32, c, -1, 1, nrows) as u32;
            count += xmas_search(&pinput, i as i32, c, -1, -1, nrows) as u32;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let pinput= input.chars().filter(|c| *c != '\n').collect::<String>();
    let nrows = (input.lines().count()) as i32;

    let mut count = 0;
    
    for i in 0..pinput.len() {
        let c = pinput.chars().nth(i).unwrap();
        if c == 'A' {
            // 8 directions
            count += xxmas_search(&pinput, i as i32, nrows) as u32;
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
