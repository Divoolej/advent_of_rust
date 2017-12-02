use std::fs::File;
use std::io::Read;

pub fn solve() -> String {
    let mut file = File::open("inputs/2017/1/captcha.txt").expect("inputs/2017/1/captcha.txt not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading inputs/2017/1/captcha.txt");
    let numbers: Vec<i32> = contents.split("").filter_map( |c| c.parse().ok() ).collect();
    let mut sum = 0;
    for i in 0..(numbers.len() - 1) {
       if numbers[i] == numbers[i + 1] {
           sum += numbers[i+1];
       }
    }
    if numbers[0] == numbers[numbers.len() - 1] {
        sum += numbers[numbers.len() - 1];
    }
    sum.to_string()
}
