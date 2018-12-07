use std::fs::File;
use std::io::Read;

pub fn solve() -> String {
    let mut file =
        File::open("inputs/2015/1/input.txt").expect("inputs/2015/1/input.txt not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading inputs/2015/1/input.txt");
    let mut floor = 0;
    for (i, c) in contents.chars().enumerate() {
        if c == '(' {
            floor += 1;
        }
        if c == ')' {
            floor -= 1;
        }
        if floor == -1 {
            return (i + 1).to_string();
        }
    }
    '0'.to_string()
}
