use std::fs::File;
use std::io::Read;

pub fn solve() -> String {
    let mut file =
        File::open("inputs/2017/1/captcha.txt").expect("inputs/2017/1/captcha.txt not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading inputs/2017/1/captcha.txt");
    let numbers: Vec<i32> = contents.split("").filter_map(|c| c.parse().ok()).collect();
    let mut sum = 0;
    let n = numbers.len();
    for (i, number) in numbers.iter().enumerate() {
        if *number == numbers[(i + n / 2) % n] {
            sum += number;
        }
    }
    sum.to_string()
}
