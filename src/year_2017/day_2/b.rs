use std::fs::File;
use std::io::Read;

fn div(line: &str) -> i32 {
    let numbers: Vec<i32> = line.split("\t").filter_map(|c| c.parse().ok()).collect();
    for number in &numbers {
        for divisor in &numbers {
            if number == divisor {
                continue;
            }
            if number % divisor == 0 {
                return number / divisor;
            }
        }
    }
    0
}

pub fn solve() -> String {
    let mut file = File::open("inputs/2017/2/spreadsheet.csv")
        .expect("inputs/2017/2/spreadsheet.csv not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading inputs/2017/2/spreadsheet.csv");
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut checksum = 0;
    for line in lines {
        checksum += div(line);
    }
    checksum.to_string()
}
