use std::fs::File;
use std::io::Read;

pub fn solve(input_dir: &str) -> String {
    let input = input!(input_dir, "input.txt");
    let mut floor = 0;
    for c in input.chars() {
        if c == '(' {
            floor += 1;
        }
        if c == ')' {
            floor -= 1;
        }
    }
    floor.to_string()
}
