use std::fs::File;
use std::io::Read;

pub fn solve(input_dir: &str) -> String {
    let input = input!(input_dir, "captcha.txt");
    let numbers: Vec<i32> = input.split("").filter_map(|c| c.parse().ok()).collect();
    let mut sum = 0;
    let n = numbers.len();
    for (i, number) in numbers.iter().enumerate() {
        if *number == numbers[(i + n / 2) % n] {
            sum += number;
        }
    }
    sum.to_string()
}
