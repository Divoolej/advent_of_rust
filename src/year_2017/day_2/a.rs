use std::fs::File;
use std::io::Read;

fn diff(line: &str) -> i32 {
    let numbers: Vec<i32> = line.split("\t").filter_map(|c| c.parse().ok()).collect();
    let mut min = i32::max_value();
    let mut max = i32::min_value();
    for n in numbers {
        min = if n < min { n } else { min };
        max = if n > max { n } else { max };
    }
    max - min
}

pub fn solve(input_dir: &str) -> String {
    let input = input!(input_dir, "spreadsheet.csv");
    let lines: Vec<&str> = input.split("\n").collect();
    let mut checksum = 0;
    for line in lines {
        checksum += diff(line);
    }
    checksum.to_string()
}
