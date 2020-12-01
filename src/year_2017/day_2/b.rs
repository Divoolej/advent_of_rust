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

pub fn solve(input_dir: &str) -> String {
    let input = input!(input_dir, "spreadsheet.csv");
    let lines: Vec<&str> = input.split("\n").collect();
    let mut checksum = 0;
    for line in lines {
        checksum += div(line);
    }
    checksum.to_string()
}
