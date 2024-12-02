pub mod template;

// Use this file to add helper functions and additional modules.
pub fn split_to_uvec(input: &str) -> Vec<u32> {
    let vec_all: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    vec_all
}