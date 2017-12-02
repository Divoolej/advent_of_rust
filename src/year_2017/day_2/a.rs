use std::fs::File;
use std::io::Read;

fn diff(line: &str) -> i32 {
    let numbers: Vec<i32> = line.split("\t").filter_map( |c| c.parse().ok() ).collect();
    let mut min = i32::max_value();
    let mut max = i32::min_value();
    for n in numbers {
        min = if n < min { n } else { min };
        max = if n > max { n } else { max };
    }
    max - min
}

pub fn solve() -> String {
    let mut file = File::open("inputs/2017/2/spreadsheet.csv").expect("inputs/2017/2/spreadsheet.csv not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading inputs/2017/2/spreadsheet.csv");
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut checksum = 0;
    for line in lines {
        checksum += diff(line);
    }
    checksum.to_string()
}
