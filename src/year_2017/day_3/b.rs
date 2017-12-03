use std::fs::File;
use std::io::Read;

pub fn solve() -> String {
    let mut file = File::open("inputs/2017/3/input.txt").expect("inputs/2017/3/input.txt not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading inputs/2017/3/input.txt");
    let target: i32 = contents.parse().expect("Error parsing inputs/2017/3/input.txt");
    target.to_string()
}
