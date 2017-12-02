use std::fs::File;
use std::io::Read;

pub fn solve() -> String {
    let mut file = File::open("inputs/2015/1/input.txt").expect("inputs/2015/1/input.txt not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading inputs/2015/1/input.txt");
    let mut floor = 0;
    for c in contents.chars() {
      if c == '(' { floor += 1; }
      if c == ')' { floor -= 1; }
    }
    floor.to_string()
}
